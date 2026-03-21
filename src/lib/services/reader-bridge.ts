/**
 * Bridge between Svelte app and foliate-js running in an iframe.
 * Communicates via postMessage to the isolated reader iframe.
 */

export interface ReaderMessage {
  type: string;
  payload?: unknown;
}

export interface RelocationEvent {
  fraction: number;
  location: {
    current: number;
    next: number;
    total: number;
  };
  cfi?: string;
  section: {
    current: number;
    total: number;
  };
  time?: {
    section: number;
    total: number;
  };
}

export interface SelectionEvent {
  text: string;
  cfi: string;
  position: { x: number; y: number };
}

export class ReaderBridge {
  private iframe: HTMLIFrameElement | null = null;
  private listeners: Map<string, Set<(data: unknown) => void>> = new Map();
  private messageHandler: ((e: MessageEvent) => void) | null = null;

  init(iframe: HTMLIFrameElement) {
    this.iframe = iframe;
    this.messageHandler = (e: MessageEvent) => {
      if (e.source !== this.iframe?.contentWindow) return;
      const { type, payload } = e.data as ReaderMessage;
      const handlers = this.listeners.get(type);
      if (handlers) {
        handlers.forEach((fn) => fn(payload));
      }
    };
    window.addEventListener("message", this.messageHandler);
  }

  destroy() {
    if (this.messageHandler) {
      window.removeEventListener("message", this.messageHandler);
    }
    this.listeners.clear();
    this.iframe = null;
  }

  private send(type: string, payload?: unknown) {
    this.iframe?.contentWindow?.postMessage({ type, payload }, "*");
  }

  on(type: string, handler: (data: unknown) => void) {
    if (!this.listeners.has(type)) {
      this.listeners.set(type, new Set());
    }
    this.listeners.get(type)!.add(handler);
    return () => this.listeners.get(type)?.delete(handler);
  }

  openBook(data: ArrayBuffer, fileName: string) {
    this.send("open-book", { data, fileName });
  }

  goToPosition(cfi: string) {
    this.send("go-to", { cfi });
  }

  goNext() {
    this.send("go-next");
  }

  goPrev() {
    this.send("go-prev");
  }

  setFontSize(size: number) {
    this.send("set-style", { fontSize: size });
  }

  setFontFamily(family: string) {
    this.send("set-style", { fontFamily: family });
  }

  setLineHeight(height: number) {
    this.send("set-style", { lineHeight: height });
  }

  setMargin(margin: number) {
    this.send("set-style", { margin });
  }

  setTheme(theme: string) {
    this.send("set-theme", { theme });
  }

  addHighlight(cfi: string, color: string) {
    this.send("add-highlight", { cfi, color });
  }

  removeHighlight(cfi: string) {
    this.send("remove-highlight", { cfi });
  }

  search(query: string) {
    this.send("search", { query });
  }

  getToc() {
    this.send("get-toc");
  }

  goToHref(href: string) {
    this.send("go-to-href", { href });
  }

  setScrollMode(enabled: boolean) {
    this.send("set-scroll-mode", { enabled });
  }
}

export const readerBridge = new ReaderBridge();
