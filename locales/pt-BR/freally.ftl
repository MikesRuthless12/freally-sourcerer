# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = Uma busca. Todas as fontes. Todos os sistemas.
window-title = Freally Sourcerer
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

wizard-title = Bem-vindo ao Freally
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
settings-ui-new-window-from-tray = Abrir nova janela pelo ícone da bandeja
settings-ui-new-window-on-launch = Abrir nova janela ao iniciar o Freally
settings-ui-search-as-you-type = Pesquisar enquanto digita
settings-ui-select-on-mouse-click = Selecionar pesquisa ao clicar com o mouse
settings-ui-focus-on-activate = Focar na pesquisa ao ativar
settings-ui-full-row-select = Selecionar linha inteira
settings-ui-single-click-open = Abrir com clique único
settings-ui-underline-titles = Sublinhar títulos dos ícones
settings-ui-row-density = Densidade dos resultados
settings-ui-row-density-compact = Compacta (32 px)
settings-ui-row-density-comfortable = Confortável (44 px)
settings-ui-show-timing-badges = Mostrar selos de tempo por lente
settings-ui-anim-crossfade = Transição animada entre temas

# §8.3 General → Home.
settings-home-match-case = Diferenciar maiúsculas de minúsculas
settings-home-match-whole-word = Coincidir palavra inteira
settings-home-match-path = Coincidir caminho
settings-home-match-diacritics = Coincidir acentos
settings-home-match-regex = Coincidir regex
settings-home-search = Pesquisar (consulta padrão personalizada)
settings-home-filter = Filtrar
settings-home-sort = Ordenar
settings-home-view = Exibir
settings-home-index = Indexar
settings-home-default-lens-visibility = Visibilidade padrão das lentes
settings-home-default-lens-result-limits = Limites padrão de resultados por lente

# §8.4 General → Search.
settings-search-fast-ascii = Pesquisa ASCII rápida
settings-search-mp-sep = Coincidir caminho quando um termo contém um separador de caminho
settings-search-mw-fn = Coincidir nome de arquivo inteiro ao usar curingas
settings-search-lit-ops = Permitir operadores literais
settings-search-paren = Permitir agrupamento com parênteses
settings-search-env = Expandir variáveis de ambiente
settings-search-fwd-slash = Substituir barras por contrabarras
settings-search-precedence = Precedência de operadores
settings-search-strict-everything = Modo de sintaxe estrita do Everything
settings-search-auto-regex = Detectar regex automaticamente
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
settings-results-list-focus = Foco na lista de resultados
settings-results-icon-prio = Prioridade de carregamento de ícones
settings-results-thumb-prio = Prioridade de carregamento de miniaturas
settings-results-ext-prio = Prioridade de carregamento de informações estendidas
settings-results-group-by-lens = Agrupar resultados por lente
settings-results-snippet-inline = Mostrar prévia do trecho em linha

# §8.6 General → View.
settings-view-double-buffer = Buffer duplo
settings-view-alt-rows = Cor alternada das linhas
settings-view-row-mouseover = Destacar linha ao passar o mouse
settings-view-highlight-terms = Destacar termos pesquisados
settings-view-status-show-selected = Mostrar item selecionado na barra de status
settings-view-rc-with-sel = Mostrar a contagem de resultados com a contagem da seleção
settings-view-status-show-size = Mostrar tamanho na barra de status
settings-view-tooltips = Mostrar dicas de ferramenta
settings-view-update-on-scroll = Atualizar a exibição imediatamente após rolar
settings-view-size-format = Formato de tamanho
settings-view-selection-rect = Retângulo de seleção
settings-view-audio-badges = Mostrar selos de LUFS / codec / duração nas linhas de áudio
settings-view-similarity-score = Mostrar pontuação de similaridade MinHash nas linhas de similaridade
settings-view-preview-pane = Painel de prévia

# §8.7 General → Context Menu.
settings-context-menu-visibility = Visibilidade
settings-context-menu-show = Mostrar
settings-context-menu-shift = Mostrar apenas com Shift pressionado
settings-context-menu-hide = Ocultar
settings-context-menu-command = Macro de comando
settings-context-menu-open-folders = Abrir (pastas)
settings-context-menu-open-files = Abrir (arquivos)
settings-context-menu-open-path = Abrir caminho
settings-context-menu-explore = Explorar
settings-context-menu-explore-path = Explorar caminho
settings-context-menu-copy-name = Copiar nome para a área de transferência
settings-context-menu-copy-path = Copiar caminho para a área de transferência
settings-context-menu-copy-full-name = Copiar nome completo para a área de transferência
settings-context-menu-reveal = Mostrar no Freally
settings-context-menu-send-to = Enviar para o Freally (caminho)

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
settings-fc-background = Plano de fundo
settings-fc-bold = Negrito
settings-fc-italic = Itálico
settings-fc-default = Padrão
settings-fc-per-lens-accent = Cor de destaque por lente
settings-fc-theme-inherit = Inverter cores personalizadas automaticamente ao trocar de tema

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Tecla de atalho global
settings-keyboard-new-window = Tecla de atalho para nova janela
settings-keyboard-show-window = Tecla de atalho para mostrar janela
settings-keyboard-toggle-window = Tecla de atalho para alternar janela
settings-keyboard-show-commands = Mostrar comandos contendo
settings-keyboard-add-chord = + Adicionar combinação
settings-keyboard-remove-chord = Remover

# §8.10 History.
settings-history-search-enable = Ativar histórico de pesquisa
settings-history-search-keep = Manter histórico de pesquisa por { $days } dias
settings-history-run-enable = Ativar histórico de execução
settings-history-run-keep = Manter histórico de execução por { $days } dias
settings-history-clear-now = Limpar agora
settings-history-privacy-mode = Modo de privacidade
settings-history-per-lens = Histórico por lente

# §8.11 Indexes (top-level).
settings-ix-database-location = Local do banco de dados
settings-ix-multiuser = Nome do arquivo do banco de dados multiusuário
settings-ix-compress = Compactar banco de dados
settings-ix-recent-changes = Indexar alterações recentes
settings-ix-file-size = Indexar tamanho do arquivo
settings-ix-fast-size-sort = Ordenação rápida por tamanho
settings-ix-folder-size = Indexar tamanho da pasta
settings-ix-fast-folder-size-sort = Ordenação rápida por tamanho da pasta
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
settings-ix-throttle = Limitação da indexação em segundo plano

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Incluir automaticamente novos volumes fixos
settings-vol-auto-removable = Incluir automaticamente novos volumes removíveis
settings-vol-auto-remove-offline = Remover automaticamente volumes offline
settings-vol-detected = Volumes detectados
settings-vol-include = Incluir no índice
settings-vol-include-only = Incluir apenas (glob/regex)
settings-vol-enable-usn = Ativar diário USN
settings-vol-enable-fsevents = Ativar fluxo FSEvents
settings-vol-enable-inotify = Ativar inotify (ou fanotify se elevado)
settings-vol-buffer = Tamanho do buffer do diário (KB)
settings-vol-allocation-delta = Delta de alocação (KB)
settings-vol-load-recent = Carregar alterações recentes do diário ao iniciar
settings-vol-monitor = Monitorar alterações
settings-vol-recreate-journal = Recriar diário
settings-vol-reset-stream = Redefinir fluxo FSEvents
settings-vol-upgrade-fanotify = Atualizar para fanotify (polkit)
settings-vol-remove = Remover

# §8.13 Indexes → Folders.
settings-folders-watched = Pastas monitoradas
settings-folders-add = Adicionar…
settings-folders-rescan-now = Reverificar agora
settings-folders-rescan-all = Reverificar tudo agora
settings-folders-monitor = Tentar monitorar alterações
settings-folders-buffer = Tamanho do buffer
settings-folders-rescan-on-full = Reverificar quando o buffer encher

# §8.14 Indexes → File Lists.
settings-flists-add = Adicionar…
settings-flists-monitor = Monitorar alterações
settings-flists-editor = Editor de listas de arquivos…
settings-flists-format = Formato da lista de arquivos
settings-flists-format-text = Texto (um caminho por linha)
settings-flists-format-json = JSON (com metadados)
settings-flists-format-srcb = Pacote do Freally (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Excluir arquivos e pastas ocultos
settings-exclude-system = Excluir arquivos e pastas do sistema
settings-exclude-list-en = Ativar lista de exclusão
settings-exclude-folders = Excluir pastas
settings-exclude-include-only-files = Incluir apenas arquivos (glob)
settings-exclude-files = Excluir arquivos (glob)
settings-exclude-os-recommended = Aplicar exclusões recomendadas pelo sistema
settings-exclude-by-class = Excluir por classe de extensão

# §8.16 Lenses → Filename.
settings-lf-trigram = Agressividade do pré-filtro de trigramas
settings-lf-suffix-mem = Orçamento de memória do vetor de sufixos
settings-lf-wildcard-limit = Limite de expansão de curingas
settings-lf-regex-timeout = Tempo limite do regex

# §8.17 Lenses → Content.
settings-lc-enable = Ativar lente de conteúdo
settings-lc-time-budget = Orçamento de tempo por documento
settings-lc-mem-ceiling = Limite de memória por documento
settings-lc-snippet-len = Tamanho do trecho
settings-lc-stop-words = Idioma das palavras vazias
settings-lc-re-extract = Reextrair ao alterar configurações
settings-lc-verify-blobs = Verificar checksums dos blobs de texto extraído na leitura

# §8.18 Lenses → Audio.
settings-la-enable = Ativar lente de áudio
settings-la-lufs-ref = Padrão de referência LUFS
settings-la-peak-compute = Calcular pico via
settings-la-silence-thresh = Limiar de silêncio
settings-la-re-extract-modify = Reextrair no evento de modificação

# §8.19 Lenses → Similarity.
settings-ls-enable = Ativar lente de similaridade
settings-ls-sig-size = Tamanho da assinatura MinHash (k)
settings-ls-bands = Bandas LSH
settings-ls-recall = Limiar de recall
settings-ls-result-cap = Limite de resultados

# §8.20 Lenses → Custom.
settings-custom-registry = Registro
settings-custom-trust = Confiança
settings-custom-refresh-hashes = Atualizar hashes

# §8.21-§8.22 Network.
settings-net-https-enable = Ativar servidor HTTPS
settings-net-bind = Vincular às interfaces
settings-net-port = Escutar na porta
settings-net-force-https = Forçar HTTPS
settings-net-legacy-auth = Autenticação HTTP básica legada
settings-net-token-regen = Regenerar token
settings-net-api-enable = Ativar servidor de API
settings-net-legacy-ftp = Suporte legado a FTP/ETP simples

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
settings-locale-rtl-preview = Prévia da direita para a esquerda
settings-locale-date-format = Formato de data
settings-locale-number-format = Formato de número

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = Licença
settings-about-credits = Créditos
settings-about-notices = Avisos de código aberto

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = Assistente de primeira execução
wizard-step-of-total = Etapa { $step } de { $total }
wizard-roots-hint = Adicione as pastas ou volumes que você quer que o Freally monitore. Você pode alterar isso depois nas configurações de Índices.
wizard-browse = Procurar…
wizard-roots-placeholder = …ou cole um caminho
wizard-roots-add = Adicionar
wizard-roots-remove = Remover
wizard-roots-empty = Nenhuma raiz configurada ainda.
wizard-locale-hint = O Freally está disponível em 18 idiomas. Você pode trocar depois.
wizard-theme-hint = O sistema segue a configuração de aparência do seu sistema operacional.
wizard-back = Voltar
wizard-next = Avançar

# Status bar polish.
statusbar-hotkey-hint = Tecla de atalho: { $hotkey }
statusbar-cycle-theme = Alternar tema
statusbar-indexed-suffix = indexado(s)

# Results / lenses.
lens-expand = Expandir lente
lens-collapse = Recolher lente
lens-no-matches = Nenhuma correspondência nesta lente.

# Preview pane.
preview-header = Prévia
preview-loading = Carregando…
preview-select-file = Selecione um arquivo para visualizar.
preview-unavailable = Nenhuma prévia disponível

# Bookmarks.
bookmarks-label = ★ Favoritos
bookmarks-empty-hint = Nenhum favorito ainda. Pressione Ctrl+D para salvar a consulta atual.
bookmarks-organize-title = Organizar favoritos
bookmarks-organize-empty = Nenhum favorito ainda.
bookmarks-rename = Renomear
bookmarks-close = Fechar

# Settings tree extras.
settings-group-history = Histórico
settings-group-privacy = Privacidade e atualizações
settings-group-logs = Logs e depuração
settings-group-backup = Backup, exportação, redefinição
settings-tree-custom-lens = Personalizada
settings-unsaved-changes = alterações não salvas

# About dialog.
about-dialog-title = Freally
about-copyright = Copyright © 2026 Mike Weaver. Todos os direitos reservados.
about-close = Fechar

# Connect endpoint dialog.
connect-ftp-title = Conectar ao servidor FTP
connect-ftp-host = Host:
connect-ftp-port = Porta:
connect-ftp-username = Nome de usuário:
connect-ftp-password = Senha:
connect-ftp-link-type = Tipo de conexão:

# UI panel.
ui-hint = Tema, integração com bandeja / barra de menus, pesquisa enquanto digita, densidade das linhas. Paridade direta com o Everything da voidtools, mais adições do Freally marcadas com (+).
ui-section-theme = Tema
ui-theme-system-default = Sistema (padrão)
ui-section-tray = Bandeja / barra de menus
ui-section-search-behavior = Comportamento da pesquisa
ui-section-result-rows = Linhas de resultado
ui-single-click-system-default = Configurações do sistema (padrão)
ui-single-click-always = Sempre clique único
ui-single-click-always-double = Sempre clique duplo
ui-underline-always = Sempre
ui-underline-on-hover = Ao passar o mouse
ui-underline-never = Nunca

# Home panel.
home-hint = Padrões carregados ao iniciar o app — cada lista suspensa pode manter "Usar último valor" ou fixar um valor. Visibilidade das lentes / limites de resultados são adições do Freally (+).
home-section-match = Padrões de correspondência
home-section-search-sort = Padrões de pesquisa e ordenação
home-search-placeholder = Vazio por padrão
home-section-index = Fonte do índice
home-file-list-path = Caminho da lista de arquivos
home-https-endpoint = URL do endpoint da API HTTPS
home-endpoint-token = Token (impressão digital exibida)

# Backup panel.
backup-section-settings = Configurações (+)
backup-section-bookmarks = Favoritos + extratores personalizados (+)
backup-section-reset = Redefinir
backup-toast-exported = Configurações exportadas para { $path }
backup-toast-export-failed = Falha na exportação: { $error }
backup-toast-imported = Configurações importadas
backup-toast-import-failed = Falha na importação: { $error }
backup-toast-bookmarks-exported = Favoritos exportados
backup-toast-bookmarks-export-failed = Falha na exportação dos favoritos: { $error }
backup-toast-bookmarks-imported = Favoritos importados
backup-toast-bookmarks-import-failed = Falha na importação dos favoritos: { $error }
backup-confirm-reset = Redefinir todas as configurações para os padrões? Isso não pode ser desfeito (a caixa de diálogo permanece aberta).
backup-toast-reset = Todas as configurações redefinidas

# Keyboard panel.
keyboard-section-global = Teclas de atalho globais
keyboard-placeholder-example = Super+Space
keyboard-section-commands = Comandos
keyboard-placeholder-command = id do comando (ex.: file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = Histórico de pesquisa
history-section-run = Histórico de execução
history-section-privacy = Privacidade (+)
history-record-filename = Registrar histórico da lente de nome de arquivo
history-record-content = Registrar histórico da lente de conteúdo
history-record-audio = Registrar histórico da lente de áudio
history-record-similarity = Registrar histórico da lente de similaridade

# Locale panel.
locale-section-language = Idioma (+)
locale-section-time-date = Hora / data (+)
locale-date-os = Padrão do sistema
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = Personalizado
locale-date-custom-format = Formato personalizado
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = Números (+)
locale-number-os = Padrão do sistema
locale-number-custom = Personalizado
locale-thousands-sep = Separador de milhares
locale-decimal-sep = Separador decimal

# Folders panel.
folders-hint = Pastas monitoradas adicionais além dos volumes padrão.
folders-list-title = Pastas monitoradas
folders-empty = Nenhuma pasta adicionada ainda.
folders-remove = Remover
folders-section-title-dynamic = Configurações de { $path }
folders-section-schedule = Agendamento de reverificação
folders-schedule-daily = Todos os dias às HH:MM
folders-schedule-hours = A cada N horas
folders-schedule-never = Nunca
folders-hour = Hora
folders-minute = Minuto
folders-hours = Horas
folders-id-label = ID da pasta (somente leitura)
folders-select-prompt = Selecione uma pasta para configurá-la.
folders-section-extras = Extras do Freally (+)
folders-extras-note = A reverificação ao retomar do modo de suspensão está ativada por padrão nesta versão; a opção se juntará aos controles de pasta no ajuste de refinamento da Fase 13.

# Volumes panel.
volumes-hint = Equivalente multiplataforma dos painéis NTFS / ReFS do Everything da voidtools. Detecta automaticamente NTFS / ReFS / exFAT / FAT32 (Win), APFS / HFS+ (macOS), ext4 / Btrfs / ZFS / XFS / F2FS (Linux).
volumes-section-auto-include = Inclusão automática
volumes-list-title = Volumes detectados
volumes-detecting = Detectando…
volumes-empty = Nenhum volume detectado.
volumes-select-prompt = Selecione um volume para configurá-lo.

# About panel polish.
about-section-version = Versão (+)
about-section-license = Licença (+)
about-license-text = Mike Weaver — Todos os direitos reservados. Este é um software proprietário.
about-license-spdx = SPDX: { $spdx }
about-section-credits = Créditos (+)
about-credits-inspired = Inspirado no Everything da voidtools.
about-credits-voidtools = voidtools.com
about-credits-repo = Repositório do projeto

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Contém comandos para trabalhar com o Freally.
menu-file-new-window = Nova janela de pesquisa
menu-file-open-list = Abrir lista de arquivos…
menu-file-close-list = Fechar lista de arquivos
menu-file-close = Fechar
menu-file-export-results = Exportar resultados…
menu-file-export-bundle = Exportar pacote de índice…
menu-file-exit = Sair

# Edit menu.
menu-edit-hint = Contém comandos para editar os resultados da pesquisa.
menu-edit-cut = Recortar
menu-edit-copy = Copiar
menu-edit-paste = Colar
menu-edit-copy-to-folder = Copiar para pasta…
menu-edit-move-to-folder = Mover para pasta…
menu-edit-select-all = Selecionar tudo
menu-edit-invert-selection = Inverter seleção
menu-edit-advanced = Avançado
menu-edit-copy-full-name = Copiar nome completo
menu-edit-copy-path = Copiar caminho
menu-edit-copy-filename = Copiar nome do arquivo
menu-edit-copy-as-json = Copiar como JSON
menu-edit-copy-with-metadata = Copiar com metadados
menu-edit-copy-as-bundle-ref = Copiar como referência de pacote do Freally

# View menu.
menu-view-hint = Contém comandos para manipular a exibição.
menu-view-filters = Filtros
menu-view-preview = Prévia
menu-view-status-bar = Barra de status
menu-view-thumbs-xl = Miniaturas extragrandes
menu-view-thumbs-l = Miniaturas grandes
menu-view-thumbs-m = Miniaturas médias
menu-view-details = Detalhes
menu-view-window-size = Tamanho da janela
menu-view-window-size-hint = Contém comandos para ajustar o tamanho da janela.
menu-view-window-small = Pequena
menu-view-window-medium = Média
menu-view-window-large = Grande
menu-view-window-auto = Ajuste automático
menu-view-zoom = Zoom
menu-view-zoom-hint = Contém comandos para ajustar o tamanho da fonte e dos ícones.
menu-view-zoom-in = Mais zoom
menu-view-zoom-out = Menos zoom
menu-view-zoom-reset = Redefinir
menu-view-sort-by = Ordenar por
menu-view-sort-by-hint = Contém comandos para ordenar a lista de resultados.
menu-view-sort-name = Nome
menu-view-sort-path = Caminho
menu-view-sort-size = Tamanho
menu-view-sort-ext = Extensão
menu-view-sort-type = Tipo
menu-view-sort-modified = Data de modificação
menu-view-sort-created = Data de criação
menu-view-sort-accessed = Data de acesso
menu-view-sort-attributes = Atributos
menu-view-sort-recently-changed = Data de alteração recente
menu-view-sort-run-count = Contagem de execuções
menu-view-sort-run-date = Data de execução
menu-view-sort-file-list-filename = Nome do arquivo da lista
menu-view-sort-lufs = LUFS
menu-view-sort-length = Duração
menu-view-sort-similarity = Pontuação de similaridade
menu-view-sort-asc = Crescente
menu-view-sort-desc = Decrescente
menu-view-go-to = Ir para
menu-view-refresh = Atualizar
menu-view-theme = Tema
menu-view-theme-hint = Alterne entre os temas do sistema, claro ou escuro.
menu-view-lenses = Lentes
menu-view-lenses-hint = Alternar a visibilidade de cada lente na lista de resultados.
menu-view-on-top = No topo
menu-view-on-top-hint = Contém comandos para manter esta janela sobre as outras.
menu-view-on-top-never = Nunca
menu-view-on-top-always = Sempre
menu-view-on-top-while-searching = Durante a pesquisa

# Search menu.
menu-search-hint = Contém opções de pesquisa.
menu-search-match-case = Diferenciar maiúsculas de minúsculas
menu-search-match-whole-word = Coincidir palavra inteira
menu-search-match-path = Coincidir caminho
menu-search-match-diacritics = Coincidir acentos
menu-search-enable-regex = Ativar regex
menu-search-advanced = Pesquisa avançada…
menu-search-add-to-filters = Adicionar aos filtros…
menu-search-organize-filters = Organizar filtros…
menu-search-filter-everything = Tudo
menu-search-filter-archive = Compactado (arquivo)
menu-search-filter-folder = Pasta
menu-search-filter-custom = Filtro personalizado…

# Bookmarks menu.
menu-bookmarks-hint = Contém comandos para trabalhar com favoritos.
menu-bookmarks-add = Adicionar aos favoritos
menu-bookmarks-organize = Organizar favoritos…

# Tools menu.
menu-tools-hint = Contém comandos de ferramentas.
menu-tools-connect = Conectar ao servidor FTP…
menu-tools-disconnect = Desconectar do servidor FTP
menu-tools-file-list-editor = Editor de listas de arquivos…
menu-tools-index-maintenance = Manutenção do índice
menu-tools-index-maintenance-hint = Ferramentas de manutenção do índice.
menu-tools-verify-index = Verificar índice…
menu-tools-compact-index = Compactar índice…
menu-tools-rebuild-index = Forçar reconstrução do índice…
menu-tools-custom-extractor = Gerenciador de extratores personalizados…
menu-tools-custom-extractor-hint = Gerencie extratores personalizados isolados em sandbox Wasm.
menu-tools-options = Opções…

# Help menu.
menu-help-hint = Contém comandos de ajuda.
menu-help-help = Ajuda do Freally
menu-help-search-syntax = Sintaxe de pesquisa
menu-help-regex-syntax = Sintaxe de regex
menu-help-audio-ref = Referência de modificadores de áudio
menu-help-similarity-ref = Referência de modificadores de similaridade
menu-help-cli-options = Opções de linha de comando
menu-help-website = Site do Freally
menu-help-check-updates = Verificar atualizações…
menu-help-sponsor = Patrocinar / doar
menu-help-about = Sobre o Freally…

# Result column headers (short forms used in the table header row).
column-name = Nome
column-path = Caminho
column-size = Tamanho
column-modified = Modificado
column-type = Tipo
column-ext = Ext
column-sort-by = Ordenar por { $name }
column-resize = Redimensionar coluna { $name }

# Section subtitle bars used inside multiple settings panels.
section-behavior = Comportamento
section-rendering = Renderização
section-status-bar = Barra de status
section-display-format = Formato de exibição
section-loading-priority = Prioridade de carregamento
section-compatibility = Compatibilidade
section-storage = Armazenamento
section-index-fields = Campos do índice
section-maintenance = Manutenção
section-logging = Registro em log
section-tools = Ferramentas
section-privacy = Privacidade
section-auto-update = Atualização automática (+)
section-bind = Vincular
section-lens = Lente
section-budgets = Orçamentos
section-other = Outros
section-per-format-mode = Modo por formato
section-loudness = Volume
section-tuning = Ajuste (+)
section-minhash-lsh = Parâmetros MinHash + LSH (+)
section-top-level = Nível superior
section-file-globs = Globs de arquivos
section-file-list-settings = Configurações da lista de arquivos selecionada
section-editor-format = Editor + formato (E + +)
section-api-server = Servidor de API (E adaptado)
section-freally-extras = Extras do Freally (+)
section-freally-additions = Adições do Freally (+)
section-freally-extensions = Extensões do Freally (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = Usar último valor
opt-use-last-value-default = Usar último valor (padrão)
opt-low = Baixa
opt-normal-default = Normal (padrão)
opt-high = Alta
opt-disabled = Desativado
opt-off = Desligado
opt-on-battery = Quando na bateria
opt-always = Sempre
opt-clamp-default = Limitar (padrão)
opt-wrap = Quebrar
opt-none = Nenhum
opt-strict-refuse = Estrito (recusar consultas em caso de corrupção)
opt-lenient-warn = Tolerante (avisar mas consultar)
opt-system-default = Padrão do sistema
opt-drag-select = Seleção por arraste
opt-auto-binary = Automático (binário)
opt-auto-decimal = Automático (decimal)

# Unit suffixes shown next to number inputs.
unit-days = dias
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = Ávido
opt-lazy-default = Preguiçoso (padrão)
opt-on = Ligado
opt-on-default = Ligado (padrão)
opt-all = Todos
opt-weekly = Semanal
opt-monthly = Mensal
opt-name-asc = Nome cresc.
opt-name-desc = Nome decresc.
opt-size-asc = Tamanho cresc.
opt-size-desc = Tamanho decresc.
opt-modified-asc = Data de modificação cresc.
opt-modified-desc = Data de modificação decresc.
opt-compact = Compacta
opt-comfortable = Confortável
opt-details = Detalhes
opt-thumbnails = Miniaturas
opt-local-db-default = Banco de dados local (padrão)
opt-file-list = Lista de arquivos
opt-https-endpoint = Endpoint da API HTTPS
opt-right-default = Direita (padrão)
opt-bottom = Inferior
opt-or-and-default = OR > AND (padrão)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (padrão)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = Broadcast film (-23)
opt-true-peak = Pico real (sobreamostragem 4×, padrão)
opt-sample-peak = Pico de amostra
opt-auto-per-doc = Automático (por documento)
opt-log-error = Erro
opt-log-warn = Aviso
opt-log-info-default = Info (padrão)
opt-log-debug = Depuração
opt-log-trace = Rastreamento
