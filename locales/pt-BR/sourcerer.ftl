# Sourcerer — Português (BR).

app-name = Sourcerer
tagline = Uma busca. Toda fonte. Todo SO.
window-title = Sourcerer
search-placeholder = Pesquisar…
about-version = Versão { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Pronto
status-indexed = Indexado ({ $count } arquivos)
status-indexing = Indexando… { $done }/{ $total }
status-paused = Pausado
status-error = Erro
status-result-count-one = { $count } resultado
status-result-count-many = { $count } resultados
status-selection = · { $count } selecionado(s)
status-selection-size = Selecionado: { $size }
status-query-timing = Consulta: { $ms } ms
status-endpoint-local = Banco de dados local
status-endpoint-remote = API: { $name }

menu-file = Arquivo
menu-edit = Editar
menu-view = Exibir
menu-search = Pesquisar
menu-bookmarks = Favoritos
menu-tools = Ferramentas
menu-help = Ajuda

theme-system = Sistema
theme-light = Claro
theme-dark = Escuro

lens-filename = Nome do arquivo
lens-content = Conteúdo
lens-audio = Áudio
lens-similarity = Similaridade

parse-error-empty = Digite uma consulta para começar.
parse-error-unknown = Sintaxe não reconhecida aqui.

action-open = Abrir
action-reveal = Mostrar na pasta
action-copy-path = Copiar caminho
action-copy-name = Copiar nome
action-delete = Excluir

quick-filter-audio = Áudio
quick-filter-video = Vídeo
quick-filter-image = Imagem
quick-filter-document = Documento
quick-filter-executable = Executável
quick-filter-archive = Compactado

wizard-title = Bem-vindo ao Sourcerer
wizard-step-roots = Escolha o que indexar
wizard-step-hotkey = Escolha uma tecla de atalho global
wizard-step-locale = Escolha seu idioma
wizard-step-theme = Escolha um tema
wizard-finish = Concluir

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Opções
settings-search-placeholder = Pesquisar opções…
settings-restore-defaults = Restaurar padrões
settings-ok = OK
settings-cancel = Cancelar
settings-apply = Aplicar

# Tree nav groups (PRD §8.1.1).
settings-group-general = Geral
settings-group-indexes = Índices
settings-group-lenses = Lentes
settings-group-network = Rede

# Tree nav leaves.
settings-node-ui = Interface
settings-node-home = Início
settings-node-search = Pesquisa
settings-node-results = Resultados
settings-node-view = Exibição
settings-node-context-menu = Menu de contexto
settings-node-fonts-colors = Fontes e cores
settings-node-keyboard = Teclado
settings-node-history = Histórico
settings-node-indexes-top = (nível superior)
settings-node-volumes = Volumes
settings-node-folders = Pastas
settings-node-file-lists = Listas de arquivos
settings-node-exclude = Excluir
settings-node-https-server = Servidor HTTP / HTTPS
settings-node-etp-api = API ETP / FTP
settings-node-privacy = Privacidade e atualizações
settings-node-logs = Logs e depuração
settings-node-backup = Backup, exportação, redefinição
settings-node-locale = Idioma
settings-node-about = Sobre

# §8.2 General → UI.
settings-ui-theme = Tema
settings-ui-run-bg = Executar em segundo plano
settings-ui-show-tray = Mostrar ícone na bandeja / barra de menus
settings-ui-single-click-tray = Clique único na bandeja / barra de menus
settings-ui-new-window-from-tray = Abrir nova janela a partir do ícone da bandeja
settings-ui-new-window-on-launch = Abrir nova janela ao iniciar o Sourcerer
settings-ui-search-as-you-type = Pesquisar enquanto digita
settings-ui-select-on-mouse-click = Selecionar pesquisa ao clicar com o mouse
settings-ui-focus-on-activate = Focar pesquisa ao ativar
settings-ui-full-row-select = Selecionar linha inteira
settings-ui-single-click-open = Abrir com clique único
settings-ui-underline-titles = Sublinhar títulos dos ícones
settings-ui-row-density = Densidade dos resultados
settings-ui-row-density-compact = Compacta (32 px)
settings-ui-row-density-comfortable = Confortável (44 px)
settings-ui-show-timing-badges = Mostrar selos de tempo por lente
settings-ui-anim-crossfade = Transição animada entre temas

# §8.3 General → Home.
settings-home-match-case = Diferenciar maiúsculas e minúsculas
settings-home-match-whole-word = Coincidir palavra inteira
settings-home-match-path = Coincidir caminho
settings-home-match-diacritics = Coincidir acentos
settings-home-match-regex = Coincidir Regex
settings-home-search = Pesquisa (consulta padrão personalizada)
settings-home-filter = Filtro
settings-home-sort = Ordenação
settings-home-view = Exibição
settings-home-index = Índice
settings-home-default-lens-visibility = Visibilidade padrão das lentes
settings-home-default-lens-result-limits = Limites padrão de resultados por lente

# §8.4 General → Search.
settings-search-fast-ascii = Pesquisa ASCII rápida
settings-search-mp-sep = Coincidir caminho quando o termo contiver um separador de caminho
settings-search-mw-fn = Coincidir nome de arquivo inteiro ao usar curingas
settings-search-lit-ops = Permitir operadores literais
settings-search-paren = Permitir agrupamento por parênteses
settings-search-env = Expandir variáveis de ambiente
settings-search-fwd-slash = Substituir barras por contrabarras
settings-search-precedence = Precedência de operadores
settings-search-strict-everything = Modo estrito de sintaxe Everything
settings-search-auto-regex = Detectar Regex automaticamente
settings-search-mod-comp = Conclusão de modificadores
settings-search-parse-tree = Mostrar árvore de análise ao passar o mouse

# §8.5 General → Results.
settings-results-hide-empty = Ocultar resultados quando a pesquisa estiver vazia
settings-results-clear-on-search = Limpar seleção ao pesquisar
settings-results-close-on-execute = Fechar janela ao executar
settings-results-dbl-path = Abrir caminho com clique duplo na coluna de caminho
settings-results-auto-scroll = Rolar a exibição automaticamente
settings-results-dquote-copy = Copiar entre aspas duplas como caminho
settings-results-no-ext-rename = Não selecionar a extensão ao renomear
settings-results-sort-date-desc = Ordenar por data decrescente primeiro
settings-results-sort-size-desc = Ordenar por tamanho decrescente primeiro
settings-results-list-focus = Foco da lista de resultados
settings-results-icon-prio = Prioridade de carregamento de ícones
settings-results-thumb-prio = Prioridade de carregamento de miniaturas
settings-results-ext-prio = Prioridade de carregamento de informações estendidas
settings-results-group-by-lens = Agrupar resultados por lente
settings-results-snippet-inline = Mostrar pré-visualização do trecho em linha

# §8.6 General → View.
settings-view-double-buffer = Buffer duplo
settings-view-alt-rows = Alternar cor das linhas
settings-view-row-mouseover = Mostrar destaque ao passar o mouse
settings-view-highlight-terms = Destacar termos pesquisados
settings-view-status-show-selected = Mostrar item selecionado na barra de status
settings-view-rc-with-sel = Mostrar contagem de resultados junto com a contagem de seleção
settings-view-status-show-size = Mostrar tamanho na barra de status
settings-view-tooltips = Mostrar dicas de ferramentas
settings-view-update-on-scroll = Atualizar exibição imediatamente após rolagem
settings-view-size-format = Formato de tamanho
settings-view-selection-rect = Retângulo de seleção
settings-view-audio-badges = Mostrar selos de LUFS / codec / duração nas linhas de áudio
settings-view-similarity-score = Mostrar pontuação de similaridade MinHash nas linhas de similaridade
settings-view-preview-pane = Painel de pré-visualização

# §8.7 General → Context Menu.
settings-context-menu-visibility = Visibilidade
settings-context-menu-show = Mostrar
settings-context-menu-shift = Mostrar somente com Shift pressionado
settings-context-menu-hide = Ocultar
settings-context-menu-command = Macro de comando
settings-context-menu-open-folders = Abrir (Pastas)
settings-context-menu-open-files = Abrir (Arquivos)
settings-context-menu-open-path = Abrir caminho
settings-context-menu-explore = Explorar
settings-context-menu-explore-path = Explorar caminho
settings-context-menu-copy-name = Copiar nome para a área de transferência
settings-context-menu-copy-path = Copiar caminho para a área de transferência
settings-context-menu-copy-full-name = Copiar nome completo para a área de transferência
settings-context-menu-reveal = Mostrar no Sourcerer
settings-context-menu-send-to = Enviar para o Sourcerer (caminho)

# §8.8 General → Fonts & Colors.
settings-fc-font = Fonte
settings-fc-size = Tamanho
settings-fc-state-normal = Normal
settings-fc-state-highlighted = Destacado
settings-fc-state-current-sort = Ordenação atual
settings-fc-state-current-sort-h = Ordenação atual (destacada)
settings-fc-state-selected = Selecionado
settings-fc-state-selected-h = Selecionado (destacado)
settings-fc-state-inactive-selected = Selecionado inativo
settings-fc-state-inactive-selected-h = Selecionado inativo (destacado)
settings-fc-foreground = Primeiro plano
settings-fc-background = Fundo
settings-fc-bold = Negrito
settings-fc-italic = Itálico
settings-fc-default = Padrão
settings-fc-per-lens-accent = Cor de destaque por lente
settings-fc-theme-inherit = Inverter cores personalizadas automaticamente ao trocar de tema

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Tecla de atalho global
settings-keyboard-new-window = Tecla de atalho de nova janela
settings-keyboard-show-window = Tecla de atalho para mostrar janela
settings-keyboard-toggle-window = Tecla de atalho para alternar janela
settings-keyboard-show-commands = Mostrar comandos contendo
settings-keyboard-add-chord = + Adicionar combinação
settings-keyboard-remove-chord = Remover

# §8.10 History.
settings-history-search-enable = Habilitar histórico de pesquisa
settings-history-search-keep = Manter histórico de pesquisa por { $days } dias
settings-history-run-enable = Habilitar histórico de execução
settings-history-run-keep = Manter histórico de execução por { $days } dias
settings-history-clear-now = Limpar agora
settings-history-privacy-mode = Modo de privacidade
settings-history-per-lens = Histórico por lente

# §8.11 Indexes (top-level).
settings-ix-database-location = Local do banco de dados
settings-ix-multiuser = Nome do banco de dados multiusuário
settings-ix-compress = Compactar banco de dados
settings-ix-recent-changes = Indexar alterações recentes
settings-ix-file-size = Indexar tamanho de arquivo
settings-ix-fast-size-sort = Ordenação rápida por tamanho
settings-ix-folder-size = Indexar tamanho de pasta
settings-ix-fast-folder-size-sort = Ordenação rápida por tamanho de pasta
settings-ix-date-created = Indexar data de criação
settings-ix-fast-date-created = Ordenação rápida por data de criação
settings-ix-date-modified = Indexar data de modificação
settings-ix-fast-date-modified = Ordenação rápida por data de modificação
settings-ix-date-accessed = Indexar data de acesso
settings-ix-fast-date-accessed = Ordenação rápida por data de acesso
settings-ix-attributes = Indexar atributos
settings-ix-fast-attributes = Ordenação rápida por atributos
settings-ix-fast-path-sort = Ordenação rápida por caminho
settings-ix-fast-extension-sort = Ordenação rápida por extensão
settings-ix-force-rebuild = Forçar reconstrução
settings-ix-compact = Compactar índice
settings-ix-verify = Verificar índice
settings-ix-integrity-policy = Política de integridade do índice
settings-ix-memory-budget = Orçamento de memória do indexador
settings-ix-throttle = Controle de indexação em segundo plano

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Incluir novos volumes fixos automaticamente
settings-vol-auto-removable = Incluir novos volumes removíveis automaticamente
settings-vol-auto-remove-offline = Remover volumes offline automaticamente
settings-vol-detected = Volumes detectados
settings-vol-include = Incluir no índice
settings-vol-include-only = Incluir somente (glob/Regex)
settings-vol-enable-usn = Habilitar USN Journal
settings-vol-enable-fsevents = Habilitar fluxo FSEvents
settings-vol-enable-inotify = Habilitar inotify (ou fanotify se elevado)
settings-vol-buffer = Tamanho do buffer do journal (KB)
settings-vol-allocation-delta = Delta de alocação (KB)
settings-vol-load-recent = Carregar alterações recentes do journal na inicialização
settings-vol-monitor = Monitorar alterações
settings-vol-recreate-journal = Recriar journal
settings-vol-reset-stream = Redefinir fluxo FSEvents
settings-vol-upgrade-fanotify = Atualizar para fanotify (polkit)
settings-vol-remove = Remover

# §8.13 Indexes → Folders.
settings-folders-watched = Pastas monitoradas
settings-folders-add = Adicionar…
settings-folders-rescan-now = Reescanear agora
settings-folders-rescan-all = Reescanear tudo agora
settings-folders-monitor = Tentar monitorar alterações
settings-folders-buffer = Tamanho do buffer
settings-folders-rescan-on-full = Reescanear quando o buffer estiver cheio

# §8.14 Indexes → File Lists.
settings-flists-add = Adicionar…
settings-flists-monitor = Monitorar alterações
settings-flists-editor = Editor de listas de arquivos…
settings-flists-format = Formato da lista de arquivos
settings-flists-format-text = Texto (um caminho por linha)
settings-flists-format-json = JSON (com metadados)
settings-flists-format-srcb = Pacote Sourcerer (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Excluir arquivos e pastas ocultos
settings-exclude-system = Excluir arquivos e pastas do sistema
settings-exclude-list-en = Habilitar lista de exclusão
settings-exclude-folders = Excluir pastas
settings-exclude-include-only-files = Incluir somente arquivos (glob)
settings-exclude-files = Excluir arquivos (glob)
settings-exclude-os-recommended = Aplicar exclusões recomendadas pelo SO
settings-exclude-by-class = Excluir por classe de extensão

# §8.16 Lenses → Filename.
settings-lf-trigram = Agressividade do pré-filtro de trigram
settings-lf-suffix-mem = Orçamento de memória do array de sufixos
settings-lf-wildcard-limit = Limite de expansão de curingas
settings-lf-regex-timeout = Tempo limite do Regex

# §8.17 Lenses → Content.
settings-lc-enable = Habilitar lente de conteúdo
settings-lc-time-budget = Orçamento de tempo por documento
settings-lc-mem-ceiling = Limite de memória por documento
settings-lc-snippet-len = Tamanho do trecho
settings-lc-stop-words = Idioma das palavras de parada
settings-lc-re-extract = Reextrair ao alterar configurações
settings-lc-verify-blobs = Verificar checksums dos blobs de texto extraído ao ler

# §8.18 Lenses → Audio.
settings-la-enable = Habilitar lente de áudio
settings-la-lufs-ref = Padrão de referência LUFS
settings-la-peak-compute = Calcular pico via
settings-la-silence-thresh = Limiar de silêncio
settings-la-re-extract-modify = Reextrair em evento de modificação

# §8.19 Lenses → Similarity.
settings-ls-enable = Habilitar lente de similaridade
settings-ls-sig-size = Tamanho da assinatura MinHash (k)
settings-ls-bands = Bandas LSH
settings-ls-recall = Limiar de revocação
settings-ls-result-cap = Limite de resultados

# §8.20 Lenses → Custom.
settings-custom-registry = Registro
settings-custom-trust = Confiança
settings-custom-refresh-hashes = Atualizar hashes

# §8.21-§8.22 Network.
settings-net-https-enable = Habilitar servidor HTTPS
settings-net-bind = Vincular a interfaces
settings-net-port = Escutar na porta
settings-net-force-https = Forçar HTTPS
settings-net-legacy-auth = Autenticação básica HTTP legada
settings-net-token-regen = Regenerar token
settings-net-api-enable = Habilitar servidor de API
settings-net-legacy-ftp = Suporte legado a FTP/ETP em texto puro

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Atualização automática
settings-privacy-prerelease = Canal de pré-lançamento
settings-privacy-network-policy = Política de chamadas de rede

# §8.24 Logs & Debug.
settings-logs-level = Nível de log
settings-logs-location = Local do arquivo de log
settings-logs-retention = Retenção de logs
settings-logs-debug-overlay = Mostrar sobreposição de depuração
settings-logs-open-folder = Abrir pasta de logs
settings-logs-export-bundle = Exportar pacote de diagnóstico

# §8.25 Backup, Export, Reset.
settings-backup-export = Exportar configurações
settings-backup-import = Importar configurações
settings-backup-export-bookmarks = Exportar pacote de favoritos
settings-backup-import-bookmarks = Importar pacote de favoritos
settings-backup-reset-all = Redefinir todas as configurações para os padrões

# §8.26 Locale.
settings-locale-current = Idioma atual
settings-locale-rtl-preview = Pré-visualização RTL
settings-locale-date-format = Formato de data
settings-locale-number-format = Formato de número

# §8.27 About.
settings-about-version = Sourcerer { $version }
settings-about-license = Licença
settings-about-credits = Créditos
settings-about-notices = Avisos de código aberto
