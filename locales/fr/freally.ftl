# Freally — English (source locale).
# Phase 0 surface; new keys land per-phase and propagate to all 18 locales.

app-name = Freally Sourcerer
tagline = Une recherche. Toutes les sources. Tous les OS.
window-title = Freally Sourcerer
search-placeholder = Rechercher…
about-version = Version { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = Prêt
status-indexed = Indexé ({ $count } fichiers)
status-indexing = Indexation… { $done }/{ $total }
status-paused = En pause
status-error = Erreur
status-result-count-one = { $count } résultat
status-result-count-many = { $count } résultats
status-selection = · { $count } sélectionné(s)
status-selection-size = Sélection : { $size }
status-query-timing = Requête : { $ms } ms
status-endpoint-local = Base locale
status-endpoint-remote = API : { $name }

menu-file = Fichier
menu-edit = Édition
menu-view = Affichage
menu-search = Recherche
menu-bookmarks = Favoris
menu-tools = Outils
menu-help = Aide

theme-system = Système
theme-light = Clair
theme-dark = Sombre

lens-filename = Nom de fichier
lens-content = Contenu
lens-audio = Audio
lens-similarity = Similarité

parse-error-empty = Saisissez une requête pour commencer.
parse-error-unknown = Syntaxe non reconnue à cet endroit.

action-open = Ouvrir
action-reveal = Afficher dans le dossier
action-copy-path = Copier le chemin
action-copy-name = Copier le nom
action-delete = Supprimer

quick-filter-audio = Audio
quick-filter-video = Vidéo
quick-filter-image = Image
quick-filter-document = Document
quick-filter-executable = Exécutable
quick-filter-archive = Archive

wizard-title = Bienvenue dans Freally
wizard-step-roots = Choisissez ce qu'il faut indexer
wizard-step-hotkey = Choisissez un raccourci global
wizard-step-locale = Choisissez votre langue
wizard-step-theme = Choisissez un thème
wizard-finish = Terminer

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = Options
settings-search-placeholder = Rechercher des options…
settings-restore-defaults = Rétablir les valeurs par défaut
settings-ok = OK
settings-cancel = Annuler
settings-apply = Appliquer

# Tree nav groups (PRD §8.1.1).
settings-group-general = Général
settings-group-indexes = Index
settings-group-lenses = Filtres
settings-group-network = Réseau

# Tree nav leaves.
settings-node-ui = Interface
settings-node-home = Accueil
settings-node-search = Recherche
settings-node-results = Résultats
settings-node-view = Affichage
settings-node-context-menu = Menu contextuel
settings-node-fonts-colors = Polices et couleurs
settings-node-keyboard = Clavier
settings-node-history = Historique
settings-node-indexes-top = (niveau supérieur)
settings-node-volumes = Volumes
settings-node-folders = Dossiers
settings-node-file-lists = Listes de fichiers
settings-node-exclude = Exclure
settings-node-https-server = Serveur HTTP / HTTPS
settings-node-etp-api = API ETP / FTP
settings-node-privacy = Confidentialité et mises à jour
settings-node-logs = Journaux et débogage
settings-node-backup = Sauvegarde, export, réinitialisation
settings-node-locale = Langue
settings-node-about = À propos

# §8.2 General → UI.
settings-ui-theme = Thème
settings-ui-run-bg = Exécuter en arrière-plan
settings-ui-show-tray = Afficher l'icône dans la zone de notification / barre de menus
settings-ui-single-click-tray = Simple clic dans la zone de notification / barre de menus
settings-ui-new-window-from-tray = Ouvrir une nouvelle fenêtre depuis l'icône de la zone de notification
settings-ui-new-window-on-launch = Ouvrir une nouvelle fenêtre au lancement de Freally
settings-ui-search-as-you-type = Rechercher à la frappe
settings-ui-select-on-mouse-click = Sélectionner la recherche au clic de souris
settings-ui-focus-on-activate = Activer le focus sur la recherche à l'ouverture
settings-ui-full-row-select = Sélection de la ligne entière
settings-ui-single-click-open = Ouverture en simple clic
settings-ui-underline-titles = Souligner les titres des icônes
settings-ui-row-density = Densité des résultats
settings-ui-row-density-compact = Compacte (32 px)
settings-ui-row-density-comfortable = Confortable (44 px)
settings-ui-show-timing-badges = Afficher les badges de durée par filtre
settings-ui-anim-crossfade = Fondu enchaîné animé du thème

# §8.3 General → Home.
settings-home-match-case = Respecter la casse
settings-home-match-whole-word = Mot entier
settings-home-match-path = Rechercher dans le chemin
settings-home-match-diacritics = Respecter les diacritiques
settings-home-match-regex = Expression régulière
settings-home-search = Recherche (requête par défaut personnalisée)
settings-home-filter = Filtre
settings-home-sort = Tri
settings-home-view = Affichage
settings-home-index = Index
settings-home-default-lens-visibility = Visibilité des filtres par défaut
settings-home-default-lens-result-limits = Limites de résultats des filtres par défaut

# §8.4 General → Search.
settings-search-fast-ascii = Recherche ASCII rapide
settings-search-mp-sep = Rechercher dans le chemin lorsqu'un terme contient un séparateur de chemin
settings-search-mw-fn = Rechercher le nom de fichier entier avec les caractères génériques
settings-search-lit-ops = Autoriser les opérateurs littéraux
settings-search-paren = Autoriser le regroupement par parenthèses
settings-search-env = Développer les variables d'environnement
settings-search-fwd-slash = Remplacer les barres obliques par des barres obliques inverses
settings-search-precedence = Priorité des opérateurs
settings-search-strict-everything = Mode syntaxe stricte Everything
settings-search-auto-regex = Détection automatique des expressions régulières
settings-search-mod-comp = Complétion des modificateurs
settings-search-parse-tree = Afficher l'arbre d'analyse au survol

# §8.5 General → Results.
settings-results-hide-empty = Masquer les résultats lorsque la recherche est vide
settings-results-clear-on-search = Effacer la sélection lors de la recherche
settings-results-close-on-execute = Fermer la fenêtre à l'exécution
settings-results-dbl-path = Ouvrir le chemin par double clic dans la colonne du chemin
settings-results-auto-scroll = Faire défiler automatiquement la vue
settings-results-dquote-copy = Copier entre guillemets comme chemin
settings-results-no-ext-rename = Ne pas sélectionner l'extension lors du renommage
settings-results-sort-date-desc = Trier d'abord par date décroissante
settings-results-sort-size-desc = Trier d'abord par taille décroissante
settings-results-list-focus = Focus sur la liste des résultats
settings-results-icon-prio = Priorité de chargement des icônes
settings-results-thumb-prio = Priorité de chargement des miniatures
settings-results-ext-prio = Priorité de chargement des informations étendues
settings-results-group-by-lens = Grouper les résultats par filtre
settings-results-snippet-inline = Afficher l'aperçu de l'extrait en ligne

# §8.6 General → View.
settings-view-double-buffer = Double mise en mémoire tampon
settings-view-alt-rows = Couleur de ligne alternée
settings-view-row-mouseover = Afficher le survol des lignes
settings-view-highlight-terms = Afficher les termes de recherche en surbrillance
settings-view-status-show-selected = Afficher l'élément sélectionné dans la barre d'état
settings-view-rc-with-sel = Afficher le nombre de résultats avec le nombre d'éléments sélectionnés
settings-view-status-show-size = Afficher la taille dans la barre d'état
settings-view-tooltips = Afficher les info-bulles
settings-view-update-on-scroll = Mettre à jour l'affichage immédiatement après le défilement
settings-view-size-format = Format de taille
settings-view-selection-rect = Rectangle de sélection
settings-view-audio-badges = Afficher les badges LUFS / codec / durée sur les lignes audio
settings-view-similarity-score = Afficher le score de similarité MinHash sur les lignes de similarité
settings-view-preview-pane = Volet d'aperçu

# §8.7 General → Context Menu.
settings-context-menu-visibility = Visibilité
settings-context-menu-show = Afficher
settings-context-menu-shift = Afficher uniquement lorsque Maj est enfoncée
settings-context-menu-hide = Masquer
settings-context-menu-command = Macro de commande
settings-context-menu-open-folders = Ouvrir (dossiers)
settings-context-menu-open-files = Ouvrir (fichiers)
settings-context-menu-open-path = Ouvrir le chemin
settings-context-menu-explore = Explorer
settings-context-menu-explore-path = Explorer le chemin
settings-context-menu-copy-name = Copier le nom dans le presse-papiers
settings-context-menu-copy-path = Copier le chemin dans le presse-papiers
settings-context-menu-copy-full-name = Copier le nom complet dans le presse-papiers
settings-context-menu-reveal = Afficher dans Freally
settings-context-menu-send-to = Envoyer vers Freally (chemin)

# §8.8 General → Fonts & Colors.
settings-fc-font = Police
settings-fc-size = Taille
settings-fc-state-normal = Normal
settings-fc-state-highlighted = En surbrillance
settings-fc-state-current-sort = Tri actuel
settings-fc-state-current-sort-h = Tri actuel (en surbrillance)
settings-fc-state-selected = Sélectionné
settings-fc-state-selected-h = Sélectionné (en surbrillance)
settings-fc-state-inactive-selected = Sélectionné inactif
settings-fc-state-inactive-selected-h = Sélectionné inactif (en surbrillance)
settings-fc-foreground = Premier plan
settings-fc-background = Arrière-plan
settings-fc-bold = Gras
settings-fc-italic = Italique
settings-fc-default = Par défaut
settings-fc-per-lens-accent = Accentuation par filtre
settings-fc-theme-inherit = Inverser automatiquement les couleurs personnalisées au changement de thème

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = Raccourci global
settings-keyboard-new-window = Raccourci Nouvelle fenêtre
settings-keyboard-show-window = Raccourci Afficher la fenêtre
settings-keyboard-toggle-window = Raccourci Basculer la fenêtre
settings-keyboard-show-commands = Afficher les commandes contenant
settings-keyboard-add-chord = + Ajouter une combinaison
settings-keyboard-remove-chord = Supprimer

# §8.10 History.
settings-history-search-enable = Activer l'historique de recherche
settings-history-search-keep = Conserver l'historique de recherche pendant { $days } jours
settings-history-run-enable = Activer l'historique d'exécution
settings-history-run-keep = Conserver l'historique d'exécution pendant { $days } jours
settings-history-clear-now = Effacer maintenant
settings-history-privacy-mode = Mode confidentialité
settings-history-per-lens = Historique par filtre

# §8.11 Indexes (top-level).
settings-ix-database-location = Emplacement de la base de données
settings-ix-multiuser = Nom de fichier de la base de données multi-utilisateurs
settings-ix-compress = Compresser la base de données
settings-ix-recent-changes = Indexer les modifications récentes
settings-ix-file-size = Indexer la taille des fichiers
settings-ix-fast-size-sort = Tri rapide par taille
settings-ix-folder-size = Indexer la taille des dossiers
settings-ix-fast-folder-size-sort = Tri rapide par taille de dossier
settings-ix-date-created = Indexer la date de création
settings-ix-fast-date-created = Tri rapide par date de création
settings-ix-date-modified = Indexer la date de modification
settings-ix-fast-date-modified = Tri rapide par date de modification
settings-ix-date-accessed = Indexer la date d'accès
settings-ix-fast-date-accessed = Tri rapide par date d'accès
settings-ix-attributes = Indexer les attributs
settings-ix-fast-attributes = Tri rapide par attributs
settings-ix-fast-path-sort = Tri rapide par chemin
settings-ix-fast-extension-sort = Tri rapide par extension
settings-ix-force-rebuild = Forcer la reconstruction
settings-ix-compact = Compacter l'index
settings-ix-verify = Vérifier l'index
settings-ix-integrity-policy = Politique d'intégrité de l'index
settings-ix-memory-budget = Budget mémoire de l'indexeur
settings-ix-throttle = Limitation de l'indexation en arrière-plan

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = Inclure automatiquement les nouveaux volumes fixes
settings-vol-auto-removable = Inclure automatiquement les nouveaux volumes amovibles
settings-vol-auto-remove-offline = Retirer automatiquement les volumes hors ligne
settings-vol-detected = Volumes détectés
settings-vol-include = Inclure dans l'index
settings-vol-include-only = Inclure uniquement (glob/regex)
settings-vol-enable-usn = Activer le journal USN
settings-vol-enable-fsevents = Activer le flux FSEvents
settings-vol-enable-inotify = Activer inotify (ou fanotify si privilégié)
settings-vol-buffer = Taille du tampon du journal (Ko)
settings-vol-allocation-delta = Delta d'allocation (Ko)
settings-vol-load-recent = Charger les modifications récentes du journal au démarrage
settings-vol-monitor = Surveiller les modifications
settings-vol-recreate-journal = Recréer le journal
settings-vol-reset-stream = Réinitialiser le flux FSEvents
settings-vol-upgrade-fanotify = Passer à fanotify (polkit)
settings-vol-remove = Supprimer

# §8.13 Indexes → Folders.
settings-folders-watched = Dossiers surveillés
settings-folders-add = Ajouter…
settings-folders-rescan-now = Réanalyser maintenant
settings-folders-rescan-all = Tout réanalyser maintenant
settings-folders-monitor = Tenter de surveiller les modifications
settings-folders-buffer = Taille du tampon
settings-folders-rescan-on-full = Réanalyser lorsque le tampon est plein

# §8.14 Indexes → File Lists.
settings-flists-add = Ajouter…
settings-flists-monitor = Surveiller les modifications
settings-flists-editor = Éditeur de liste de fichiers…
settings-flists-format = Format de la liste de fichiers
settings-flists-format-text = Texte (un chemin par ligne)
settings-flists-format-json = JSON (avec métadonnées)
settings-flists-format-srcb = Bundle Freally (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = Exclure les fichiers et dossiers masqués
settings-exclude-system = Exclure les fichiers et dossiers système
settings-exclude-list-en = Activer la liste d'exclusion
settings-exclude-folders = Exclure les dossiers
settings-exclude-include-only-files = Inclure uniquement les fichiers (glob)
settings-exclude-files = Exclure les fichiers (glob)
settings-exclude-os-recommended = Appliquer les exclusions recommandées par l'OS
settings-exclude-by-class = Exclure par classe d'extension

# §8.16 Lenses → Filename.
settings-lf-trigram = Agressivité du préfiltrage par trigrammes
settings-lf-suffix-mem = Budget mémoire du tableau de suffixes
settings-lf-wildcard-limit = Limite d'expansion des caractères génériques
settings-lf-regex-timeout = Délai d'expiration des expressions régulières

# §8.17 Lenses → Content.
settings-lc-enable = Activer le filtre de contenu
settings-lc-time-budget = Budget de temps par document
settings-lc-mem-ceiling = Plafond mémoire par document
settings-lc-snippet-len = Longueur de l'extrait
settings-lc-stop-words = Langue des mots vides
settings-lc-re-extract = Réextraire lors d'un changement de paramètres
settings-lc-verify-blobs = Vérifier les sommes de contrôle des blobs de texte extrait à la lecture

# §8.18 Lenses → Audio.
settings-la-enable = Activer le filtre audio
settings-la-lufs-ref = Standard de référence LUFS
settings-la-peak-compute = Calculer le pic via
settings-la-silence-thresh = Seuil de silence
settings-la-re-extract-modify = Réextraire lors d'un événement de modification

# §8.19 Lenses → Similarity.
settings-ls-enable = Activer le filtre de similarité
settings-ls-sig-size = Taille de signature MinHash (k)
settings-ls-bands = Bandes LSH
settings-ls-recall = Seuil de rappel
settings-ls-result-cap = Plafond de résultats

# §8.20 Lenses → Custom.
settings-custom-registry = Registre
settings-custom-trust = Confiance
settings-custom-refresh-hashes = Actualiser les empreintes

# §8.21-§8.22 Network.
settings-net-https-enable = Activer le serveur HTTPS
settings-net-bind = Lier aux interfaces
settings-net-port = Écouter sur le port
settings-net-force-https = Forcer HTTPS
settings-net-legacy-auth = Authentification HTTP-basic héritée
settings-net-token-regen = Régénérer le jeton
settings-net-api-enable = Activer le serveur API
settings-net-legacy-ftp = Prise en charge FTP/ETP en clair héritée

# §8.23 Privacy & Updates.
settings-privacy-auto-update = Mise à jour automatique
settings-privacy-prerelease = Canal préliminaire
settings-privacy-network-policy = Politique des appels réseau

# §8.24 Logs & Debug.
settings-logs-level = Niveau de journalisation
settings-logs-location = Emplacement du fichier journal
settings-logs-retention = Conservation des journaux
settings-logs-debug-overlay = Afficher la superposition de débogage
settings-logs-open-folder = Ouvrir le dossier des journaux
settings-logs-export-bundle = Exporter le pack de diagnostics

# §8.25 Backup, Export, Reset.
settings-backup-export = Exporter les paramètres
settings-backup-import = Importer les paramètres
settings-backup-export-bookmarks = Exporter le pack de favoris
settings-backup-import-bookmarks = Importer le pack de favoris
settings-backup-reset-all = Réinitialiser tous les paramètres aux valeurs par défaut

# §8.26 Locale.
settings-locale-current = Langue actuelle
settings-locale-rtl-preview = Aperçu de droite à gauche
settings-locale-date-format = Format de date
settings-locale-number-format = Format des nombres

# §8.27 About.
settings-about-version = Freally { $version }
settings-about-license = Licence
settings-about-credits = Crédits
settings-about-notices = Mentions open-source

# --- TASK-098 additions: hints, placeholders, sub-sections, toasts ---

# Wizard polish.
wizard-aria-label = Assistant de première utilisation
wizard-step-of-total = Étape { $step } sur { $total }
wizard-roots-hint = Ajoutez les dossiers ou volumes que vous souhaitez faire surveiller par Freally. Vous pourrez les modifier ultérieurement dans les paramètres des index.
wizard-browse = Parcourir…
wizard-roots-placeholder = …ou collez un chemin
wizard-roots-add = Ajouter
wizard-roots-remove = Supprimer
wizard-roots-empty = Aucune racine configurée pour l'instant.
wizard-locale-hint = Freally est disponible en 18 langues. Vous pourrez en changer plus tard.
wizard-theme-hint = Système suit le paramètre d'apparence de votre OS.
wizard-back = Précédent
wizard-next = Suivant

# Status bar polish.
statusbar-hotkey-hint = Raccourci : { $hotkey }
statusbar-cycle-theme = Changer de thème
statusbar-indexed-suffix = indexés

# Results / lenses.
lens-expand = Développer le filtre
lens-collapse = Réduire le filtre
lens-no-matches = Aucune correspondance dans ce filtre.

# Preview pane.
preview-header = Aperçu
preview-loading = Chargement…
preview-select-file = Sélectionnez un fichier à prévisualiser.
preview-unavailable = Aucun aperçu disponible

# Bookmarks.
bookmarks-label = ★ Favoris
bookmarks-empty-hint = Aucun favori pour l'instant. Appuyez sur Ctrl+D pour enregistrer la requête actuelle.
bookmarks-organize-title = Organiser les favoris
bookmarks-organize-empty = Aucun favori pour l'instant.
bookmarks-rename = Renommer
bookmarks-close = Fermer

# Settings tree extras.
settings-group-history = Historique
settings-group-privacy = Confidentialité et mises à jour
settings-group-logs = Journaux et débogage
settings-group-backup = Sauvegarde, export, réinitialisation
settings-tree-custom-lens = Personnalisé
settings-unsaved-changes = modifications non enregistrées

# About dialog.
about-dialog-title = Freally
about-copyright = Copyright © 2026 Mike Weaver. Tous droits réservés.
about-close = Fermer

# Connect endpoint dialog.
connect-ftp-title = Se connecter au serveur FTP
connect-ftp-host = Hôte :
connect-ftp-port = Port :
connect-ftp-username = Nom d'utilisateur :
connect-ftp-password = Mot de passe :
connect-ftp-link-type = Type de liaison :

# UI panel.
ui-hint = Thème, intégration à la zone de notification / barre de menus, recherche à la frappe, densité des lignes. Parité directe avec voidtools-Everything, plus les ajouts Freally marqués d'un (+).
ui-section-theme = Thème
ui-theme-system-default = Système (par défaut)
ui-section-tray = Zone de notification / Barre de menus
ui-section-search-behavior = Comportement de recherche
ui-section-result-rows = Lignes de résultats
ui-single-click-system-default = Paramètres système (par défaut)
ui-single-click-always = Toujours simple clic
ui-single-click-always-double = Toujours double clic
ui-underline-always = Toujours
ui-underline-on-hover = Au survol
ui-underline-never = Jamais

# Home panel.
home-hint = Valeurs par défaut chargées au lancement de l'application — chaque menu déroulant peut conserver « Utiliser la dernière valeur » ou figer une valeur fixe. La visibilité des filtres / les limites de résultats sont des ajouts Freally (+).
home-section-match = Valeurs de correspondance par défaut
home-section-search-sort = Valeurs de recherche et de tri par défaut
home-search-placeholder = Vide par défaut
home-section-index = Source d'index
home-file-list-path = Chemin de la liste de fichiers
home-https-endpoint = URL du point de terminaison de l'API HTTPS
home-endpoint-token = Jeton (empreinte affichée)

# Backup panel.
backup-section-settings = Paramètres (+)
backup-section-bookmarks = Favoris + Extracteurs personnalisés (+)
backup-section-reset = Réinitialisation
backup-toast-exported = Paramètres exportés vers { $path }
backup-toast-export-failed = Échec de l'export : { $error }
backup-toast-imported = Paramètres importés
backup-toast-import-failed = Échec de l'import : { $error }
backup-toast-bookmarks-exported = Favoris exportés
backup-toast-bookmarks-export-failed = Échec de l'export des favoris : { $error }
backup-toast-bookmarks-imported = Favoris importés
backup-toast-bookmarks-import-failed = Échec de l'import des favoris : { $error }
backup-confirm-reset = Réinitialiser tous les paramètres aux valeurs par défaut ? Cette action est irréversible (la boîte de dialogue reste ouverte).
backup-toast-reset = Tous les paramètres ont été réinitialisés

# Keyboard panel.
keyboard-section-global = Raccourcis globaux
keyboard-placeholder-example = Super+Space
keyboard-section-commands = Commandes
keyboard-placeholder-command = identifiant de commande (par ex. file.export_results)
keyboard-placeholder-binding = Ctrl+K, B

# History panel.
history-section-search = Historique de recherche
history-section-run = Historique d'exécution
history-section-privacy = Confidentialité (+)
history-record-filename = Enregistrer l'historique du filtre Nom de fichier
history-record-content = Enregistrer l'historique du filtre Contenu
history-record-audio = Enregistrer l'historique du filtre Audio
history-record-similarity = Enregistrer l'historique du filtre Similarité

# Locale panel.
locale-section-language = Langue (+)
locale-section-time-date = Heure / Date (+)
locale-date-os = Valeur par défaut de l'OS
locale-date-iso8601 = ISO 8601
locale-date-rfc3339 = RFC 3339
locale-date-custom-label = Personnalisé
locale-date-custom-format = Format personnalisé
locale-date-placeholder = YYYY-MM-DD
locale-section-numbers = Nombres (+)
locale-number-os = Valeur par défaut de l'OS
locale-number-custom = Personnalisé
locale-thousands-sep = Séparateur de milliers
locale-decimal-sep = Séparateur décimal

# Folders panel.
folders-hint = Dossiers surveillés supplémentaires en plus des volumes par défaut.
folders-list-title = Dossiers surveillés
folders-empty = Aucun dossier ajouté pour l'instant.
folders-remove = Supprimer
folders-section-title-dynamic = Paramètres pour { $path }
folders-section-schedule = Planification de la réanalyse
folders-schedule-daily = Tous les jours à HH:MM
folders-schedule-hours = Toutes les N heures
folders-schedule-never = Jamais
folders-hour = Heure
folders-minute = Minute
folders-hours = Heures
folders-id-label = ID du dossier (lecture seule)
folders-select-prompt = Sélectionnez un dossier pour le configurer.
folders-section-extras = Extras Freally (+)
folders-extras-note = La réanalyse à la reprise après veille est activée par défaut dans cette version ; le commutateur rejoindra les contrôles au niveau des dossiers lors de la phase de finition 13.

# Volumes panel.
volumes-hint = Équivalent multiplateforme des panneaux NTFS / ReFS de voidtools-Everything. Détecte automatiquement NTFS / ReFS / exFAT / FAT32 (Win), APFS / HFS+ (macOS), ext4 / Btrfs / ZFS / XFS / F2FS (Linux).
volumes-section-auto-include = Inclusion automatique
volumes-list-title = Volumes détectés
volumes-detecting = Détection…
volumes-empty = Aucun volume détecté.
volumes-select-prompt = Sélectionnez un volume pour le configurer.

# About panel polish.
about-section-version = Version (+)
about-section-license = Licence (+)
about-license-text = Mike Weaver — Tous droits réservés. Ce logiciel est propriétaire.
about-license-spdx = SPDX : { $spdx }
about-section-credits = Crédits (+)
about-credits-inspired = Inspiré par Everything de voidtools.
about-credits-voidtools = voidtools.com
about-credits-repo = Dépôt du projet

# --- Menu bar (PRD §8.28) — every label + submenu + status-bar hover hint ---

# File menu.
menu-file-hint = Contient des commandes pour travailler avec Freally.
menu-file-new-window = Nouvelle fenêtre de recherche
menu-file-open-list = Ouvrir une liste de fichiers…
menu-file-close-list = Fermer la liste de fichiers
menu-file-close = Fermer
menu-file-export-results = Exporter les résultats…
menu-file-export-bundle = Exporter le pack d'index…
menu-file-exit = Quitter

# Edit menu.
menu-edit-hint = Contient des commandes pour modifier les résultats de recherche.
menu-edit-cut = Couper
menu-edit-copy = Copier
menu-edit-paste = Coller
menu-edit-copy-to-folder = Copier vers le dossier…
menu-edit-move-to-folder = Déplacer vers le dossier…
menu-edit-select-all = Tout sélectionner
menu-edit-invert-selection = Inverser la sélection
menu-edit-advanced = Avancé
menu-edit-copy-full-name = Copier le nom complet
menu-edit-copy-path = Copier le chemin
menu-edit-copy-filename = Copier le nom de fichier
menu-edit-copy-as-json = Copier en JSON
menu-edit-copy-with-metadata = Copier avec les métadonnées
menu-edit-copy-as-bundle-ref = Copier comme référence de bundle Freally

# View menu.
menu-view-hint = Contient des commandes pour manipuler l'affichage.
menu-view-filters = Filtres
menu-view-preview = Aperçu
menu-view-status-bar = Barre d'état
menu-view-thumbs-xl = Très grandes miniatures
menu-view-thumbs-l = Grandes miniatures
menu-view-thumbs-m = Miniatures moyennes
menu-view-details = Détails
menu-view-window-size = Taille de la fenêtre
menu-view-window-size-hint = Contient des commandes pour ajuster la taille de la fenêtre.
menu-view-window-small = Petite
menu-view-window-medium = Moyenne
menu-view-window-large = Grande
menu-view-window-auto = Ajustement automatique
menu-view-zoom = Zoom
menu-view-zoom-hint = Contient des commandes pour ajuster la taille de la police et des icônes.
menu-view-zoom-in = Zoom avant
menu-view-zoom-out = Zoom arrière
menu-view-zoom-reset = Réinitialiser
menu-view-sort-by = Trier par
menu-view-sort-by-hint = Contient des commandes pour trier la liste des résultats.
menu-view-sort-name = Nom
menu-view-sort-path = Chemin
menu-view-sort-size = Taille
menu-view-sort-ext = Extension
menu-view-sort-type = Type
menu-view-sort-modified = Date de modification
menu-view-sort-created = Date de création
menu-view-sort-accessed = Date d'accès
menu-view-sort-attributes = Attributs
menu-view-sort-recently-changed = Date de modification récente
menu-view-sort-run-count = Nombre d'exécutions
menu-view-sort-run-date = Date d'exécution
menu-view-sort-file-list-filename = Nom de fichier de la liste
menu-view-sort-lufs = LUFS
menu-view-sort-length = Durée
menu-view-sort-similarity = Score de similarité
menu-view-sort-asc = Croissant
menu-view-sort-desc = Décroissant
menu-view-go-to = Aller à
menu-view-refresh = Actualiser
menu-view-theme = Thème
menu-view-theme-hint = Basculer entre les thèmes système, clair ou sombre.
menu-view-lenses = Filtres
menu-view-lenses-hint = Activer ou désactiver la visibilité de chaque filtre dans la liste des résultats.
menu-view-on-top = Au premier plan
menu-view-on-top-hint = Contient des commandes pour garder cette fenêtre au-dessus des autres.
menu-view-on-top-never = Jamais
menu-view-on-top-always = Toujours
menu-view-on-top-while-searching = Pendant la recherche

# Search menu.
menu-search-hint = Contient les options de recherche.
menu-search-match-case = Respecter la casse
menu-search-match-whole-word = Mot entier
menu-search-match-path = Rechercher dans le chemin
menu-search-match-diacritics = Respecter les diacritiques
menu-search-enable-regex = Activer les expressions régulières
menu-search-advanced = Recherche avancée…
menu-search-add-to-filters = Ajouter aux filtres…
menu-search-organize-filters = Organiser les filtres…
menu-search-filter-everything = Tout
menu-search-filter-archive = Compressé (archive)
menu-search-filter-folder = Dossier
menu-search-filter-custom = Filtre personnalisé…

# Bookmarks menu.
menu-bookmarks-hint = Contient des commandes pour travailler avec les favoris.
menu-bookmarks-add = Ajouter aux favoris
menu-bookmarks-organize = Organiser les favoris…

# Tools menu.
menu-tools-hint = Contient les commandes d'outils.
menu-tools-connect = Se connecter au serveur FTP…
menu-tools-disconnect = Se déconnecter du serveur FTP
menu-tools-file-list-editor = Éditeur de liste de fichiers…
menu-tools-index-maintenance = Maintenance de l'index
menu-tools-index-maintenance-hint = Outils de maintenance de l'index.
menu-tools-verify-index = Vérifier l'index…
menu-tools-compact-index = Compacter l'index…
menu-tools-rebuild-index = Forcer la reconstruction de l'index…
menu-tools-custom-extractor = Gestionnaire d'extracteurs personnalisés…
menu-tools-custom-extractor-hint = Gérer les extracteurs personnalisés isolés dans un bac à sable Wasm.
menu-tools-options = Options…

# Help menu.
menu-help-hint = Contient les commandes d'aide.
menu-help-help = Aide de Freally
menu-help-search-syntax = Syntaxe de recherche
menu-help-regex-syntax = Syntaxe des expressions régulières
menu-help-audio-ref = Référence des modificateurs audio
menu-help-similarity-ref = Référence des modificateurs de similarité
menu-help-cli-options = Options de ligne de commande
menu-help-website = Site web de Freally
menu-help-check-updates = Rechercher des mises à jour…
menu-help-sponsor = Parrainer / Faire un don
menu-help-about = À propos de Freally…

# Result column headers (short forms used in the table header row).
column-name = Nom
column-path = Chemin
column-size = Taille
column-modified = Modifié
column-type = Type
column-ext = Ext
column-sort-by = Trier par { $name }
column-resize = Redimensionner la colonne { $name }

# Section subtitle bars used inside multiple settings panels.
section-behavior = Comportement
section-rendering = Rendu
section-status-bar = Barre d'état
section-display-format = Format d'affichage
section-loading-priority = Priorité de chargement
section-compatibility = Compatibilité
section-storage = Stockage
section-index-fields = Champs d'index
section-maintenance = Maintenance
section-logging = Journalisation
section-tools = Outils
section-privacy = Confidentialité
section-auto-update = Mise à jour automatique (+)
section-bind = Liaison
section-lens = Filtre
section-budgets = Budgets
section-other = Autre
section-per-format-mode = Mode par format
section-loudness = Sonie
section-tuning = Réglage (+)
section-minhash-lsh = Paramètres MinHash + LSH (+)
section-top-level = Niveau supérieur
section-file-globs = Globs de fichiers
section-file-list-settings = Paramètres de la liste de fichiers sélectionnée
section-editor-format = Éditeur + Format (E + +)
section-api-server = Serveur API (E adapté)
section-freally-extras = Extras Freally (+)
section-freally-additions = Ajouts Freally (+)
section-freally-extensions = Extensions Freally (+)

# Common option labels used across several Dropdowns.
opt-use-last-value = Utiliser la dernière valeur
opt-use-last-value-default = Utiliser la dernière valeur (par défaut)
opt-low = Faible
opt-normal-default = Normal (par défaut)
opt-high = Élevé
opt-disabled = Désactivé
opt-off = Désactivé
opt-on-battery = Sur batterie
opt-always = Toujours
opt-clamp-default = Limiter (par défaut)
opt-wrap = Renvoyer à la ligne
opt-none = Aucun
opt-strict-refuse = Strict (refuser les requêtes en cas de corruption)
opt-lenient-warn = Tolérant (avertir mais interroger)
opt-system-default = Valeur par défaut du système
opt-drag-select = Sélection par glissement
opt-auto-binary = Auto (binaire)
opt-auto-decimal = Auto (décimal)

# Unit suffixes shown next to number inputs.
unit-days = jours
unit-b = o
unit-kb = Ko
unit-mb = Mo
unit-gb = Go
unit-tb = To

# Additional dropdown option labels (extractor mode / sort / view / index / pane / precedence / LUFS / peak / log level / update channel).
opt-eager = Immédiat
opt-lazy-default = Différé (par défaut)
opt-on = Activé
opt-on-default = Activé (par défaut)
opt-all = Tout
opt-weekly = Hebdomadaire
opt-monthly = Mensuel
opt-name-asc = Nom croissant
opt-name-desc = Nom décroissant
opt-size-asc = Taille croissante
opt-size-desc = Taille décroissante
opt-modified-asc = Date de modification croissante
opt-modified-desc = Date de modification décroissante
opt-compact = Compact
opt-comfortable = Confortable
opt-details = Détails
opt-thumbnails = Miniatures
opt-local-db-default = Base de données locale (par défaut)
opt-file-list = Liste de fichiers
opt-https-endpoint = Point de terminaison de l'API HTTPS
opt-right-default = Droite (par défaut)
opt-bottom = Bas
opt-or-and-default = OR > AND (par défaut)
opt-and-or = AND > OR
opt-ebu-r128-default = EBU R128 (par défaut)
opt-atsc-a85 = ATSC A/85
opt-spotify = Spotify (-14)
opt-apple-music = Apple Music (-16)
opt-broadcast-film = Film de diffusion (-23)
opt-true-peak = Crête réelle (suréchantillonnage 4×, par défaut)
opt-sample-peak = Crête d'échantillon
opt-auto-per-doc = Auto (par document)
opt-log-error = Erreur
opt-log-warn = Avertissement
opt-log-info-default = Info (par défaut)
opt-log-debug = Débogage
opt-log-trace = Trace

# More Freally apps (Central inside panel) — host chrome
menu-help-more-apps = Plus d'apps Freally…
moreapps-title = Plus d'apps Freally
