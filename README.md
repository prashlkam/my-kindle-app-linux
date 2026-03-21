# Kindled

A native Linux ebook reader for Kindle formats, built with Tauri 2 + Svelte 5 + Rust.

## Supported Formats

- **EPUB** 2/3
- **MOBI** (DRM-free)
- **AZW3 / KF8** (DRM-free)
- **FB2**
- **CBZ/CBR**
- **PDF**

## Features

- Library management with grid/list view, search, and collections
- Reader with paginated view, TOC navigation, and keyboard shortcuts
- Highlights (4 colors), notes, and bookmarks with persistence
- Font controls (family, size, line height, margins)
- Light, dark, and sepia themes
- Dictionary lookup (online via Free Dictionary API)
- Export annotations to Markdown/JSON
- Plugin architecture for extensibility

## Prerequisites

### Linux
```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

### All platforms
- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) 18+

## Setup

```bash
# Clone with submodules
git clone --recurse-submodules https://github.com/your-username/kindle-app-linux.git
cd kindle-app-linux

# If you already cloned without submodules:
git submodule add https://github.com/johnfactotum/foliate-js.git vendor/foliate-js

# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Arrow Right` / `Space` | Next page |
| `Arrow Left` | Previous page |
| `T` | Toggle table of contents |
| `Escape` | Close book / settings |

## Project Structure

```
kindle-app-linux/
├── src-tauri/          # Rust backend (Tauri)
│   ├── src/
│   │   ├── commands/   # IPC command handlers
│   │   ├── db/         # SQLite database layer
│   │   ├── library/    # Book import & management
│   │   └── plugins/    # Plugin system (future)
├── src/                # Svelte frontend
│   ├── lib/
│   │   ├── components/ # UI components
│   │   ├── stores/     # Svelte stores
│   │   ├── services/   # IPC & reader bridge
│   │   └── types/      # TypeScript types
├── static/             # Static assets & reader iframe
└── vendor/             # foliate-js submodule
```

## License

MIT
