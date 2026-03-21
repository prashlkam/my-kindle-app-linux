# Kindle App for Linux — Complete Build Plan

## 1. Project Overview

**Goal:** Build a native Linux ebook reader that supports Kindle formats (MOBI, AZW3/KF8) alongside EPUB, with a reading experience comparable to the official Kindle app — annotations, bookmarks, dictionary, library management, and optional sync.

**Name (working title):** `kindle-linux` or `Kindled`

---

## 2. Technology Stack

| Layer | Choice | Rationale |
|-------|--------|-----------|
| **App Framework** | **Tauri 2.x** (Rust backend + web frontend) | Small binary (<10MB), uses system WebKitGTK — no bundled Chromium. Rust backend for file I/O, DB, and format parsing. |
| **Frontend** | **Svelte 5 + TypeScript** | Lightweight, fast, minimal boilerplate. Excellent for reactive UI without virtual DOM overhead. |
| **Book Rendering** | **foliate-js** | Framework-agnostic JS library. Supports EPUB, MOBI, KF8/AZW3, FB2, CBZ. Production-proven in Foliate. Runs in the WebView. |
| **PDF Rendering** | **pdf.js** (Mozilla) | Standard PDF renderer for WebView contexts. |
| **Database** | **SQLite** via `rusqlite` | Local storage for library metadata, annotations, bookmarks, reading progress. |
| **Styling** | **Tailwind CSS 4** | Utility-first, small output, easy theming. |
| **Build System** | **Cargo + Vite** | Tauri's default toolchain. |
| **Package Formats** | **.deb, .rpm, AppImage, Flatpak** | Tauri bundles .deb and AppImage natively. Add Flatpak via CI. |

---

## 3. Supported Formats

| Format | Support Level | Library |
|--------|--------------|---------|
| EPUB 2/3 | Full | foliate-js |
| MOBI | Full (DRM-free) | foliate-js |
| AZW3 / KF8 | Full (DRM-free) | foliate-js |
| FB2 | Full | foliate-js |
| CBZ/CBR | Full | foliate-js |
| PDF | Full | pdf.js |
| KFX | Future / Plugin | Requires custom Ion parser |
| DRM-protected | Not supported | Legal constraints (DMCA §1201) |

**DRM Policy:** The app will NOT include or endorse DRM circumvention. It handles DRM-free files only. A plugin architecture allows third-party extensions at the user's discretion.

---

## 4. Architecture

```
┌──────────────────────────────────────────────────┐
│                   Tauri Shell                     │
│                                                   │
│  ┌─────────────────────┐  ┌────────────────────┐ │
│  │   Rust Backend       │  │  Svelte Frontend   │ │
│  │                      │  │                    │ │
│  │  - File I/O          │  │  - Library UI      │ │
│  │  - SQLite (rusqlite) │  │  - Reader View     │ │
│  │  - Book import       │  │  - Settings        │ │
│  │  - Metadata extract  │  │  - Annotations     │ │
│  │  - Plugin host       │  │  - Search          │ │
│  │  - OPDS client       │  │                    │ │
│  │  - Export/Import     │  │  ┌──────────────┐  │ │
│  │  - IPC commands      │  │  │  WebView     │  │ │
│  │                      │  │  │  (WebKitGTK) │  │ │
│  │                      │  │  │              │  │ │
│  │                      │  │  │  foliate-js  │  │ │
│  │                      │  │  │  pdf.js      │  │ │
│  │                      │  │  └──────────────┘  │ │
│  └─────────────────────┘  └────────────────────┘ │
│                                                   │
│  ┌──────────────────────────────────────────────┐ │
│  │              SQLite Database                  │ │
│  │  books | annotations | bookmarks | settings  │ │
│  └──────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────┘
```

### 4.1 Rust Backend Responsibilities
- File system operations (import, scan library folder, watch for changes)
- Metadata extraction (title, author, cover, language, publisher)
- SQLite database CRUD via `rusqlite`
- IPC command handlers (Tauri `#[tauri::command]`)
- Plugin loading and lifecycle management
- OPDS catalog client (for Calibre Content Server integration)
- Export annotations to Markdown/JSON

### 4.2 Svelte Frontend Responsibilities
- Library grid/list view with search and filtering
- Reader view wrapping foliate-js / pdf.js in an iframe or shadow DOM
- Toolbar: font settings, themes, TOC, bookmarks, annotations panel
- Settings page (appearance, library paths, keybindings)
- All UI state management via Svelte stores

### 4.3 Book Rendering Pipeline
```
User opens book
  → Rust reads file from disk, sends bytes to frontend via IPC
  → Frontend passes Blob to foliate-js
  → foliate-js parses format, produces paginated XHTML
  → WebKitGTK renders the content
  → User interactions (highlight, bookmark) → Svelte → IPC → Rust → SQLite
```

---

## 5. Database Schema

```sql
CREATE TABLE books (
    id          TEXT PRIMARY KEY,  -- UUID
    title       TEXT NOT NULL,
    author      TEXT,
    language    TEXT,
    publisher   TEXT,
    format      TEXT NOT NULL,     -- epub, mobi, azw3, pdf, etc.
    file_path   TEXT NOT NULL UNIQUE,
    file_hash   TEXT NOT NULL,     -- SHA-256 for dedup
    cover_path  TEXT,              -- extracted cover image path
    added_at    TEXT NOT NULL,     -- ISO 8601
    last_read   TEXT,
    metadata    TEXT               -- JSON blob for extra fields
);

CREATE TABLE reading_progress (
    book_id     TEXT PRIMARY KEY REFERENCES books(id),
    position    TEXT NOT NULL,     -- format-specific locator (CFI for EPUB, etc.)
    percentage  REAL,
    updated_at  TEXT NOT NULL
);

CREATE TABLE bookmarks (
    id          TEXT PRIMARY KEY,
    book_id     TEXT NOT NULL REFERENCES books(id),
    position    TEXT NOT NULL,
    label       TEXT,
    created_at  TEXT NOT NULL
);

CREATE TABLE annotations (
    id          TEXT PRIMARY KEY,
    book_id     TEXT NOT NULL REFERENCES books(id),
    type        TEXT NOT NULL,     -- highlight, note
    position    TEXT NOT NULL,     -- start locator
    end_pos     TEXT,              -- end locator
    text        TEXT,              -- highlighted text
    note        TEXT,              -- user's note
    color       TEXT DEFAULT 'yellow',
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);

CREATE TABLE collections (
    id          TEXT PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    created_at  TEXT NOT NULL
);

CREATE TABLE book_collections (
    book_id       TEXT REFERENCES books(id),
    collection_id TEXT REFERENCES collections(id),
    PRIMARY KEY (book_id, collection_id)
);

CREATE TABLE settings (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL
);
```

---

## 6. Feature Roadmap

### Phase 1 — MVP Reader (Weeks 1–4)

**Goal:** Open and read EPUB and MOBI files with basic navigation.

- [ ] **Project scaffolding** — `cargo create-tauri-app` with Svelte + TypeScript template
- [ ] **Integrate foliate-js** — embed as a git submodule or npm dependency
- [ ] **File open dialog** — Tauri's `dialog::FileDialogBuilder`, filter by supported extensions
- [ ] **Book rendering** — load file bytes via IPC, pass to foliate-js, render in WebView
- [ ] **Basic navigation** — next/prev page, TOC sidebar, go-to-page
- [ ] **Reading progress persistence** — save/restore position in SQLite on close/open
- [ ] **Font controls** — size, family (serif/sans/mono), line height, margins
- [ ] **Themes** — light, dark, sepia (CSS custom properties on the reader iframe)
- [ ] **Keyboard shortcuts** — arrow keys, space, Home/End, Escape to close book

### Phase 2 — Library Management (Weeks 5–7)

**Goal:** Import, organize, and browse a personal ebook library.

- [ ] **Library grid view** — cover thumbnails, title, author, reading progress bar
- [ ] **List view** — sortable table (title, author, date added, last read, format)
- [ ] **Import books** — drag-and-drop or file picker; copy to library folder, extract metadata & cover
- [ ] **Watch folder** — `notify` crate to auto-import new files from a configured directory
- [ ] **Search** — full-text search over title, author, metadata
- [ ] **Collections** — create, rename, delete; drag books into collections
- [ ] **Sort & filter** — by title, author, date, format, reading status
- [ ] **Delete book** — remove from library (with option to delete file)
- [ ] **Book detail view** — cover, metadata, reading stats, annotations count

### Phase 3 — Annotations & Bookmarks (Weeks 8–10)

**Goal:** Full annotation workflow matching Kindle's capabilities.

- [ ] **Text selection highlights** — yellow, blue, pink, orange colors
- [ ] **Add notes to highlights** — popup editor on highlight
- [ ] **Bookmarks** — toggle bookmark on current page, bookmark list in sidebar
- [ ] **Annotations panel** — sidebar listing all highlights and notes for current book
- [ ] **Annotations notebook** — library-wide view of all annotations across all books
- [ ] **Export annotations** — Markdown, JSON, plain text
- [ ] **Search within book** — full-text search with result navigation

### Phase 4 — Dictionary & Lookup (Weeks 11–12)

**Goal:** Tap/click a word for instant definitions.

- [ ] **Word lookup** — press-and-hold or double-click a word to trigger lookup
- [ ] **Offline dictionary** — bundle or download StarDict/DICT format dictionaries
- [ ] **Dictionary popup** — show definition in a floating panel near the selected word
- [ ] **Wikipedia summary** — optional online lookup via Wikipedia API
- [ ] **Translation** — optional integration with LibreTranslate (self-hosted) or similar
- [ ] **Vocabulary builder** — save looked-up words to a vocabulary list with spaced repetition

### Phase 5 — Advanced Reading Features (Weeks 13–15)

**Goal:** Power-user and quality-of-life features.

- [ ] **Reading statistics** — time spent reading, pages/day, streak tracking
- [ ] **Time-to-finish estimate** — based on reading speed
- [ ] **Continuous scroll mode** — alternative to paginated view
- [ ] **Two-column layout** — for wide screens
- [ ] **Custom CSS injection** — user-defined stylesheets per book
- [ ] **Fullscreen mode** — distraction-free reading with auto-hiding toolbar
- [ ] **Page turn animations** — slide or fade transitions (optional)
- [ ] **Image zoom** — click to zoom inline images
- [ ] **Footnote popups** — render EPUB footnotes as inline popups instead of navigation

### Phase 6 — Sync & Integration (Weeks 16–18)

**Goal:** Sync across devices and integrate with the ebook ecosystem.

- [ ] **Self-hosted sync server** — simple REST API (Rust/Actix-web or Go) for syncing reading progress, bookmarks, and annotations across multiple Linux machines
- [ ] **OPDS catalog browser** — connect to Calibre Content Server or other OPDS sources to browse and download books directly
- [ ] **Calibre integration** — import Calibre library database, respect Calibre metadata
- [ ] **Send-to-device** — accept files via a local HTTP endpoint (like KOReader's wireless transfer)
- [ ] **Import Kindle clippings** — parse `My Clippings.txt` from a Kindle device

### Phase 7 — Plugin System & Extensibility (Weeks 19–21)

**Goal:** Allow community extensions without modifying core code.

- [ ] **Plugin API** — define traits/interfaces for:
  - Format readers (add new file format support)
  - Dictionary providers
  - Sync backends
  - UI themes
  - Annotation exporters
- [ ] **Plugin discovery** — scan `~/.config/kindled/plugins/` for WASM or shared library plugins
- [ ] **Plugin manager UI** — enable/disable/configure plugins
- [ ] **Plugin sandboxing** — use WASM (via `wasmtime`) for safe third-party code execution

### Phase 8 — Polish & Distribution (Weeks 22–24)

**Goal:** Production-ready release.

- [ ] **Accessibility** — screen reader support (ATK/AT-SPI2), keyboard-only navigation, high contrast theme
- [ ] **Internationalization (i18n)** — UI translations via `fluent-rs` or JSON locale files
- [ ] **Performance** — profile and optimize large library loading, book rendering, memory usage
- [ ] **Error handling** — graceful handling of corrupt files, missing fonts, DB errors
- [ ] **Auto-update** — Tauri's built-in updater with GitHub Releases as backend
- [ ] **CI/CD pipeline** — GitHub Actions: lint, test, build .deb, .rpm, AppImage, Flatpak
- [ ] **Flatpak manifest** — for Flathub submission
- [ ] **Landing page & docs** — project website, user guide, contribution guide

---

## 7. Project Structure

```
kindle-app-linux/
├── src-tauri/                  # Rust backend
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs             # Tauri entry point
│   │   ├── lib.rs              # Module exports
│   │   ├── commands/           # IPC command handlers
│   │   │   ├── mod.rs
│   │   │   ├── books.rs        # Import, list, delete books
│   │   │   ├── reading.rs      # Progress, position
│   │   │   ├── annotations.rs  # CRUD for highlights, notes, bookmarks
│   │   │   └── settings.rs     # App settings
│   │   ├── db/                 # Database layer
│   │   │   ├── mod.rs
│   │   │   ├── schema.rs       # Table definitions, migrations
│   │   │   └── models.rs       # Rust structs for DB rows
│   │   ├── library/            # Library management
│   │   │   ├── mod.rs
│   │   │   ├── import.rs       # File import, metadata extraction
│   │   │   ├── watcher.rs      # Filesystem watcher
│   │   │   └── opds.rs         # OPDS client
│   │   ├── plugins/            # Plugin host
│   │   │   ├── mod.rs
│   │   │   └── loader.rs
│   │   └── utils.rs
│   ├── migrations/             # SQL migration files
│   └── tauri.conf.json
├── src/                        # Svelte frontend
│   ├── lib/
│   │   ├── components/
│   │   │   ├── Library.svelte
│   │   │   ├── BookCard.svelte
│   │   │   ├── Reader.svelte
│   │   │   ├── Toolbar.svelte
│   │   │   ├── TOCSidebar.svelte
│   │   │   ├── AnnotationPanel.svelte
│   │   │   ├── DictionaryPopup.svelte
│   │   │   └── Settings.svelte
│   │   ├── stores/
│   │   │   ├── library.ts      # Book list, collections
│   │   │   ├── reader.ts       # Current book state, position
│   │   │   ├── annotations.ts  # Highlights, notes, bookmarks
│   │   │   └── settings.ts     # Theme, font, preferences
│   │   ├── services/
│   │   │   ├── tauri-ipc.ts    # Typed wrappers around invoke()
│   │   │   └── reader-bridge.ts # Interface between Svelte and foliate-js
│   │   └── types/
│   │       └── index.ts        # Shared TypeScript types
│   ├── routes/                 # SvelteKit-style routing (if used) or simple router
│   ├── app.html
│   ├── app.css
│   └── main.ts
├── vendor/
│   └── foliate-js/             # Git submodule
├── static/
│   ├── dictionaries/           # Bundled dictionary files
│   └── icons/
├── tests/
│   ├── rust/                   # Rust unit & integration tests
│   └── frontend/               # Vitest / Playwright tests
├── .github/
│   └── workflows/
│       ├── ci.yml              # Lint + test on PR
│       └── release.yml         # Build & publish artifacts
├── package.json
├── vite.config.ts
├── svelte.config.js
├── tsconfig.json
├── Plan.md                     # This file
└── README.md
```

---

## 8. Key Dependencies

### Rust (Cargo.toml)
```toml
[dependencies]
tauri = { version = "2", features = ["dialog", "fs", "shell", "updater"] }
rusqlite = { version = "0.32", features = ["bundled"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4"] }
sha2 = "0.10"              # File hashing
notify = "7"                # Filesystem watcher
reqwest = { version = "0.12", features = ["json"] }  # OPDS/HTTP client
quick-xml = "0.37"          # OPDS XML parsing
chrono = { version = "0.4", features = ["serde"] }
thiserror = "2"
tokio = { version = "1", features = ["full"] }
```

### Frontend (package.json)
```json
{
  "dependencies": {
    "@tauri-apps/api": "^2.0",
    "svelte": "^5.0"
  },
  "devDependencies": {
    "@sveltejs/vite-plugin-svelte": "^4.0",
    "typescript": "^5.5",
    "vite": "^6.0",
    "tailwindcss": "^4.0",
    "vitest": "^3.0",
    "@playwright/test": "^1.48"
  }
}
```

---

## 9. Development Setup

```bash
# Prerequisites
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (via nvm)
nvm install 22

# Create project
npm create tauri-app@latest kindle-app-linux -- --template svelte-ts
cd kindle-app-linux

# Add foliate-js
git submodule add https://github.com/johnfactotum/foliate-js.git vendor/foliate-js

# Install dependencies
npm install
cd src-tauri && cargo build
cd ..

# Run in dev mode
npm run tauri dev
```

---

## 10. Testing Strategy

| Level | Tool | What to Test |
|-------|------|-------------|
| **Rust unit tests** | `cargo test` | DB operations, metadata extraction, file hashing, plugin loading |
| **Rust integration tests** | `cargo test` | Full import pipeline, IPC round-trips |
| **Frontend unit tests** | Vitest | Svelte component rendering, store logic, IPC mocks |
| **E2E tests** | Playwright + Tauri driver | Open book, navigate pages, create highlight, verify persistence |
| **Format tests** | Dedicated test corpus | One sample file per format; verify render, TOC, metadata extraction |
| **Performance benchmarks** | `criterion` (Rust) | Library load time, book open time, memory usage |

---

## 11. Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|-----------|
| **WebKitGTK version inconsistency** across distros | Rendering differences | Test on Ubuntu LTS, Fedora, Arch. Document minimum WebKitGTK version. |
| **foliate-js API changes** | Breaking upgrades | Pin to a specific commit via submodule. Contribute upstream. |
| **KFX format complexity** | Can't support newest Kindle books | Defer to plugin system. Focus on AZW3/EPUB which cover most content. |
| **DRM expectations** | Users expect to read purchased Kindle books | Clear documentation. Plugin architecture for user-installed tools. |
| **Performance with large libraries** (10K+ books) | Slow UI, high memory | Virtual scrolling, lazy cover loading, DB indexing, pagination. |
| **Amazon API/TOS changes** | Sync features break | Self-hosted sync as primary. Amazon integration is optional/community-maintained. |

---

## 12. Success Criteria

1. Opens EPUB, MOBI, and AZW3 files in under 2 seconds
2. Memory usage under 200MB while reading
3. Binary size under 15MB (AppImage)
4. All annotations persist across app restarts
5. Usable entirely via keyboard
6. Installable via .deb, AppImage, and Flatpak
7. Accessible to screen reader users

---

## 13. Reference Projects

- **Foliate** — https://github.com/johnfactotum/foliate (GTK4 + GJS reader, foliate-js)
- **foliate-js** — https://github.com/johnfactotum/foliate-js (ebook rendering library)
- **Calibre** — https://github.com/kovidgoyal/calibre (Python + Qt, comprehensive format support)
- **KOReader** — https://github.com/koreader/koreader (Lua + C, e-ink optimized)
- **Readest** — cross-platform reader with Tauri-like approach
- **epub.js** — https://github.com/futurepress/epub.js (EPUB rendering in browser)
