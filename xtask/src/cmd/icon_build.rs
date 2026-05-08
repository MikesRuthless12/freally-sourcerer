use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{Options, Tree};

use crate::workspace_root;

/// Standard PNG sizes shipped in `assets/icons/png/`.
const PNG_SIZES: &[u32] = &[16, 32, 48, 64, 128, 256, 512, 1024];

/// Hicolor PNG-set sizes shipped under `assets/icons/hicolor/<size>x<size>/apps/sourcerer.png`.
const HICOLOR_SIZES: &[u32] = &[16, 22, 24, 32, 48, 64, 96, 128, 256, 512];

/// Sizes embedded into the Windows `.ico` container.
const ICO_SIZES: &[u32] = &[16, 24, 32, 48, 64, 128, 256];

/// Sizes embedded into the macOS `.icns` container, with their OSType codes.
/// `icns` 0.3 supports these standard sizes via `IconType::from_ostype`.
const ICNS_TYPES: &[(u32, [u8; 4])] = &[
    (16, *b"icp4"),   // 16x16
    (32, *b"icp5"),   // 32x32
    (64, *b"icp6"),   // 64x64
    (128, *b"ic07"),  // 128x128
    (256, *b"ic08"),  // 256x256
    (512, *b"ic09"),  // 512x512
    (1024, *b"ic10"), // 1024x1024 @ 2x
];

pub fn run(assets: Option<PathBuf>) -> Result<()> {
    let assets = assets.unwrap_or_else(|| workspace_root().join("assets").join("icons"));
    let svg_path = assets.join("sourcerer.svg");
    let svg_data = fs::read(&svg_path).with_context(|| format!("read {}", svg_path.display()))?;
    let tree = Tree::from_data(&svg_data, &Options::default())
        .with_context(|| format!("parse SVG {}", svg_path.display()))?;

    // 1. Plain PNG set under assets/icons/png/.
    let png_dir = assets.join("png");
    fs::create_dir_all(&png_dir)?;
    for &size in PNG_SIZES {
        let pixmap = render(&tree, size)?;
        let path = png_dir.join(format!("sourcerer-{size}.png"));
        pixmap
            .save_png(&path)
            .with_context(|| format!("write {}", path.display()))?;
    }
    println!("wrote {} PNG sizes", PNG_SIZES.len());

    // 2. Hicolor tree under assets/icons/hicolor/<size>x<size>/apps/sourcerer.png.
    let hicolor = assets.join("hicolor");
    for &size in HICOLOR_SIZES {
        let dir = hicolor.join(format!("{size}x{size}")).join("apps");
        fs::create_dir_all(&dir)?;
        let path = dir.join("sourcerer.png");
        let pixmap = render(&tree, size)?;
        pixmap
            .save_png(&path)
            .with_context(|| format!("write {}", path.display()))?;
    }
    println!("wrote Hicolor PNG set ({} sizes)", HICOLOR_SIZES.len());

    // 3. Windows .ico.
    write_ico(&tree, &assets.join("sourcerer.ico"))?;

    // 4. macOS .icns.
    write_icns(&tree, &assets.join("sourcerer.icns"))?;

    Ok(())
}

fn render(tree: &Tree, size: u32) -> Result<Pixmap> {
    let mut pixmap = Pixmap::new(size, size).context("alloc pixmap")?;
    let view = tree.size();
    let scale = size as f32 / view.width().max(view.height());
    let transform = Transform::from_scale(scale, scale);
    resvg::render(tree, transform, &mut pixmap.as_mut());
    Ok(pixmap)
}

fn write_ico(tree: &Tree, path: &Path) -> Result<()> {
    let mut dir = ico::IconDir::new(ico::ResourceType::Icon);
    for &size in ICO_SIZES {
        let pixmap = render(tree, size)?;
        let image = ico::IconImage::from_rgba_data(size, size, pixmap.data().to_vec());
        let entry = ico::IconDirEntry::encode(&image)
            .with_context(|| format!("encode .ico size {size}"))?;
        dir.add_entry(entry);
    }
    let file = fs::File::create(path).with_context(|| format!("create {}", path.display()))?;
    dir.write(file).context("write .ico")?;
    println!("wrote {}", path.display());
    Ok(())
}

fn write_icns(tree: &Tree, path: &Path) -> Result<()> {
    let mut family = icns::IconFamily::new();
    for (size, ostype) in ICNS_TYPES {
        let pixmap = render(tree, *size)?;
        let img = icns::Image::from_data(
            icns::PixelFormat::RGBA,
            *size,
            *size,
            pixmap.data().to_vec(),
        )
        .with_context(|| format!("icns image at {size}px"))?;
        let kind = icns::IconType::from_ostype(icns::OSType(*ostype))
            .ok_or_else(|| anyhow::anyhow!("unknown icns OSType {:?}", ostype))?;
        if family.has_icon_with_type(kind) {
            continue;
        }
        if let Err(e) = family.add_icon_with_type(&img, kind) {
            // Some sizes may not be supported by the `icns` crate version; warn and skip.
            bail!("icns add {size}px: {e}");
        }
    }
    let mut file = fs::File::create(path).with_context(|| format!("create {}", path.display()))?;
    family.write(&mut file).context("write .icns")?;
    println!("wrote {}", path.display());
    Ok(())
}
