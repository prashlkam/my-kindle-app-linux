# my Kindle app linux

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

### 1. Install Rust

Rust is required to build the Tauri backend. If you don't have Rust installed, follow the steps for your platform:

#### Linux / macOS

Open a terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When prompted, select **option 1** (default installation). This installs `rustup`, `rustc`, and `cargo`.

After installation, load Rust into your current shell session:

```bash
source "$HOME/.cargo/env"
```

> This line is also added to your `~/.bashrc` (or `~/.zshrc`) automatically, so future terminal sessions will have Rust available.

#### Windows

1. Download and run the [Rust installer (rustup-init.exe)](https://win.rustup.rs/).
2. If prompted to install **Visual Studio C++ Build Tools**, follow the instructions — they are required for compiling native code on Windows.
3. Follow the on-screen prompts and select the default installation.
4. Close and reopen your terminal after installation.

#### Verify the installation

```bash
rustc --version
cargo --version
```

You should see version output for both commands (e.g., `rustc 1.XX.X`). If not, ensure `~/.cargo/bin` is in your `PATH`.

#### Update Rust (if already installed)

```bash
rustup update
```

### 2. Install Node.js

[Node.js](https://nodejs.org/) **18+** is required for the Svelte frontend.

Download the LTS version from the official site, or install via a version manager:

```bash
# Using nvm (Linux/macOS)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
nvm install --lts

# Using fnm (cross-platform alternative)
cargo install fnm
fnm install --lts
```

Verify:

```bash
node --version
npm --version
```

### 3. Install system dependencies (Linux only)

Tauri requires several system libraries on Linux. Install them with:

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

> **Fedora/RHEL:** `sudo dnf install webkit2gtk4.1-devel openssl-devel gtk3-devel libappindicator-gtk3-devel librsvg2-devel`
>
> **Arch:** `sudo pacman -S webkit2gtk-4.1 base-devel curl wget openssl gtk3 libappindicator-gtk3 librsvg`

### 4. Install the Tauri CLI

The Tauri CLI is used to run and build the application. Install it globally via Cargo:

```bash
cargo install tauri-cli --version "^2"
```

Or use it through npm (already configured in this project's scripts):

```bash
npm install
npx tauri --version
```

Verify:

```bash
cargo tauri --version
```

You should see output like `tauri-cli 2.x.x`.

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

### Troubleshooting

- **`cargo: command not found`** — Rust is not in your PATH. Run `source "$HOME/.cargo/env"` or restart your terminal.
- **`error: linker 'cc' not found`** — Install build tools: `sudo apt install build-essential` (Ubuntu) or install Visual Studio C++ Build Tools (Windows).
- **WebKit errors on Linux** — Ensure you installed the system dependencies from step 3 above.
- **`tauri-cli` version mismatch** — This project uses Tauri 2. Run `cargo install tauri-cli --version "^2"` to get the correct version.

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

GPL - Ver 3.0
