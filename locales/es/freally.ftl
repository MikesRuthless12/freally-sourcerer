# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = Una búsqueda. Todas las fuentes. Todos los sistemas.
window-title = Freally Sourcerer
search-placeholder = Buscar…
about-version = Versión { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Listo
status-indexed = Indexado ({ $count } archivos)
status-indexing = Indexando… { $done }/{ $total }
status-paused = En pausa
status-error = Error
status-result-count-one = { $count } resultado
status-result-count-many = { $count } resultados
status-selection = · { $count } seleccionados
status-selection-size = Seleccionado: { $size }
status-query-timing = Consulta: { $ms } ms
status-endpoint-local = BD local
status-endpoint-remote = API: { $name }

menu-file = Archivo
menu-edit = Editar
menu-view = Ver
menu-search = Buscar
menu-bookmarks = Marcadores
menu-tools = Herramientas
menu-help = Ayuda

theme-system = Sistema
theme-light = Claro
theme-dark = Oscuro

lens-filename = Nombre de archivo
lens-content = Contenido
lens-audio = Audio
lens-similarity = Similitud

parse-error-empty = Escribe una consulta para comenzar.
parse-error-unknown = Sintaxis no reconocida cerca de aquí.

action-open = Abrir
action-reveal = Mostrar en la carpeta
action-copy-path = Copiar ruta
action-copy-name = Copiar nombre
action-delete = Eliminar

quick-filter-audio = Audio
quick-filter-video = Vídeo
quick-filter-image = Imagen
quick-filter-document = Documento
quick-filter-executable = Ejecutable
quick-filter-archive = Archivo comprimido

wizard-title = Te damos la bienvenida a Freally
wizard-step-roots = Elige qué indexar
wizard-step-hotkey = Elige una tecla de acceso global
wizard-step-locale = Elige tu idioma
wizard-step-theme = Elige un tema
wizard-finish = Finalizar

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Opciones
settings-search-placeholder = Buscar opciones…
settings-restore-defaults = Restablecer valores predeterminados
settings-ok = Aceptar
settings-cancel = Cancelar
settings-apply = Aplicar

# Tree nav groups (PRD §8.1.1).
settings-group-general = General
settings-group-indexes = Índices
settings-group-lenses = Lentes
settings-group-network = Red

# Tree nav leaves.
settings-node-ui = Interfaz
settings-node-home = Inicio
settings-node-search = Búsqueda
settings-node-results = Resultados
settings-node-view = Vista
settings-node-context-menu = Menú contextual
settings-node-fonts-colors = Fuentes y colores
settings-node-keyboard = Teclado
settings-node-history = Historial
settings-node-indexes-top = (nivel superior)
settings-node-volumes = Volúmenes
settings-node-folders = Carpetas
settings-node-file-lists = Listas de archivos
settings-node-exclude = Excluir
settings-node-https-server = Servidor HTTP / HTTPS
settings-node-etp-api = API ETP / FTP
settings-node-privacy = Privacidad y actualizaciones
settings-node-logs = Registros y depuración
settings-node-backup = Copia de seguridad, exportación y restablecimiento
settings-node-locale = Configuración regional
settings-node-about = Acerca de

# §8.2 General → UI.
settings-ui-theme = Tema
settings-ui-run-bg = Ejecutar en segundo plano
settings-ui-show-tray = Mostrar icono en la bandeja / barra de menús
settings-ui-single-click-tray = Un clic en la bandeja / barra de menús
settings-ui-new-window-from-tray = Abrir una ventana nueva desde el icono de la bandeja
settings-ui-new-window-on-launch = Abrir una ventana nueva al iniciar Freally
settings-ui-search-as-you-type = Buscar mientras escribes
settings-ui-select-on-mouse-click = Seleccionar la búsqueda al hacer clic
settings-ui-focus-on-activate = Enfocar la búsqueda al activar
settings-ui-full-row-select = Seleccionar la fila completa
settings-ui-single-click-open = Abrir con un solo clic
settings-ui-underline-titles = Subrayar los títulos de los iconos
settings-ui-row-density = Densidad de resultados
settings-ui-row-density-compact = Compacta (32 px)
settings-ui-row-density-comfortable = Cómoda (44 px)
settings-ui-show-timing-badges = Mostrar distintivos de tiempo por lente
settings-ui-anim-crossfade = Fundido animado al cambiar de tema

# §8.3 General → Home.
settings-home-match-case = Coincidir mayúsculas y minúsculas
settings-home-match-whole-word = Coincidir palabra completa
settings-home-match-path = Coincidir ruta
settings-home-match-diacritics = Coincidir signos diacríticos
settings-home-match-regex = Coincidir expresión regular
settings-home-search = Búsqueda (consulta predeterminada personalizada)
settings-home-filter = Filtro
settings-home-sort = Ordenar
settings-home-view = Vista
settings-home-index = Índice
settings-home-default-lens-visibility = Visibilidad predeterminada de las lentes
settings-home-default-lens-result-limits = Límites predeterminados de resultados por lente

# §8.4 General → Search.
settings-search-fast-ascii = Búsqueda ASCII rápida
settings-search-mp-sep = Coincidir la ruta cuando un término contiene un separador de ruta
settings-search-mw-fn = Coincidir el nombre de archivo completo al usar comodines
settings-search-lit-ops = Permitir operadores literales
settings-search-paren = Permitir agrupación con paréntesis
settings-search-env = Expandir variables de entorno
settings-search-fwd-slash = Sustituir las barras diagonales por barras invertidas
settings-search-precedence = Precedencia de operadores
settings-search-strict-everything = Modo de sintaxis estricta de Everything
settings-search-auto-regex = Detectar expresiones regulares automáticamente
settings-search-mod-comp = Completado de modificadores
settings-search-parse-tree = Mostrar el árbol de análisis al pasar el cursor

# §8.5 General → Results.
settings-results-hide-empty = Ocultar los resultados cuando la búsqueda está vacía
settings-results-clear-on-search = Borrar la selección al buscar
settings-results-close-on-execute = Cerrar la ventana al ejecutar
settings-results-dbl-path = Abrir la ruta con doble clic en la columna de ruta
settings-results-auto-scroll = Desplazar la vista automáticamente
settings-results-dquote-copy = Copiar entre comillas dobles como ruta
settings-results-no-ext-rename = No seleccionar la extensión al cambiar el nombre
settings-results-sort-date-desc = Ordenar primero por fecha descendente
settings-results-sort-size-desc = Ordenar primero por tamaño descendente
settings-results-list-focus = Foco en la lista de resultados
settings-results-icon-prio = Prioridad de carga de iconos
settings-results-thumb-prio = Prioridad de carga de miniaturas
settings-results-ext-prio = Prioridad de carga de información ampliada
settings-results-group-by-lens = Agrupar los resultados por lente
settings-results-snippet-inline = Mostrar la vista previa del fragmento en línea

# §8.6 General → View.
settings-view-double-buffer = Doble búfer
settings-view-alt-rows = Alternar el color de las filas
settings-view-row-mouseover = Resaltar la fila al pasar el cursor
settings-view-highlight-terms = Resaltar los términos de búsqueda
settings-view-status-show-selected = Mostrar el elemento seleccionado en la barra de estado
settings-view-rc-with-sel = Mostrar el recuento de resultados junto al de la selección
settings-view-status-show-size = Mostrar el tamaño en la barra de estado
settings-view-tooltips = Mostrar información sobre herramientas
settings-view-update-on-scroll = Actualizar la vista de inmediato tras desplazarse
settings-view-size-format = Formato de tamaño
settings-view-selection-rect = Rectángulo de selección
settings-view-audio-badges = Mostrar distintivos de LUFS / códec / duración en las filas de audio
settings-view-similarity-score = Mostrar la puntuación de similitud MinHash en las filas de similitud
settings-view-preview-pane = Panel de vista previa

# §8.7 General → Context Menu.
settings-context-menu-visibility = Visibilidad
settings-context-menu-show = Mostrar
settings-context-menu-shift = Mostrar solo al mantener Mayús
settings-context-menu-hide = Ocultar
settings-context-menu-command = Macro de comando
settings-context-menu-open-folders = Abrir (carpetas)
settings-context-menu-open-files = Abrir (archivos)
settings-context-menu-open-path = Abrir ruta
settings-context-menu-explore = Explorar
settings-context-menu-explore-path = Explorar ruta
settings-context-menu-copy-name = Copiar el nombre al portapapeles
settings-context-menu-copy-path = Copiar la ruta al portapapeles
settings-context-menu-copy-full-name = Copiar el nombre completo al portapapeles
settings-context-menu-reveal = Mostrar en Freally
settings-context-menu-send-to = Enviar a Freally (ruta)

# §8.8 General → Fonts & Colors.
settings-fc-font = Fuente
settings-fc-size = Tamaño
settings-fc-state-normal = Normal
settings-fc-state-highlighted = Resaltado
settings-fc-state-current-sort = Orden actual
settings-fc-state-current-sort-h = Orden actual (resaltado)
settings-fc-state-selected = Seleccionado
settings-fc-state-selected-h = Seleccionado (resaltado)
settings-fc-state-inactive-selected = Seleccionado inactivo
settings-fc-state-inactive-selected-h = Seleccionado inactivo (resaltado)
settings-fc-foreground = Primer plano
settings-fc-background = Fondo
settings-fc-bold = Negrita
settings-fc-italic = Cursiva
settings-fc-default = Predeterminado
settings-fc-per-lens-accent = Color de acento por lente
settings-fc-theme-inherit = Invertir colores personalizados al cambiar de tema

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Tecla de acceso global
settings-keyboard-new-window = Tecla de acceso para ventana nueva
settings-keyboard-show-window = Tecla de acceso para mostrar la ventana
settings-keyboard-toggle-window = Tecla de acceso para alternar la ventana
settings-keyboard-show-commands = Mostrar comandos que contengan
settings-keyboard-add-chord = + Añadir combinación
settings-keyboard-remove-chord = Quitar

# §8.10 History.
settings-history-search-enable = Habilitar el historial de búsqueda
settings-history-search-keep = Conservar el historial de búsqueda durante { $days } días
settings-history-run-enable = Habilitar el historial de ejecución
settings-history-run-keep = Conservar el historial de ejecución durante { $days } días
settings-history-clear-now = Borrar ahora
settings-history-privacy-mode = Modo de privacidad
settings-history-per-lens = Historial por lente

# §8.11 Indexes (top-level).
settings-ix-database-location = Ubicación de la base de datos
settings-ix-multiuser = Nombre de archivo de la base de datos multiusuario
settings-ix-compress = Comprimir la base de datos
settings-ix-recent-changes = Indexar cambios recientes
settings-ix-file-size = Indexar el tamaño de archivo
settings-ix-fast-size-sort = Orden rápido por tamaño
settings-ix-folder-size = Indexar el tamaño de carpeta
settings-ix-fast-folder-size-sort = Orden rápido por tamaño de carpeta
settings-ix-date-created = Indexar la fecha de creación
settings-ix-fast-date-created = Orden rápido por fecha de creación
settings-ix-date-modified = Indexar la fecha de modificación
settings-ix-fast-date-modified = Orden rápido por fecha de modificación
settings-ix-date-accessed = Indexar la fecha de acceso
settings-ix-fast-date-accessed = Orden rápido por fecha de acceso
settings-ix-attributes = Indexar los atributos
settings-ix-fast-attributes = Orden rápido por atributos
settings-ix-fast-path-sort = Orden rápido por ruta
settings-ix-fast-extension-sort = Orden rápido por extensión
settings-ix-force-rebuild = Forzar reconstrucción
settings-ix-compact = Compactar el índice
settings-ix-verify = Verificar el índice
settings-ix-integrity-policy = Política de integridad del índice
settings-ix-memory-budget = Presupuesto de memoria para el indexador
settings-ix-throttle = Limitación de la indexación en segundo plano

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Incluir automáticamente los nuevos volúmenes fijos
settings-vol-auto-removable = Incluir automáticamente los nuevos volúmenes extraíbles
settings-vol-auto-remove-offline = Quitar automáticamente los volúmenes sin conexión
settings-vol-detected = Volúmenes detectados
settings-vol-include = Incluir en el índice
settings-vol-include-only = Incluir solo (glob/regex)
settings-vol-enable-usn = Habilitar el diario USN
settings-vol-enable-fsevents = Habilitar el flujo FSEvents
settings-vol-enable-inotify = Habilitar inotify (o fanotify si hay privilegios elevados)
settings-vol-buffer = Tamaño del búfer del diario (KB)
settings-vol-allocation-delta = Delta de asignación (KB)
settings-vol-load-recent = Cargar los cambios recientes del diario al iniciar
settings-vol-monitor = Supervisar cambios
settings-vol-recreate-journal = Recrear el diario
settings-vol-reset-stream = Restablecer el flujo FSEvents
settings-vol-upgrade-fanotify = Actualizar a fanotify (polkit)
settings-vol-remove = Quitar

# §8.13 Indexes → Folders.
settings-folders-watched = Carpetas supervisadas
settings-folders-add = Añadir…
settings-folders-rescan-now = Volver a escanear ahora
settings-folders-rescan-all = Volver a escanear todo ahora
settings-folders-monitor = Intentar supervisar cambios
settings-folders-buffer = Tamaño del búfer
settings-folders-rescan-on-full = Volver a escanear cuando el búfer se llene

# §8.14 Indexes → File Lists.
settings-flists-add = Añadir…
settings-flists-monitor = Supervisar cambios
settings-flists-editor = Editor de listas de archivos…
settings-flists-format = Formato de la lista de archivos
settings-flists-format-text = Texto (una ruta por línea)
settings-flists-format-json = JSON (con metadatos)
settings-flists-format-srcb = Paquete de Freally (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Excluir los archivos y carpetas ocultos
settings-exclude-system = Excluir los archivos y carpetas del sistema
settings-exclude-list-en = Habilitar la lista de exclusión
settings-exclude-folders = Excluir carpetas
settings-exclude-include-only-files = Incluir solo archivos (glob)
settings-exclude-files = Excluir archivos (glob)
settings-exclude-os-recommended = Aplicar las exclusiones recomendadas por el sistema
settings-exclude-by-class = Excluir por clase de extensión

# §8.16 Lenses → Filename.
settings-lf-trigram = Agresividad del prefiltro de trigramas
settings-lf-suffix-mem = Presupuesto de memoria del arreglo de sufijos
settings-lf-wildcard-limit = Límite de expansión de comodines
settings-lf-regex-timeout = Tiempo de espera de la expresión regular

# §8.17 Lenses → Content.
settings-lc-enable = Habilitar la lente de contenido
settings-lc-time-budget = Presupuesto de tiempo por documento
settings-lc-mem-ceiling = Límite de memoria por documento
settings-lc-snippet-len = Longitud del fragmento
settings-lc-stop-words = Idioma de las palabras vacías
settings-lc-re-extract = Volver a extraer al cambiar la configuración
settings-lc-verify-blobs = Verificar las sumas de comprobación del texto extraído al leer

# §8.18 Lenses → Audio.
settings-la-enable = Habilitar la lente de audio
settings-la-lufs-ref = Estándar de referencia LUFS
settings-la-peak-compute = Calcular el pico mediante
settings-la-silence-thresh = Umbral de silencio
settings-la-re-extract-modify = Volver a extraer en el evento de modificación

# §8.19 Lenses → Similarity.
settings-ls-enable = Habilitar la lente de similitud
settings-ls-sig-size = Tamaño de la firma MinHash (k)
settings-ls-bands = Bandas LSH
settings-ls-recall = Umbral de exhaustividad
settings-ls-result-cap = Tope de resultados

# §8.20 Lenses → Custom.
settings-custom-registry = Registro
settings-custom-trust = Confianza
settings-custom-refresh-hashes = Actualizar los hashes

# §8.21-§8.22 Network.
settings-net-https-enable = Habilitar el servidor HTTPS
settings-net-bind = Enlazar a las interfaces
settings-net-port = Escuchar en el puerto
settings-net-force-https = Forzar HTTPS
settings-net-legacy-auth = Autenticación HTTP básica heredada
settings-net-token-regen = Regenerar el token
settings-net-api-enable = Habilitar el servidor de API
settings-net-legacy-ftp = Compatibilidad con FTP/ETP heredado sin cifrar

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Actualización automática
settings-privacy-prerelease = Canal de versiones preliminares
settings-privacy-network-policy = Política de llamadas de red

# §8.24 Logs & Debug.
settings-logs-level = Nivel de registro
settings-logs-location = Ubicación del archivo de registro
settings-logs-retention = Retención de registros
settings-logs-debug-overlay = Mostrar la superposición de depuración
settings-logs-open-folder = Abrir la carpeta de registros
settings-logs-export-bundle = Exportar el paquete de diagnóstico

# §8.25 Backup, Export, Reset.
settings-backup-export = Exportar la configuración
settings-backup-import = Importar la configuración
settings-backup-export-bookmarks = Exportar el paquete de marcadores
settings-backup-import-bookmarks = Importar el paquete de marcadores
settings-backup-reset-all = Restablecer toda la configuración a los valores predeterminados

# §8.26 Locale.
settings-locale-current = Configuración regional actual
settings-locale-rtl-preview = Vista previa de derecha a izquierda
settings-locale-date-format = Formato de fecha
settings-locale-number-format = Formato de número

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = Licencia
settings-about-credits = Créditos
settings-about-notices = Avisos de código abierto

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = Asistente de primera ejecución
wizard-step-of-total = Paso { $step } de { $total }
wizard-roots-hint = Añade las carpetas o volúmenes que quieres que Freally supervise. Puedes cambiarlo más adelante en la configuración de Índices.
wizard-browse = Examinar…
wizard-roots-placeholder = …o pega una ruta
wizard-roots-add = Añadir
wizard-roots-remove = Quitar
wizard-roots-empty = Aún no se ha configurado ninguna raíz.
wizard-locale-hint = Freally está disponible en 18 idiomas. Puedes cambiarlo más adelante.
wizard-theme-hint = El sistema sigue la configuración de apariencia de tu sistema operativo.
wizard-back = Atrás
wizard-next = Siguiente

# Status bar polish.
statusbar-hotkey-hint = Tecla de acceso: { $hotkey }
statusbar-cycle-theme = Alternar tema
statusbar-indexed-suffix = indexados

# Results / lenses.
lens-expand = Expandir la lente
lens-collapse = Contraer la lente
lens-no-matches = No hay coincidencias en esta lente.

# Preview pane.
preview-header = Vista previa
preview-loading = Cargando…
preview-select-file = Selecciona un archivo para ver la vista previa.
preview-unavailable = No hay vista previa disponible

# Bookmarks.
bookmarks-label = ★ Marcadores
bookmarks-empty-hint = Aún no hay marcadores. Pulsa Ctrl+D para guardar la consulta actual.
bookmarks-organize-title = Organizar marcadores
bookmarks-organize-empty = Aún no hay marcadores.
bookmarks-rename = Cambiar nombre
bookmarks-close = Cerrar

# Settings tree extras.
settings-group-history = Historial
settings-group-privacy = Privacidad y actualizaciones
settings-group-logs = Registros y depuración
settings-group-backup = Copia de seguridad, exportación y restablecimiento
settings-tree-custom-lens = Personalizada
settings-unsaved-changes = cambios sin guardar

# About dialog.
about-dialog-title = Freally
about-copyright = Copyright © 2026 Mike Weaver. Todos los derechos reservados.
about-close = Cerrar

# Connect endpoint dialog.
connect-ftp-title = Conectar al servidor FTP
connect-ftp-host = Host:
connect-ftp-port = Puerto:
connect-ftp-username = Usuario:
connect-ftp-password = Contraseña:
connect-ftp-link-type = Tipo de enlace:

# UI panel.
ui-hint = Tema, integración con la bandeja / barra de menús, búsqueda mientras escribes, densidad de filas. Paridad directa con voidtools-Everything más las funciones añadidas de Freally marcadas con (+).
ui-section-theme = Tema
ui-theme-system-default = Sistema (predeterminado)
ui-section-tray = Bandeja / barra de menús
ui-section-search-behavior = Comportamiento de búsqueda
ui-section-result-rows = Filas de resultados
ui-single-click-system-default = Configuración del sistema (predeterminado)
ui-single-click-always = Siempre con un solo clic
ui-single-click-always-double = Siempre con doble clic
ui-underline-always = Siempre
ui-underline-on-hover = Al pasar el cursor
ui-underline-never = Nunca

# Home panel.
home-hint = Valores predeterminados cargados al iniciar la aplicación: cada menú desplegable puede mantener «Usar el último valor» o fijar un valor concreto. La visibilidad de las lentes y los límites de resultados son funciones añadidas de Freally (+).
home-section-match = Valores predeterminados de coincidencia
home-section-search-sort = Valores predeterminados de búsqueda y orden
home-search-placeholder = Vacío de forma predeterminada
home-section-index = Origen del índice
home-file-list-path = Ruta de la lista de archivos
home-https-endpoint = URL del extremo de la API HTTPS
home-endpoint-token = Token (se muestra la huella)

# Backup panel.
backup-section-settings = Configuración (+)
backup-section-bookmarks = Marcadores + extractores personalizados (+)
backup-section-reset = Restablecer
backup-toast-exported = Configuración exportada a { $path }
backup-toast-export-failed = Error al exportar: { $error }
backup-toast-imported = Configuración importada
backup-toast-import-failed = Error al importar: { $error }
backup-toast-bookmarks-exported = Marcadores exportados
backup-toast-bookmarks-export-failed = Error al exportar los marcadores: { $error }
backup-toast-bookmarks-imported = Marcadores importados
backup-toast-bookmarks-import-failed = Error al importar los marcadores: { $error }
backup-confirm-reset = ¿Restablecer toda la configuración a los valores predeterminados? Esta acción no se puede deshacer (el cuadro de diálogo permanece abierto).
backup-toast-reset = Se ha restablecido toda la configuración

# Keyboard panel.
keyboard-section-global = Teclas de acceso globales
keyboard-placeholder-example = Super+Space
keyboard-section-commands = Comandos
keyboard-placeholder-command = id de comando (p. ej. file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = Historial de búsqueda
history-section-run = Historial de ejecución
history-section-privacy = Privacidad (+)
history-record-filename = Registrar el historial de la lente de nombre de archivo
history-record-content = Registrar el historial de la lente de contenido
history-record-audio = Registrar el historial de la lente de audio
history-record-similarity = Registrar el historial de la lente de similitud

# Locale panel.
locale-section-language = Idioma (+)
locale-section-time-date = Hora / fecha (+)
locale-date-os = Predeterminado del sistema
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = Personalizado
locale-date-custom-format = Formato personalizado
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = Números (+)
locale-number-os = Predeterminado del sistema
locale-number-custom = Personalizado
locale-thousands-sep = Separador de miles
locale-decimal-sep = Separador decimal

# Folders panel.
folders-hint = Carpetas supervisadas adicionales más allá de los volúmenes predeterminados.
folders-list-title = Carpetas supervisadas
folders-empty = Aún no se ha añadido ninguna carpeta.
folders-remove = Quitar
folders-section-title-dynamic = Configuración de { $path }
folders-section-schedule = Programación de reescaneo
folders-schedule-daily = Cada día a las HH:MM
folders-schedule-hours = Cada N horas
folders-schedule-never = Nunca
folders-hour = Hora
folders-minute = Minuto
folders-hours = Horas
folders-id-label = ID de carpeta (solo lectura)
folders-select-prompt = Selecciona una carpeta para configurarla.
folders-section-extras = Extras de Freally (+)
folders-extras-note = El reescaneo al reanudar tras la suspensión está habilitado de forma predeterminada en esta versión; la opción se sumará a los controles a nivel de carpeta en la fase de pulido de la Phase 13.

# Volumes panel.
volumes-hint = Equivalente multiplataforma de los paneles NTFS / ReFS de voidtools-Everything. Detecta automáticamente NTFS / ReFS / exFAT / FAT32 (Win), APFS / HFS+ (macOS), ext4 / Btrfs / ZFS / XFS / F2FS (Linux).
volumes-section-auto-include = Inclusión automática
volumes-list-title = Volúmenes detectados
volumes-detecting = Detectando…
volumes-empty = No se han detectado volúmenes.
volumes-select-prompt = Selecciona un volumen para configurarlo.

# About panel polish.
about-section-version = Versión (+)
about-section-license = Licencia (+)
about-license-text = Mike Weaver — Todos los derechos reservados. Este es software propietario.
about-license-spdx = SPDX: { $spdx }
about-section-credits = Créditos (+)
about-credits-inspired = Inspirado en Everything de voidtools.
about-credits-voidtools = voidtools.com
about-credits-repo = Repositorio del proyecto

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Contiene comandos para trabajar con Freally.
menu-file-new-window = Nueva ventana de búsqueda
menu-file-open-list = Abrir lista de archivos…
menu-file-close-list = Cerrar lista de archivos
menu-file-close = Cerrar
menu-file-export-results = Exportar resultados…
menu-file-export-bundle = Exportar paquete de índice…
menu-file-exit = Salir

# Edit menu.
menu-edit-hint = Contiene comandos para editar los resultados de búsqueda.
menu-edit-cut = Cortar
menu-edit-copy = Copiar
menu-edit-paste = Pegar
menu-edit-copy-to-folder = Copiar a la carpeta…
menu-edit-move-to-folder = Mover a la carpeta…
menu-edit-select-all = Seleccionar todo
menu-edit-invert-selection = Invertir la selección
menu-edit-advanced = Avanzado
menu-edit-copy-full-name = Copiar el nombre completo
menu-edit-copy-path = Copiar la ruta
menu-edit-copy-filename = Copiar el nombre de archivo
menu-edit-copy-as-json = Copiar como JSON
menu-edit-copy-with-metadata = Copiar con metadatos
menu-edit-copy-as-bundle-ref = Copiar como referencia de paquete de Freally

# View menu.
menu-view-hint = Contiene comandos para manipular la vista.
menu-view-filters = Filtros
menu-view-preview = Vista previa
menu-view-status-bar = Barra de estado
menu-view-thumbs-xl = Miniaturas extragrandes
menu-view-thumbs-l = Miniaturas grandes
menu-view-thumbs-m = Miniaturas medianas
menu-view-details = Detalles
menu-view-window-size = Tamaño de la ventana
menu-view-window-size-hint = Contiene comandos para ajustar el tamaño de la ventana.
menu-view-window-small = Pequeña
menu-view-window-medium = Mediana
menu-view-window-large = Grande
menu-view-window-auto = Ajuste automático
menu-view-zoom = Zoom
menu-view-zoom-hint = Contiene comandos para ajustar el tamaño de la fuente y los iconos.
menu-view-zoom-in = Acercar
menu-view-zoom-out = Alejar
menu-view-zoom-reset = Restablecer
menu-view-sort-by = Ordenar por
menu-view-sort-by-hint = Contiene comandos para ordenar la lista de resultados.
menu-view-sort-name = Nombre
menu-view-sort-path = Ruta
menu-view-sort-size = Tamaño
menu-view-sort-ext = Extensión
menu-view-sort-type = Tipo
menu-view-sort-modified = Fecha de modificación
menu-view-sort-created = Fecha de creación
menu-view-sort-accessed = Fecha de acceso
menu-view-sort-attributes = Atributos
menu-view-sort-recently-changed = Fecha de cambio reciente
menu-view-sort-run-count = Recuento de ejecuciones
menu-view-sort-run-date = Fecha de ejecución
menu-view-sort-file-list-filename = Nombre de archivo de la lista de archivos
menu-view-sort-lufs = LUFS
menu-view-sort-length = Duración
menu-view-sort-similarity = Puntuación de similitud
menu-view-sort-asc = Ascendente
menu-view-sort-desc = Descendente
menu-view-go-to = Ir a
menu-view-refresh = Actualizar
menu-view-theme = Tema
menu-view-theme-hint = Cambia entre los temas del sistema, claro u oscuro.
menu-view-lenses = Lentes
menu-view-lenses-hint = Activa o desactiva la visibilidad de cada lente en la lista de resultados.
menu-view-on-top = Encima
menu-view-on-top-hint = Contiene comandos para mantener esta ventana encima de las demás.
menu-view-on-top-never = Nunca
menu-view-on-top-always = Siempre
menu-view-on-top-while-searching = Al buscar

# Search menu.
menu-search-hint = Contiene los conmutadores de búsqueda.
menu-search-match-case = Coincidir mayúsculas y minúsculas
menu-search-match-whole-word = Coincidir palabra completa
menu-search-match-path = Coincidir ruta
menu-search-match-diacritics = Coincidir signos diacríticos
menu-search-enable-regex = Habilitar expresiones regulares
menu-search-advanced = Búsqueda avanzada…
menu-search-add-to-filters = Añadir a los filtros…
menu-search-organize-filters = Organizar los filtros…
menu-search-filter-everything = Everything
menu-search-filter-archive = Comprimido (archivo)
menu-search-filter-folder = Carpeta
menu-search-filter-custom = Filtro personalizado…

# Bookmarks menu.
menu-bookmarks-hint = Contiene comandos para trabajar con marcadores.
menu-bookmarks-add = Añadir a marcadores
menu-bookmarks-organize = Organizar marcadores…

# Tools menu.
menu-tools-hint = Contiene comandos de herramientas.
menu-tools-connect = Conectar al servidor FTP…
menu-tools-disconnect = Desconectar del servidor FTP
menu-tools-file-list-editor = Editor de listas de archivos…
menu-tools-index-maintenance = Mantenimiento del índice
menu-tools-index-maintenance-hint = Herramientas de mantenimiento del índice.
menu-tools-verify-index = Verificar el índice…
menu-tools-compact-index = Compactar el índice…
menu-tools-rebuild-index = Forzar la reconstrucción del índice…
menu-tools-custom-extractor = Administrador de extractores personalizados…
menu-tools-custom-extractor-hint = Administra los extractores personalizados aislados con Wasm.
menu-tools-options = Opciones…

# Help menu.
menu-help-hint = Contiene comandos de ayuda.
menu-help-help = Ayuda de Freally
menu-help-search-syntax = Sintaxis de búsqueda
menu-help-regex-syntax = Sintaxis de expresiones regulares
menu-help-audio-ref = Referencia de modificadores de audio
menu-help-similarity-ref = Referencia de modificadores de similitud
menu-help-cli-options = Opciones de línea de comandos
menu-help-website = Sitio web de Freally
menu-help-check-updates = Buscar actualizaciones…
menu-help-sponsor = Patrocinar / Donar
menu-help-about = Acerca de Freally…

# Result column headers (short forms used in the table header row).
column-name = Nombre
column-path = Ruta
column-size = Tamaño
column-modified = Modificado
column-type = Tipo
column-ext = Ext
column-sort-by = Ordenar por { $name }
column-resize = Cambiar el tamaño de la columna { $name }

# Section subtitle bars used inside multiple settings panels.
section-behavior = Comportamiento
section-rendering = Representación
section-status-bar = Barra de estado
section-display-format = Formato de visualización
section-loading-priority = Prioridad de carga
section-compatibility = Compatibilidad
section-storage = Almacenamiento
section-index-fields = Campos del índice
section-maintenance = Mantenimiento
section-logging = Registro
section-tools = Herramientas
section-privacy = Privacidad
section-auto-update = Actualización automática (+)
section-bind = Enlace
section-lens = Lente
section-budgets = Presupuestos
section-other = Otros
section-per-format-mode = Modo por formato
section-loudness = Sonoridad
section-tuning = Ajuste (+)
section-minhash-lsh = Parámetros de MinHash + LSH (+)
section-top-level = Nivel superior
section-file-globs = Globs de archivos
section-file-list-settings = Configuración de la lista de archivos seleccionada
section-editor-format = Editor + formato (E + +)
section-api-server = Servidor de API (E adaptado)
section-freally-extras = Extras de Freally (+)
section-freally-additions = Funciones añadidas de Freally (+)
section-freally-extensions = Extensiones de Freally (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = Usar el último valor
opt-use-last-value-default = Usar el último valor (predeterminado)
opt-low = Baja
opt-normal-default = Normal (predeterminado)
opt-high = Alta
opt-disabled = Deshabilitado
opt-off = Desactivado
opt-on-battery = Con batería
opt-always = Siempre
opt-clamp-default = Fijar (predeterminado)
opt-wrap = Ajustar
opt-none = Ninguno
opt-strict-refuse = Estricto (rechazar consultas si hay corrupción)
opt-lenient-warn = Flexible (avisar pero consultar)
opt-system-default = Predeterminado del sistema
opt-drag-select = Selección por arrastre
opt-auto-binary = Automático (binario)
opt-auto-decimal = Automático (decimal)

# Unit suffixes shown next to number inputs.
unit-days = días
unit-b = B
unit-kb = KB
unit-mb = MB
unit-gb = GB
unit-tb = TB

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = Inmediato
opt-lazy-default = Diferido (predeterminado)
opt-on = Activado
opt-on-default = Activado (predeterminado)
opt-all = Todos
opt-weekly = Semanal
opt-monthly = Mensual
opt-name-asc = Nombre asc.
opt-name-desc = Nombre desc.
opt-size-asc = Tamaño asc.
opt-size-desc = Tamaño desc.
opt-modified-asc = Fecha de modificación asc.
opt-modified-desc = Fecha de modificación desc.
opt-compact = Compacta
opt-comfortable = Cómoda
opt-details = Detalles
opt-thumbnails = Miniaturas
opt-local-db-default = Base de datos local (predeterminado)
opt-file-list = Lista de archivos
opt-https-endpoint = Extremo de la API HTTPS
opt-right-default = Derecha (predeterminado)
opt-bottom = Abajo
opt-or-and-default = OR > AND (predeterminado)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (predeterminado)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = Cine de difusión (-23)
opt-true-peak = Pico real (sobremuestreo 4×, predeterminado)
opt-sample-peak = Pico de muestra
opt-auto-per-doc = Automático (por documento)
opt-log-error = Error
opt-log-warn = Advertencia
opt-log-info-default = Información (predeterminado)
opt-log-debug = Depuración
opt-log-trace = Traza
