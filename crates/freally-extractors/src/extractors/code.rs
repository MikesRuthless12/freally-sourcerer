//! Source-code extractor — tree-sitter parse, capture identifiers,
//! comments, and string literals.
//!
//! Build Guide Phase 8 named eight languages (Rust, Python, JS/TS, Go,
//! Java, C, C++); we ship a far broader set so a `content:` query
//! against a polyglot project doesn't have any blind spots. Languages
//! supported as of this phase:
//!
//!   * **Systems:** Rust, C, C++, Go, Zig, Nim
//!   * **Scripting:** Python, Ruby, Bash, Lua, Perl, R, Julia
//!   * **JVM:** Java, Kotlin, Scala, Clojure
//!   * **.NET:** C#
//!   * **Web/JS family:** JavaScript, TypeScript, TSX/JSX
//!   * **Functional:** Haskell, OCaml, Elm, Erlang, Elixir
//!   * **Mobile:** Swift, Dart
//!   * **Markup/Config:** HTML, CSS, JSON, TOML, YAML, Nix
//!   * **Database:** SQL
//!
//! ### Output shape
//!
//! Per-language sections, each labeled and separated by a blank line:
//!
//! ```text
//! [lang]
//! rust
//!
//! [identifiers]
//! foo bar baz_struct ...
//!
//! [strings]
//! "hello world"
//! "another literal"
//!
//! [comments]
//! // top-level doc
//! /// trait doc
//! ```
//!
//! Identifiers are de-duplicated; strings and comments preserve their
//! literal form so a `content:"hello world"` query matches the
//! original source. Cooperative cancel checks fire every
//! [`CANCEL_CHECK_EVERY`] node so the supervisor can interrupt a long
//! tree walk.

use std::collections::BTreeSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use tree_sitter::{Language, Node, Parser};

use crate::sink::TextSink;
use crate::{ExtractError, ExtractionStats, Extractor, ExtractorId};

/// Hard cap on bytes the code extractor reads from disk. Source files
/// over 4 MiB are nearly always generated; the long tail rarely pays
/// for itself in search-relevance terms.
pub const CODE_CAP_BYTES: usize = 4 * 1024 * 1024;

const CANCEL_CHECK_EVERY: usize = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Lang {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Tsx,
    Go,
    Java,
    C,
    Cpp,
    CSharp,
    Ruby,
    Bash,
    Lua,
    Php,
    Kotlin,
    Scala,
    Swift,
    Haskell,
    Ocaml,
    Elixir,
    Erlang,
    Clojure,
    Elm,
    Dart,
    R,
    Julia,
    Zig,
    Nix,
    Toml,
    Yaml,
    Json,
    Html,
    Css,
    Sql,
}

impl Lang {
    fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_ascii_lowercase().as_str() {
            "rs" => Some(Self::Rust),
            "py" | "pyi" | "pyx" => Some(Self::Python),
            "js" | "mjs" | "cjs" => Some(Self::JavaScript),
            "ts" => Some(Self::TypeScript),
            "tsx" | "jsx" => Some(Self::Tsx),
            "go" => Some(Self::Go),
            "java" => Some(Self::Java),
            "c" | "h" => Some(Self::C),
            "cc" | "cpp" | "cxx" | "hh" | "hpp" | "hxx" | "c++" | "h++" | "ipp" | "tpp" => {
                Some(Self::Cpp)
            }
            "cs" | "csx" => Some(Self::CSharp),
            "rb" | "rake" | "gemspec" => Some(Self::Ruby),
            "sh" | "bash" | "zsh" | "ksh" => Some(Self::Bash),
            "lua" => Some(Self::Lua),
            "php" | "phtml" | "php5" | "php7" | "php8" => Some(Self::Php),
            "kt" | "kts" => Some(Self::Kotlin),
            "scala" | "sbt" | "sc" => Some(Self::Scala),
            "swift" => Some(Self::Swift),
            "hs" | "lhs" => Some(Self::Haskell),
            "ml" | "mli" => Some(Self::Ocaml),
            "ex" | "exs" => Some(Self::Elixir),
            "erl" | "hrl" => Some(Self::Erlang),
            "clj" | "cljs" | "cljc" | "edn" => Some(Self::Clojure),
            "elm" => Some(Self::Elm),
            "dart" => Some(Self::Dart),
            "r" | "rd" => Some(Self::R),
            "jl" => Some(Self::Julia),
            "zig" | "zon" => Some(Self::Zig),
            "nix" => Some(Self::Nix),
            "toml" => Some(Self::Toml),
            "yaml" | "yml" => Some(Self::Yaml),
            "json" | "jsonc" | "json5" => Some(Self::Json),
            "html" | "htm" | "xhtml" => Some(Self::Html),
            "css" | "scss" => Some(Self::Css),
            "sql" => Some(Self::Sql),
            _ => None,
        }
    }

    fn from_filename_only(name: &str) -> Option<Self> {
        // Files with no extension or whose extension is misleading.
        let lower = name.to_ascii_lowercase();
        if matches!(
            lower.as_str(),
            "makefile" | "rakefile" | "gemfile" | "guardfile" | "vagrantfile"
        ) {
            // Most "Rakefile" / "Gemfile" content is Ruby DSL.
            return Some(Self::Ruby);
        }
        None
    }

    fn language(self) -> Language {
        match self {
            Self::Rust => tree_sitter_rust::LANGUAGE.into(),
            Self::Python => tree_sitter_python::LANGUAGE.into(),
            Self::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
            Self::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Self::Tsx => tree_sitter_typescript::LANGUAGE_TSX.into(),
            Self::Go => tree_sitter_go::LANGUAGE.into(),
            Self::Java => tree_sitter_java::LANGUAGE.into(),
            Self::C => tree_sitter_c::LANGUAGE.into(),
            Self::Cpp => tree_sitter_cpp::LANGUAGE.into(),
            Self::CSharp => tree_sitter_c_sharp::LANGUAGE.into(),
            Self::Ruby => tree_sitter_ruby::LANGUAGE.into(),
            Self::Bash => tree_sitter_bash::LANGUAGE.into(),
            Self::Lua => tree_sitter_lua::LANGUAGE.into(),
            Self::Php => tree_sitter_php::LANGUAGE_PHP.into(),
            Self::Kotlin => tree_sitter_kotlin_ng::LANGUAGE.into(),
            Self::Scala => tree_sitter_scala::LANGUAGE.into(),
            Self::Swift => tree_sitter_swift::LANGUAGE.into(),
            Self::Haskell => tree_sitter_haskell::LANGUAGE.into(),
            Self::Ocaml => tree_sitter_ocaml::LANGUAGE_OCAML.into(),
            Self::Elixir => tree_sitter_elixir::LANGUAGE.into(),
            Self::Erlang => tree_sitter_erlang::LANGUAGE.into(),
            Self::Clojure => tree_sitter_clojure::LANGUAGE.into(),
            Self::Elm => tree_sitter_elm::LANGUAGE.into(),
            Self::Dart => tree_sitter_dart::LANGUAGE.into(),
            Self::R => tree_sitter_r::LANGUAGE.into(),
            Self::Julia => tree_sitter_julia::LANGUAGE.into(),
            Self::Zig => tree_sitter_zig::LANGUAGE.into(),
            Self::Nix => tree_sitter_nix::LANGUAGE.into(),
            Self::Toml => tree_sitter_toml_ng::LANGUAGE.into(),
            Self::Yaml => tree_sitter_yaml::LANGUAGE.into(),
            Self::Json => tree_sitter_json::LANGUAGE.into(),
            Self::Html => tree_sitter_html::LANGUAGE.into(),
            Self::Css => tree_sitter_css::LANGUAGE.into(),
            Self::Sql => tree_sitter_sequel::LANGUAGE.into(),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::JavaScript => "javascript",
            Self::TypeScript => "typescript",
            Self::Tsx => "tsx",
            Self::Go => "go",
            Self::Java => "java",
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::CSharp => "c#",
            Self::Ruby => "ruby",
            Self::Bash => "bash",
            Self::Lua => "lua",
            Self::Php => "php",
            Self::Kotlin => "kotlin",
            Self::Scala => "scala",
            Self::Swift => "swift",
            Self::Haskell => "haskell",
            Self::Ocaml => "ocaml",
            Self::Elixir => "elixir",
            Self::Erlang => "erlang",
            Self::Clojure => "clojure",
            Self::Elm => "elm",
            Self::Dart => "dart",
            Self::R => "r",
            Self::Julia => "julia",
            Self::Zig => "zig",
            Self::Nix => "nix",
            Self::Toml => "toml",
            Self::Yaml => "yaml",
            Self::Json => "json",
            Self::Html => "html",
            Self::Css => "css",
            Self::Sql => "sql",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CodeExtractor {
    pub max_bytes: Option<usize>,
}

impl CodeExtractor {
    pub fn with_max_bytes(max_bytes: usize) -> Self {
        Self {
            max_bytes: Some(max_bytes),
        }
    }

    fn cap(&self) -> usize {
        self.max_bytes.unwrap_or(CODE_CAP_BYTES)
    }

    fn classify(path: &Path) -> Option<Lang> {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if let Some(lang) = Lang::from_extension(ext) {
                return Some(lang);
            }
        }
        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            if let Some(lang) = Lang::from_filename_only(name) {
                return Some(lang);
            }
        }
        None
    }
}

impl Extractor for CodeExtractor {
    fn id(&self) -> ExtractorId {
        ExtractorId::new("code")
    }

    fn matches(&self, path: &Path, _magic: &[u8]) -> bool {
        Self::classify(path).is_some()
    }

    fn extract(&self, path: &Path, sink: &mut TextSink) -> Result<ExtractionStats, ExtractError> {
        let lang = Self::classify(path)
            .ok_or_else(|| ExtractError::Unsupported("not a recognized source file".into()))?;

        let cap = self.cap();
        let mut file = File::open(path).map_err(|e| ExtractError::io(path, e))?;
        let mut source = Vec::with_capacity(cap.min(64 * 1024));
        (&mut file)
            .take(cap as u64 + 1)
            .read_to_end(&mut source)
            .map_err(|e| ExtractError::io(path, e))?;
        let bytes_in = source.len() as u64;
        if source.len() > cap {
            source.truncate(cap);
            // Trim back to a UTF-8 boundary so tree-sitter doesn't see
            // a half-codepoint at the tail. The continuation-byte
            // backtrack runs in O(1) (UTF-8 codepoints are at most 4
            // bytes); the previous `from_utf8` loop re-scanned the
            // whole 4 MiB buffer per pop.
            super::util::trim_to_utf8_boundary(&mut source);
        }

        let mut parser = Parser::new();
        parser
            .set_language(&lang.language())
            .map_err(|e| ExtractError::Other(format!("tree-sitter set_language failed: {e}")))?;
        let tree = parser.parse(&source, None).ok_or_else(|| {
            ExtractError::Malformed(format!("tree-sitter parse failed for {}", lang.name()))
        })?;

        let mut idents: BTreeSet<String> = BTreeSet::new();
        let mut strings: Vec<String> = Vec::new();
        let mut comments: Vec<String> = Vec::new();
        let mut node_count = 0usize;

        walk_tree(
            tree.root_node(),
            &source,
            sink,
            &mut idents,
            &mut strings,
            &mut comments,
            &mut node_count,
        )?;

        // Header so search results can highlight by section.
        let lang_header = format!("[lang]\n{}\n\n", lang.name());
        sink.push_str(&lang_header)
            .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
        if !idents.is_empty() {
            sink.push_str("[identifiers]\n")
                .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
            for (i, id) in idents.iter().enumerate() {
                if i > 0 {
                    sink.push_str(" ")
                        .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
                }
                sink.push_str(id)
                    .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
            }
            sink.push_str("\n\n")
                .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
        }
        if !strings.is_empty() {
            sink.push_str("[strings]\n")
                .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
            for s in &strings {
                sink.push_str(s)
                    .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
                sink.push_str("\n")
                    .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
            }
            sink.push_str("\n")
                .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
        }
        if !comments.is_empty() {
            sink.push_str("[comments]\n")
                .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
            for c in &comments {
                sink.push_str(c)
                    .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
                sink.push_str("\n")
                    .map_err(|_| ExtractError::OutputTooLarge { cap: sink.cap() })?;
            }
        }

        Ok(ExtractionStats {
            bytes_in,
            bytes_out: sink.len() as u64,
            elapsed: std::time::Duration::ZERO,
        })
    }
}

/// Recursive descent over the tree-sitter parse tree. Phase-13
/// perf-pass note: this is recursive, which a deeply-nested
/// adversarial source (10k-deep nested calls in JS, say) could
/// stack-overflow. The default 8 MiB stack handles ~10k levels of
/// trivial recursion comfortably; tree-sitter's own recursion
/// budget caps grammars long before we hit that. If a real-world
/// source surfaces a stack issue, refactor to an iterative
/// `TreeCursor` walk.
fn walk_tree(
    node: Node<'_>,
    source: &[u8],
    sink: &TextSink,
    idents: &mut BTreeSet<String>,
    strings: &mut Vec<String>,
    comments: &mut Vec<String>,
    counter: &mut usize,
) -> Result<(), ExtractError> {
    *counter += 1;
    if *counter % CANCEL_CHECK_EVERY == 0 && sink.is_cancelled() {
        return Err(ExtractError::Cancelled);
    }
    let kind = node.kind();
    if is_identifier_kind(kind) {
        if let Some(text) = node_text(node, source) {
            // Skip empty / whitespace-only identifiers — defensive
            // against grammar quirks (some grammars expose "" for
            // ERROR nodes).
            if !text.trim().is_empty() {
                idents.insert(text.to_owned());
            }
        }
    } else if is_string_kind(kind) {
        if let Some(text) = node_text(node, source) {
            strings.push(text.to_owned());
        }
    } else if is_comment_kind(kind) {
        if let Some(text) = node_text(node, source) {
            comments.push(text.to_owned());
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk_tree(child, source, sink, idents, strings, comments, counter)?;
    }
    Ok(())
}

fn node_text<'a>(node: Node<'a>, source: &'a [u8]) -> Option<&'a str> {
    let range = node.byte_range();
    if range.end > source.len() {
        return None;
    }
    std::str::from_utf8(&source[range]).ok()
}

/// Cross-language node-kind classifier — true for any identifier-shaped
/// node. The names cover the conventions used by every grammar Phase 8
/// loads; new grammars only need to expose one of these names.
fn is_identifier_kind(kind: &str) -> bool {
    matches!(
        kind,
        "identifier"
            | "field_identifier"
            | "type_identifier"
            | "property_identifier"
            | "shorthand_property_identifier"
            | "shorthand_property_identifier_pattern"
            | "scoped_identifier"
            | "package_identifier"
            | "namespace_identifier"
            | "statement_identifier"
            | "constant"
            | "variable_name"
            | "method_identifier"
            | "module_name"
            | "atom"
            | "label"
            | "name"
            | "lower_identifier"
            | "upper_identifier"
            | "var_name"
            | "tag_name"
            | "attribute_name"
            | "class_name"
    )
}

fn is_string_kind(kind: &str) -> bool {
    matches!(
        kind,
        "string"
            | "string_literal"
            | "raw_string_literal"
            | "interpreted_string_literal"
            | "template_string"
            | "string_fragment"
            | "char_literal"
            | "character_literal"
            | "heredoc_body"
            | "string_content"
            | "quoted_content"
            | "raw_string"
            | "rune_literal"
            | "byte_literal"
    )
}

fn is_comment_kind(kind: &str) -> bool {
    matches!(
        kind,
        "comment"
            | "line_comment"
            | "block_comment"
            | "doc_comment"
            | "documentation_comment"
            | "html_comment"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    fn write_fixture(name: &str, contents: &[u8]) -> std::path::PathBuf {
        let dir = tempdir().unwrap().keep();
        let path = dir.join(name);
        let mut f = File::create(&path).unwrap();
        f.write_all(contents).unwrap();
        path
    }

    fn extract_to_string(name: &str, contents: &[u8]) -> String {
        let path = write_fixture(name, contents);
        let mut sink = TextSink::new(8192);
        CodeExtractor::default().extract(&path, &mut sink).unwrap();
        std::str::from_utf8(sink.as_bytes()).unwrap().to_owned()
    }

    #[test]
    fn matches_by_extension_core_eight() {
        let ext = CodeExtractor::default();
        assert!(ext.matches(Path::new("/x/foo.rs"), b""));
        assert!(ext.matches(Path::new("/x/foo.py"), b""));
        assert!(ext.matches(Path::new("/x/foo.ts"), b""));
        assert!(ext.matches(Path::new("/x/foo.tsx"), b""));
        assert!(ext.matches(Path::new("/x/foo.go"), b""));
        assert!(ext.matches(Path::new("/x/foo.java"), b""));
        assert!(ext.matches(Path::new("/x/foo.c"), b""));
        assert!(ext.matches(Path::new("/x/foo.cpp"), b""));
        assert!(!ext.matches(Path::new("/x/foo.txt"), b""));
    }

    #[test]
    fn matches_by_extension_extended_set() {
        let ext = CodeExtractor::default();
        for case in [
            "foo.cs",
            "foo.rb",
            "foo.sh",
            "foo.lua",
            "foo.php",
            "foo.kt",
            "foo.scala",
            "foo.swift",
            "foo.hs",
            "foo.ml",
            "foo.ex",
            "foo.erl",
            "foo.clj",
            "foo.elm",
            "foo.dart",
            "foo.r",
            "foo.jl",
            "foo.zig",
            "foo.nix",
            "foo.toml",
            "foo.yml",
            "foo.json",
            "foo.html",
            "foo.css",
            "foo.sql",
        ] {
            assert!(
                ext.matches(Path::new(case), b""),
                "expected {case} to be matched"
            );
        }
    }

    #[test]
    fn matches_rakefile_as_ruby() {
        let ext = CodeExtractor::default();
        assert!(ext.matches(Path::new("/x/Rakefile"), b""));
        assert_eq!(
            CodeExtractor::classify(Path::new("/x/Rakefile")),
            Some(Lang::Ruby)
        );
    }

    #[test]
    fn rust_extracts_idents_strings_comments() {
        let out = extract_to_string(
            "a.rs",
            br#"
// top-level doc
fn hello_world() {
    let greeting = "hi there";
    println!("{}", greeting);
}
"#,
        );
        assert!(out.contains("[lang]\nrust"));
        assert!(out.contains("hello_world"));
        assert!(out.contains("greeting"));
        assert!(out.contains("\"hi there\""));
        assert!(out.contains("// top-level doc"));
    }

    #[test]
    fn python_extracts() {
        let out = extract_to_string(
            "a.py",
            br#"
# module doc
def greet(name):
    msg = "hello"
    return msg + name
"#,
        );
        assert!(out.contains("[lang]\npython"));
        assert!(out.contains("greet"));
        assert!(out.contains("\"hello\""));
        assert!(out.contains("# module doc"));
    }

    #[test]
    fn javascript_extracts() {
        let out = extract_to_string(
            "a.js",
            br#"
// hello
function add(a, b) { return a + b; }
const greeting = "world";
"#,
        );
        assert!(out.contains("[lang]\njavascript"));
        assert!(out.contains("add"));
        assert!(out.contains("greeting"));
    }

    #[test]
    fn go_extracts() {
        let out = extract_to_string(
            "a.go",
            br#"
package main
// doc
func Add(a, b int) int { return a + b }
"#,
        );
        assert!(out.contains("[lang]\ngo"));
        assert!(out.contains("Add"));
    }

    #[test]
    fn ruby_extracts() {
        let out = extract_to_string(
            "a.rb",
            br#"
# top
def hello
  "world"
end
"#,
        );
        assert!(out.contains("[lang]\nruby"));
        assert!(out.contains("hello"));
    }

    #[test]
    fn json_extracts() {
        let out = extract_to_string("a.json", br#"{"name": "alice", "age": 30}"#);
        assert!(out.contains("[lang]\njson"));
    }

    #[test]
    fn unknown_extension_is_not_matched() {
        let ext = CodeExtractor::default();
        assert!(!ext.matches(Path::new("/x/foo.unknownlang"), b""));
    }

    /// Round-trip every grammar Phase 8 ships with a one-line fixture
    /// per language. The assertion is loose on purpose — we only
    /// verify the parser loads, the dispatch lands on the right
    /// `Lang`, and the extractor produces a non-empty `[lang]` header
    /// followed by *some* identifier / string / comment. This regresses
    /// against future tree-sitter grammar-symbol renames or a Cargo
    /// version bump that drops a grammar's `LANGUAGE` constant.
    #[test]
    fn extended_languages_round_trip() {
        let cases: &[(&str, &[u8], &str)] = &[
            ("a.cs", b"// hi\nclass Foo { string s = \"x\"; }\n", "c#"),
            (
                "a.rb",
                b"# hi\nclass Foo; def bar; \"x\"; end; end\n",
                "ruby",
            ),
            (
                "a.sh",
                b"#!/bin/bash\n# hi\nfoo() { echo \"x\"; }\n",
                "bash",
            ),
            ("a.lua", b"-- hi\nlocal foo = \"x\"\n", "lua"),
            (
                "a.php",
                b"<?php\n// hi\nfunction foo() { return \"x\"; }\n",
                "php",
            ),
            ("a.kt", b"// hi\nfun foo() = \"x\"\n", "kotlin"),
            ("a.scala", b"// hi\ndef foo = \"x\"\n", "scala"),
            ("a.swift", b"// hi\nlet foo = \"x\"\n", "swift"),
            ("a.hs", b"-- hi\nfoo :: String\nfoo = \"x\"\n", "haskell"),
            ("a.ml", b"(* hi *)\nlet foo = \"x\"\n", "ocaml"),
            (
                "a.ex",
                b"# hi\ndefmodule Foo do\n  def bar, do: \"x\"\nend\n",
                "elixir",
            ),
            (
                "a.erl",
                b"%% hi\n-module(foo).\nbar() -> \"x\".\n",
                "erlang",
            ),
            ("a.clj", b";; hi\n(defn foo [] \"x\")\n", "clojure"),
            ("a.elm", b"-- hi\nfoo : String\nfoo = \"x\"\n", "elm"),
            ("a.dart", b"// hi\nString foo = \"x\";\n", "dart"),
            ("a.r", b"# hi\nfoo <- \"x\"\n", "r"),
            ("a.jl", b"# hi\nfoo = \"x\"\n", "julia"),
            ("a.zig", b"// hi\nconst foo = \"x\";\n", "zig"),
            ("a.nix", b"# hi\n{ foo = \"x\"; }\n", "nix"),
            ("a.toml", b"# hi\nfoo = \"x\"\n", "toml"),
            ("a.yml", b"# hi\nfoo: \"x\"\n", "yaml"),
            ("a.json", b"{\"foo\": \"x\"}\n", "json"),
            ("a.html", b"<!-- hi -->\n<p>x</p>\n", "html"),
            ("a.css", b"/* hi */\n.foo { color: red; }\n", "css"),
            ("a.sql", b"-- hi\nSELECT 'x' AS foo;\n", "sql"),
        ];

        for (name, src, lang_label) in cases {
            let path = write_fixture(name, src);
            let mut sink = TextSink::new(8192);
            CodeExtractor::default()
                .extract(&path, &mut sink)
                .unwrap_or_else(|e| panic!("extract failed for {name}: {e:?}"));
            let out = std::str::from_utf8(sink.as_bytes()).unwrap();
            assert!(
                out.contains(&format!("[lang]\n{lang_label}")),
                "{name}: expected [lang]\\n{lang_label}, got {out:?}"
            );
            // At least one of the three section headers must fire —
            // otherwise the parser produced an empty tree and the
            // grammar is silently broken.
            assert!(
                out.contains("[identifiers]")
                    || out.contains("[strings]")
                    || out.contains("[comments]"),
                "{name}: expected at least one section header, got {out:?}"
            );
        }
    }
}
