# Sourcerer — Français.

app-name = Sourcerer
tagline = Une recherche. Toutes les sources. Tous les OS.
window-title = Sourcerer
search-placeholder = Rechercher…
about-version = Version { $version }

# Phase 11 — Chaînes d’interface (barre de recherche, barre de menu, barre d’état, assistant, etc.).
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
menu-bookmarks = Signets
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

wizard-title = Bienvenue dans Sourcerer
wizard-step-roots = Choisir les éléments à indexer
wizard-step-hotkey = Choisir un raccourci global
wizard-step-locale = Choisir votre langue
wizard-step-theme = Choisir un thème
wizard-finish = Terminer

# Phase 12 — Boîte de dialogue Options (PRD §8.1-§8.27).

settings-title = Options
settings-search-placeholder = Rechercher des options…
settings-restore-defaults = Réinitialiser
settings-ok = OK
settings-cancel = Annuler
settings-apply = Appliquer

# Groupes de navigation arborescente (PRD §8.1.1).
settings-group-general = Général
settings-group-indexes = Index
settings-group-lenses = Lentilles
settings-group-network = Réseau

# Feuilles de navigation arborescente.
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

# §8.2 Général → Interface.
settings-ui-theme = Thème
settings-ui-run-bg = Exécuter en arrière-plan
settings-ui-show-tray = Afficher l’icône dans la zone de notification / barre de menu
settings-ui-single-click-tray = Clic simple sur la zone de notification / barre de menu
settings-ui-new-window-from-tray = Ouvrir une nouvelle fenêtre depuis l’icône de la zone de notification
settings-ui-new-window-on-launch = Ouvrir une nouvelle fenêtre au lancement de Sourcerer
settings-ui-search-as-you-type = Recherche au fur et à mesure de la saisie
settings-ui-select-on-mouse-click = Sélectionner la recherche au clic de la souris
settings-ui-focus-on-activate = Mettre le focus sur la recherche à l’activation
settings-ui-full-row-select = Sélection de la ligne entière
settings-ui-single-click-open = Ouverture en un clic
settings-ui-underline-titles = Souligner les titres des icônes
settings-ui-row-density = Densité des résultats
settings-ui-row-density-compact = Compacte (32 px)
settings-ui-row-density-comfortable = Confortable (44 px)
settings-ui-show-timing-badges = Afficher les badges de chronométrage par lentille
settings-ui-anim-crossfade = Fondu enchaîné animé lors du changement de thème

# §8.3 Général → Accueil.
settings-home-match-case = Respecter la casse
settings-home-match-whole-word = Mot entier uniquement
settings-home-match-path = Inclure le chemin
settings-home-match-diacritics = Respecter les diacritiques
settings-home-match-regex = Utiliser des regex
settings-home-search = Recherche (requête par défaut personnalisée)
settings-home-filter = Filtrer
settings-home-sort = Trier
settings-home-view = Affichage
settings-home-index = Index
settings-home-default-lens-visibility = Visibilité par défaut des lentilles
settings-home-default-lens-result-limits = Limites de résultats par défaut des lentilles

# §8.4 Général → Recherche.
settings-search-fast-ascii = Recherche ASCII rapide
settings-search-mp-sep = Inclure le chemin si un terme contient un séparateur
settings-search-mw-fn = Comparer le nom de fichier complet avec les jokers
settings-search-lit-ops = Autoriser les opérateurs littéraux
settings-search-paren = Autoriser le regroupement entre parenthèses
settings-search-env = Étendre les variables d’environnement
settings-search-fwd-slash = Remplacer les barres obliques par des barres obliques inverses
settings-search-precedence = Précédence des opérateurs
settings-search-strict-everything = Mode syntaxe Everything strict
settings-search-auto-regex = Détection automatique des regex
settings-search-mod-comp = Complétion des modificateurs
settings-search-parse-tree = Afficher l’arbre d’analyse au survol

# §8.5 Général → Résultats.
settings-results-hide-empty = Masquer les résultats lorsque la recherche est vide
settings-results-clear-on-search = Effacer la sélection lors de la recherche
settings-results-close-on-execute = Fermer la fenêtre à l’exécution
settings-results-dbl-path = Ouvrir le chemin par double-clic dans la colonne du chemin
settings-results-auto-scroll = Faire défiler automatiquement la vue
settings-results-dquote-copy = Copier entre guillemets en tant que chemin
settings-results-no-ext-rename = Ne pas sélectionner l’extension lors du renommage
settings-results-sort-date-desc = Trier d’abord les dates décroissantes
settings-results-sort-size-desc = Trier d’abord les tailles décroissantes
settings-results-list-focus = Focus de la liste de résultats
settings-results-icon-prio = Priorité de chargement des icônes
settings-results-thumb-prio = Priorité de chargement des miniatures
settings-results-ext-prio = Priorité de chargement des informations étendues
settings-results-group-by-lens = Grouper les résultats par lentille
settings-results-snippet-inline = Afficher l’aperçu d’extrait en ligne

# §8.6 Général → Affichage.
settings-view-double-buffer = Double tampon
settings-view-alt-rows = Alterner la couleur des lignes
settings-view-row-mouseover = Afficher le survol des lignes
settings-view-highlight-terms = Surligner les termes recherchés
settings-view-status-show-selected = Afficher l’élément sélectionné dans la barre d’état
settings-view-rc-with-sel = Afficher le nombre de résultats avec le nombre de sélection
settings-view-status-show-size = Afficher la taille dans la barre d’état
settings-view-tooltips = Afficher les info-bulles
settings-view-update-on-scroll = Mettre à jour l’affichage immédiatement après le défilement
settings-view-size-format = Format de taille
settings-view-selection-rect = Rectangle de sélection
settings-view-audio-badges = Afficher les badges LUFS / codec / durée sur les lignes audio
settings-view-similarity-score = Afficher le score de similarité MinHash sur les lignes de similarité
settings-view-preview-pane = Volet d’aperçu

# §8.7 Général → Menu contextuel.
settings-context-menu-visibility = Visibilité
settings-context-menu-show = Afficher
settings-context-menu-shift = Afficher uniquement avec Maj enfoncée
settings-context-menu-hide = Masquer
settings-context-menu-command = Macro de commande
settings-context-menu-open-folders = Ouvrir (Dossiers)
settings-context-menu-open-files = Ouvrir (Fichiers)
settings-context-menu-open-path = Ouvrir le chemin
settings-context-menu-explore = Explorer
settings-context-menu-explore-path = Explorer le chemin
settings-context-menu-copy-name = Copier le nom dans le presse-papiers
settings-context-menu-copy-path = Copier le chemin dans le presse-papiers
settings-context-menu-copy-full-name = Copier le nom complet dans le presse-papiers
settings-context-menu-reveal = Afficher dans Sourcerer
settings-context-menu-send-to = Envoyer vers Sourcerer (chemin)

# §8.8 Général → Polices et couleurs.
settings-fc-font = Police
settings-fc-size = Taille
settings-fc-state-normal = Normal
settings-fc-state-highlighted = Surligné
settings-fc-state-current-sort = Tri courant
settings-fc-state-current-sort-h = Tri courant (surligné)
settings-fc-state-selected = Sélectionné
settings-fc-state-selected-h = Sélectionné (surligné)
settings-fc-state-inactive-selected = Sélectionné inactif
settings-fc-state-inactive-selected-h = Sélectionné inactif (surligné)
settings-fc-foreground = Premier plan
settings-fc-background = Arrière-plan
settings-fc-bold = Gras
settings-fc-italic = Italique
settings-fc-default = Par défaut
settings-fc-per-lens-accent = Accent par lentille
settings-fc-theme-inherit = Inverser automatiquement les couleurs personnalisées au changement de thème

# §8.9 Général → Clavier.
settings-keyboard-global-hotkey = Raccourci global
settings-keyboard-new-window = Raccourci Nouvelle fenêtre
settings-keyboard-show-window = Raccourci Afficher la fenêtre
settings-keyboard-toggle-window = Raccourci Basculer la fenêtre
settings-keyboard-show-commands = Afficher les commandes contenant
settings-keyboard-add-chord = + Ajouter un accord
settings-keyboard-remove-chord = Supprimer

# §8.10 Historique.
settings-history-search-enable = Activer l’historique de recherche
settings-history-search-keep = Conserver l’historique de recherche pendant { $days } jours
settings-history-run-enable = Activer l’historique d’exécution
settings-history-run-keep = Conserver l’historique d’exécution pendant { $days } jours
settings-history-clear-now = Effacer maintenant
settings-history-privacy-mode = Mode confidentiel
settings-history-per-lens = Historique par lentille

# §8.11 Index (niveau supérieur).
settings-ix-database-location = Emplacement de la base de données
settings-ix-multiuser = Nom de fichier de la base multi-utilisateur
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
settings-ix-date-accessed = Indexer la date d’accès
settings-ix-fast-date-accessed = Tri rapide par date d’accès
settings-ix-attributes = Indexer les attributs
settings-ix-fast-attributes = Tri rapide par attributs
settings-ix-fast-path-sort = Tri rapide par chemin
settings-ix-fast-extension-sort = Tri rapide par extension
settings-ix-force-rebuild = Forcer la reconstruction
settings-ix-compact = Compacter l’index
settings-ix-verify = Vérifier l’index
settings-ix-integrity-policy = Politique d’intégrité de l’index
settings-ix-memory-budget = Budget mémoire de l’indexeur
settings-ix-throttle = Limitation de l’indexation en arrière-plan

# §8.12 Index → Volumes.
settings-vol-auto-fixed = Inclure automatiquement les nouveaux volumes fixes
settings-vol-auto-removable = Inclure automatiquement les nouveaux volumes amovibles
settings-vol-auto-remove-offline = Retirer automatiquement les volumes hors ligne
settings-vol-detected = Volumes détectés
settings-vol-include = Inclure dans l’index
settings-vol-include-only = Inclure uniquement (glob/regex)
settings-vol-enable-usn = Activer le journal USN
settings-vol-enable-fsevents = Activer le flux FSEvents
settings-vol-enable-inotify = Activer inotify (ou fanotify si élevé)
settings-vol-buffer = Taille du tampon de journal (Ko)
settings-vol-allocation-delta = Delta d’allocation (Ko)
settings-vol-load-recent = Charger les modifications récentes du journal au démarrage
settings-vol-monitor = Surveiller les modifications
settings-vol-recreate-journal = Recréer le journal
settings-vol-reset-stream = Réinitialiser le flux FSEvents
settings-vol-upgrade-fanotify = Passer à fanotify (polkit)
settings-vol-remove = Retirer

# §8.13 Index → Dossiers.
settings-folders-watched = Dossiers surveillés
settings-folders-add = Ajouter…
settings-folders-rescan-now = Réanalyser maintenant
settings-folders-rescan-all = Tout réanalyser maintenant
settings-folders-monitor = Tenter de surveiller les modifications
settings-folders-buffer = Taille du tampon
settings-folders-rescan-on-full = Réanalyser quand le tampon est plein

# §8.14 Index → Listes de fichiers.
settings-flists-add = Ajouter…
settings-flists-monitor = Surveiller les modifications
settings-flists-editor = Éditeur de liste de fichiers…
settings-flists-format = Format de liste de fichiers
settings-flists-format-text = Texte (un chemin par ligne)
settings-flists-format-json = JSON (avec métadonnées)
settings-flists-format-srcb = Bundle Sourcerer (.srcb)

# §8.15 Index → Exclure.
settings-exclude-hidden = Exclure les fichiers et dossiers cachés
settings-exclude-system = Exclure les fichiers et dossiers système
settings-exclude-list-en = Activer la liste d’exclusion
settings-exclude-folders = Exclure les dossiers
settings-exclude-include-only-files = Inclure uniquement les fichiers (glob)
settings-exclude-files = Exclure les fichiers (glob)
settings-exclude-os-recommended = Appliquer les exclusions recommandées par l’OS
settings-exclude-by-class = Exclure par classe d’extension

# §8.16 Lentilles → Nom de fichier.
settings-lf-trigram = Agressivité du préfiltrage par trigrammes
settings-lf-suffix-mem = Budget mémoire du tableau de suffixes
settings-lf-wildcard-limit = Limite d’expansion des jokers
settings-lf-regex-timeout = Délai d’expiration des regex

# §8.17 Lentilles → Contenu.
settings-lc-enable = Activer la lentille de contenu
settings-lc-time-budget = Budget temps par document
settings-lc-mem-ceiling = Plafond mémoire par document
settings-lc-snippet-len = Longueur des extraits
settings-lc-stop-words = Langue des mots vides
settings-lc-re-extract = Réextraire au changement de paramètres
settings-lc-verify-blobs = Vérifier les sommes de contrôle des blobs de texte extrait à la lecture

# §8.18 Lentilles → Audio.
settings-la-enable = Activer la lentille audio
settings-la-lufs-ref = Standard de référence LUFS
settings-la-peak-compute = Calculer le pic via
settings-la-silence-thresh = Seuil de silence
settings-la-re-extract-modify = Réextraire à l’événement Modifier

# §8.19 Lentilles → Similarité.
settings-ls-enable = Activer la lentille de similarité
settings-ls-sig-size = Taille de signature MinHash (k)
settings-ls-bands = Bandes LSH
settings-ls-recall = Seuil de rappel
settings-ls-result-cap = Plafond de résultats

# §8.20 Lentilles → Personnalisé.
settings-custom-registry = Registre
settings-custom-trust = Confiance
settings-custom-refresh-hashes = Actualiser les hachages

# §8.21-§8.22 Réseau.
settings-net-https-enable = Activer le serveur HTTPS
settings-net-bind = Lier aux interfaces
settings-net-port = Écouter sur le port
settings-net-force-https = Forcer HTTPS
settings-net-legacy-auth = Authentification HTTP-basic héritée
settings-net-token-regen = Régénérer le jeton
settings-net-api-enable = Activer le serveur API
settings-net-legacy-ftp = Prise en charge FTP/ETP en clair (héritée)

# §8.23 Confidentialité et mises à jour.
settings-privacy-auto-update = Mise à jour automatique
settings-privacy-prerelease = Canal préliminaire
settings-privacy-network-policy = Politique d’appels réseau

# §8.24 Journaux et débogage.
settings-logs-level = Niveau de journalisation
settings-logs-location = Emplacement du fichier journal
settings-logs-retention = Rétention des journaux
settings-logs-debug-overlay = Afficher la surcouche de débogage
settings-logs-open-folder = Ouvrir le dossier des journaux
settings-logs-export-bundle = Exporter le bundle de diagnostics

# §8.25 Sauvegarde, export, réinitialisation.
settings-backup-export = Exporter les paramètres
settings-backup-import = Importer les paramètres
settings-backup-export-bookmarks = Exporter le bundle de signets
settings-backup-import-bookmarks = Importer le bundle de signets
settings-backup-reset-all = Réinitialiser tous les paramètres aux valeurs par défaut

# §8.26 Langue.
settings-locale-current = Langue actuelle
settings-locale-rtl-preview = Aperçu droite-à-gauche
settings-locale-date-format = Format de date
settings-locale-number-format = Format de nombre

# §8.27 À propos.
settings-about-version = Sourcerer { $version }
settings-about-license = Licence
settings-about-credits = Crédits
settings-about-notices = Mentions des logiciels libres
