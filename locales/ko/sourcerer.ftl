# Sourcerer — 한국어.

app-name = Sourcerer
tagline = 하나의 검색. 모든 소스. 모든 OS.
window-title = Sourcerer
search-placeholder = 검색…
about-version = 버전 { $version }

# Phase 11 — UI strings (search bar, menu bar, status bar, wizard, etc.).
status-ready = 준비됨
status-indexed = 색인 완료 (파일 { $count }개)
status-indexing = 색인 중… { $done }/{ $total }
status-paused = 일시 중지됨
status-error = 오류
status-result-count-one = 결과 { $count }개
status-result-count-many = 결과 { $count }개
status-selection = · { $count }개 선택됨
status-selection-size = 선택됨: { $size }
status-query-timing = 쿼리: { $ms } ms
status-endpoint-local = 로컬 DB
status-endpoint-remote = API: { $name }

menu-file = 파일
menu-edit = 편집
menu-view = 보기
menu-search = 검색
menu-bookmarks = 즐겨찾기
menu-tools = 도구
menu-help = 도움말

theme-system = 시스템
theme-light = 라이트
theme-dark = 다크

lens-filename = 파일 이름
lens-content = 콘텐츠
lens-audio = 오디오
lens-similarity = 유사도

parse-error-empty = 검색어를 입력하세요.
parse-error-unknown = 인식할 수 없는 구문입니다.

action-open = 열기
action-reveal = 폴더에서 보기
action-copy-path = 경로 복사
action-copy-name = 이름 복사
action-delete = 삭제

quick-filter-audio = 오디오
quick-filter-video = 비디오
quick-filter-image = 이미지
quick-filter-document = 문서
quick-filter-executable = 실행 파일
quick-filter-archive = 압축 파일

wizard-title = Sourcerer에 오신 것을 환영합니다
wizard-step-roots = 색인할 항목 선택
wizard-step-hotkey = 전역 단축키 선택
wizard-step-locale = 언어 선택
wizard-step-theme = 테마 선택
wizard-finish = 완료

# Phase 12 — Settings dialog (PRD §8.1-§8.27).

settings-title = 옵션
settings-search-placeholder = 옵션 검색…
settings-restore-defaults = 기본값 복원
settings-ok = 확인
settings-cancel = 취소
settings-apply = 적용

# Tree nav groups (PRD §8.1.1).
settings-group-general = 일반
settings-group-indexes = 색인
settings-group-lenses = 렌즈
settings-group-network = 네트워크

# Tree nav leaves.
settings-node-ui = UI
settings-node-home = 홈
settings-node-search = 검색
settings-node-results = 결과
settings-node-view = 보기
settings-node-context-menu = 컨텍스트 메뉴
settings-node-fonts-colors = 글꼴 및 색상
settings-node-keyboard = 키보드
settings-node-history = 기록
settings-node-indexes-top = (최상위)
settings-node-volumes = 볼륨
settings-node-folders = 폴더
settings-node-file-lists = 파일 목록
settings-node-exclude = 제외
settings-node-https-server = HTTP / HTTPS 서버
settings-node-etp-api = ETP / FTP API
settings-node-privacy = 개인정보 및 업데이트
settings-node-logs = 로그 및 디버그
settings-node-backup = 백업, 내보내기, 초기화
settings-node-locale = 로케일
settings-node-about = 정보

# §8.2 General → UI.
settings-ui-theme = 테마
settings-ui-run-bg = 백그라운드에서 실행
settings-ui-show-tray = 트레이 / 메뉴 바 아이콘 표시
settings-ui-single-click-tray = 트레이 / 메뉴 바 단일 클릭
settings-ui-new-window-from-tray = 트레이 아이콘에서 새 창 열기
settings-ui-new-window-on-launch = Sourcerer 실행 시 새 창 열기
settings-ui-search-as-you-type = 입력하는 동안 검색
settings-ui-select-on-mouse-click = 마우스 클릭 시 검색어 선택
settings-ui-focus-on-activate = 활성화 시 검색창에 포커스
settings-ui-full-row-select = 행 전체 선택
settings-ui-single-click-open = 단일 클릭으로 열기
settings-ui-underline-titles = 아이콘 제목에 밑줄
settings-ui-row-density = 결과 밀도
settings-ui-row-density-compact = 축소 (32 px)
settings-ui-row-density-comfortable = 여유 (44 px)
settings-ui-show-timing-badges = 렌즈별 타이밍 배지 표시
settings-ui-anim-crossfade = 테마 전환 시 크로스페이드 애니메이션

# §8.3 General → Home.
settings-home-match-case = 대/소문자 구분
settings-home-match-whole-word = 단어 단위 일치
settings-home-match-path = 경로 일치
settings-home-match-diacritics = 발음 구별 부호 일치
settings-home-match-regex = Regex 일치
settings-home-search = 검색 (사용자 지정 기본 쿼리)
settings-home-filter = 필터
settings-home-sort = 정렬
settings-home-view = 보기
settings-home-index = 색인
settings-home-default-lens-visibility = 기본 렌즈 표시 여부
settings-home-default-lens-result-limits = 기본 렌즈 결과 한도

# §8.4 General → Search.
settings-search-fast-ascii = 빠른 ASCII 검색
settings-search-mp-sep = 검색어에 경로 구분자가 포함된 경우 경로 일치
settings-search-mw-fn = 와일드카드 사용 시 전체 파일 이름 일치
settings-search-lit-ops = 리터럴 연산자 허용
settings-search-paren = 소괄호 그룹화 허용
settings-search-env = 환경 변수 확장
settings-search-fwd-slash = 슬래시를 백슬래시로 대체
settings-search-precedence = 연산자 우선순위
settings-search-strict-everything = 엄격한 Everything 구문 모드
settings-search-auto-regex = Regex 자동 감지
settings-search-mod-comp = 한정자 자동 완성
settings-search-parse-tree = 마우스 오버 시 파스 트리 표시

# §8.5 General → Results.
settings-results-hide-empty = 검색어가 비어 있을 때 결과 숨기기
settings-results-clear-on-search = 검색 시 선택 항목 지우기
settings-results-close-on-execute = 실행 시 창 닫기
settings-results-dbl-path = 경로 열에서 두 번 클릭하여 경로 열기
settings-results-auto-scroll = 보기 자동 스크롤
settings-results-dquote-copy = 큰따옴표로 묶어 경로 복사
settings-results-no-ext-rename = 이름 변경 시 확장자 제외
settings-results-sort-date-desc = 날짜를 우선 내림차순 정렬
settings-results-sort-size-desc = 크기를 우선 내림차순 정렬
settings-results-list-focus = 결과 목록 포커스
settings-results-icon-prio = 아이콘 로드 우선순위
settings-results-thumb-prio = 미리 보기 로드 우선순위
settings-results-ext-prio = 확장 정보 로드 우선순위
settings-results-group-by-lens = 렌즈별로 결과 그룹화
settings-results-snippet-inline = 스니펫 미리 보기를 인라인으로 표시

# §8.6 General → View.
settings-view-double-buffer = 더블 버퍼링
settings-view-alt-rows = 행 색상 교차 표시
settings-view-row-mouseover = 행 마우스 오버 표시
settings-view-highlight-terms = 검색어 강조 표시
settings-view-status-show-selected = 상태 표시줄에 선택된 항목 표시
settings-view-rc-with-sel = 결과 수와 선택 수를 함께 표시
settings-view-status-show-size = 상태 표시줄에 크기 표시
settings-view-tooltips = 툴팁 표시
settings-view-update-on-scroll = 스크롤 직후 화면 즉시 업데이트
settings-view-size-format = 크기 형식
settings-view-selection-rect = 선택 영역
settings-view-audio-badges = 오디오 행에 LUFS / codec / 길이 배지 표시
settings-view-similarity-score = 유사도 행에 MinHash 유사도 점수 표시
settings-view-preview-pane = 미리 보기 창

# §8.7 General → Context Menu.
settings-context-menu-visibility = 표시 여부
settings-context-menu-show = 표시
settings-context-menu-shift = Shift를 누른 경우에만 표시
settings-context-menu-hide = 숨기기
settings-context-menu-command = 명령 매크로
settings-context-menu-open-folders = 열기 (폴더)
settings-context-menu-open-files = 열기 (파일)
settings-context-menu-open-path = 경로 열기
settings-context-menu-explore = 탐색
settings-context-menu-explore-path = 경로 탐색
settings-context-menu-copy-name = 클립보드에 이름 복사
settings-context-menu-copy-path = 클립보드에 경로 복사
settings-context-menu-copy-full-name = 클립보드에 전체 이름 복사
settings-context-menu-reveal = Sourcerer에서 보기
settings-context-menu-send-to = Sourcerer로 보내기 (경로)

# §8.8 General → Fonts & Colors.
settings-fc-font = 글꼴
settings-fc-size = 크기
settings-fc-state-normal = 보통
settings-fc-state-highlighted = 강조됨
settings-fc-state-current-sort = 현재 정렬
settings-fc-state-current-sort-h = 현재 정렬 (강조됨)
settings-fc-state-selected = 선택됨
settings-fc-state-selected-h = 선택됨 (강조됨)
settings-fc-state-inactive-selected = 비활성 선택됨
settings-fc-state-inactive-selected-h = 비활성 선택됨 (강조됨)
settings-fc-foreground = 전경색
settings-fc-background = 배경색
settings-fc-bold = 굵게
settings-fc-italic = 기울임꼴
settings-fc-default = 기본값
settings-fc-per-lens-accent = 렌즈별 강조색
settings-fc-theme-inherit = 테마 전환 시 사용자 지정 색상 자동 반전

# §8.9 General → Keyboard.
settings-keyboard-global-hotkey = 전역 단축키
settings-keyboard-new-window = 새 창 단축키
settings-keyboard-show-window = 창 표시 단축키
settings-keyboard-toggle-window = 창 토글 단축키
settings-keyboard-show-commands = 다음을 포함하는 명령 표시
settings-keyboard-add-chord = + 코드 추가
settings-keyboard-remove-chord = 제거

# §8.10 History.
settings-history-search-enable = 검색 기록 사용
settings-history-search-keep = 검색 기록을 { $days }일 동안 보관
settings-history-run-enable = 실행 기록 사용
settings-history-run-keep = 실행 기록을 { $days }일 동안 보관
settings-history-clear-now = 지금 지우기
settings-history-privacy-mode = 프라이버시 모드
settings-history-per-lens = 렌즈별 기록

# §8.11 Indexes (top-level).
settings-ix-database-location = 데이터베이스 위치
settings-ix-multiuser = 다중 사용자 데이터베이스 파일 이름
settings-ix-compress = 데이터베이스 압축
settings-ix-recent-changes = 최근 변경 사항 색인
settings-ix-file-size = 파일 크기 색인
settings-ix-fast-size-sort = 빠른 크기 정렬
settings-ix-folder-size = 폴더 크기 색인
settings-ix-fast-folder-size-sort = 빠른 폴더 크기 정렬
settings-ix-date-created = 생성 날짜 색인
settings-ix-fast-date-created = 빠른 생성 날짜 정렬
settings-ix-date-modified = 수정 날짜 색인
settings-ix-fast-date-modified = 빠른 수정 날짜 정렬
settings-ix-date-accessed = 액세스 날짜 색인
settings-ix-fast-date-accessed = 빠른 액세스 날짜 정렬
settings-ix-attributes = 속성 색인
settings-ix-fast-attributes = 빠른 속성 정렬
settings-ix-fast-path-sort = 빠른 경로 정렬
settings-ix-fast-extension-sort = 빠른 확장자 정렬
settings-ix-force-rebuild = 강제 재구축
settings-ix-compact = 색인 압축
settings-ix-verify = 색인 검증
settings-ix-integrity-policy = 색인 무결성 정책
settings-ix-memory-budget = 색인 작업기 메모리 예산
settings-ix-throttle = 백그라운드 색인 제한

# §8.12 Indexes → Volumes.
settings-vol-auto-fixed = 새 고정 볼륨 자동 포함
settings-vol-auto-removable = 새 이동식 볼륨 자동 포함
settings-vol-auto-remove-offline = 오프라인 볼륨 자동 제거
settings-vol-detected = 감지된 볼륨
settings-vol-include = 색인에 포함
settings-vol-include-only = 다음만 포함 (glob/regex)
settings-vol-enable-usn = USN Journal 사용
settings-vol-enable-fsevents = FSEvents 스트림 사용
settings-vol-enable-inotify = inotify 사용 (권한 상승 시 fanotify)
settings-vol-buffer = 저널 버퍼 크기 (KB)
settings-vol-allocation-delta = 할당 델타 (KB)
settings-vol-load-recent = 시작 시 저널에서 최근 변경 사항 로드
settings-vol-monitor = 변경 사항 모니터링
settings-vol-recreate-journal = 저널 재생성
settings-vol-reset-stream = FSEvents 스트림 재설정
settings-vol-upgrade-fanotify = fanotify로 업그레이드 (polkit)
settings-vol-remove = 제거

# §8.13 Indexes → Folders.
settings-folders-watched = 감시 중인 폴더
settings-folders-add = 추가…
settings-folders-rescan-now = 지금 다시 스캔
settings-folders-rescan-all = 모두 다시 스캔
settings-folders-monitor = 변경 사항 모니터링 시도
settings-folders-buffer = 버퍼 크기
settings-folders-rescan-on-full = 버퍼 가득 참 시 다시 스캔

# §8.14 Indexes → File Lists.
settings-flists-add = 추가…
settings-flists-monitor = 변경 사항 모니터링
settings-flists-editor = 파일 목록 편집기…
settings-flists-format = 파일 목록 형식
settings-flists-format-text = 텍스트 (행당 경로 하나)
settings-flists-format-json = JSON (메타데이터 포함)
settings-flists-format-srcb = Sourcerer 번들 (.srcb)

# §8.15 Indexes → Exclude.
settings-exclude-hidden = 숨김 파일 및 폴더 제외
settings-exclude-system = 시스템 파일 및 폴더 제외
settings-exclude-list-en = 제외 목록 사용
settings-exclude-folders = 제외할 폴더
settings-exclude-include-only-files = 다음 파일만 포함 (glob)
settings-exclude-files = 제외할 파일 (glob)
settings-exclude-os-recommended = OS 권장 제외 항목 적용
settings-exclude-by-class = 확장자 클래스별 제외

# §8.16 Lenses → Filename.
settings-lf-trigram = trigram 사전 필터 적극성
settings-lf-suffix-mem = 접미사 배열 메모리 예산
settings-lf-wildcard-limit = 와일드카드 확장 한도
settings-lf-regex-timeout = Regex 타임아웃

# §8.17 Lenses → Content.
settings-lc-enable = 콘텐츠 렌즈 사용
settings-lc-time-budget = 문서당 시간 예산
settings-lc-mem-ceiling = 문서당 메모리 상한
settings-lc-snippet-len = 스니펫 길이
settings-lc-stop-words = 불용어 언어
settings-lc-re-extract = 설정 변경 시 재추출
settings-lc-verify-blobs = 읽을 때 추출된 텍스트 blob 체크섬 검증

# §8.18 Lenses → Audio.
settings-la-enable = 오디오 렌즈 사용
settings-la-lufs-ref = LUFS 기준 표준
settings-la-peak-compute = 피크 계산 방식
settings-la-silence-thresh = 무음 임계값
settings-la-re-extract-modify = 수정 이벤트 시 재추출

# §8.19 Lenses → Similarity.
settings-ls-enable = 유사도 렌즈 사용
settings-ls-sig-size = MinHash 서명 크기 (k)
settings-ls-bands = LSH 밴드
settings-ls-recall = 재현율 임계값
settings-ls-result-cap = 결과 상한

# §8.20 Lenses → Custom.
settings-custom-registry = 레지스트리
settings-custom-trust = 신뢰
settings-custom-refresh-hashes = 해시 새로 고침

# §8.21-§8.22 Network.
settings-net-https-enable = HTTPS 서버 사용
settings-net-bind = 인터페이스에 바인딩
settings-net-port = 수신 포트
settings-net-force-https = HTTPS 강제 사용
settings-net-legacy-auth = 레거시 HTTP 기본 인증
settings-net-token-regen = 토큰 재생성
settings-net-api-enable = API 서버 사용
settings-net-legacy-ftp = 레거시 일반 FTP/ETP 지원

# §8.23 Privacy & Updates.
settings-privacy-auto-update = 자동 업데이트
settings-privacy-prerelease = 사전 출시 채널
settings-privacy-network-policy = 네트워크 호출 정책

# §8.24 Logs & Debug.
settings-logs-level = 로그 수준
settings-logs-location = 로그 파일 위치
settings-logs-retention = 로그 보관 기간
settings-logs-debug-overlay = 디버그 오버레이 표시
settings-logs-open-folder = 로그 폴더 열기
settings-logs-export-bundle = 진단 번들 내보내기

# §8.25 Backup, Export, Reset.
settings-backup-export = 설정 내보내기
settings-backup-import = 설정 가져오기
settings-backup-export-bookmarks = 즐겨찾기 번들 내보내기
settings-backup-import-bookmarks = 즐겨찾기 번들 가져오기
settings-backup-reset-all = 모든 설정을 기본값으로 초기화

# §8.26 Locale.
settings-locale-current = 현재 로케일
settings-locale-rtl-preview = RTL 미리 보기
settings-locale-date-format = 날짜 형식
settings-locale-number-format = 숫자 형식

# §8.27 About.
settings-about-version = Sourcerer { $version }
settings-about-license = 라이선스
settings-about-credits = 크레딧
settings-about-notices = 오픈 소스 고지
