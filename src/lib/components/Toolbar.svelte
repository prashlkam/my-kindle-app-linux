<script lang="ts">
  import {
    currentBook,
    showToc,
    showAnnotations,
    toggleBookmark,
    bookmarks,
    currentPosition,
  } from "$lib/stores/reader";
  import { settings, updateSetting } from "$lib/stores/settings";
  import { readerBridge } from "$lib/services/reader-bridge";

  let { onclose }: { onclose: () => void } = $props();

  let showFontMenu = $state(false);

  $effect(() => {
    const handleClick = () => { showFontMenu = false; };
    document.addEventListener("click", handleClick);
    return () => document.removeEventListener("click", handleClick);
  });

  function changeFontSize(delta: number) {
    const newSize = Math.max(12, Math.min(32, $settings.font_size + delta));
    updateSetting("font_size", newSize);
    readerBridge.setFontSize(newSize);
  }

  function changeFontFamily(family: string) {
    updateSetting("font_family", family);
    readerBridge.setFontFamily(family);
  }

  function changeTheme(theme: "light" | "dark" | "sepia") {
    updateSetting("theme", theme);
    readerBridge.setTheme(theme);
  }

  function isBookmarked(): boolean {
    const pos = $currentPosition;
    if (!pos) return false;
    return $bookmarks.some((b) => b.position === pos);
  }
</script>

<header class="toolbar">
  <div class="toolbar-left">
    <button class="tb-btn" onclick={onclose} title="Back to library (Esc)">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M19 12H5M12 19l-7-7 7-7" />
      </svg>
    </button>
    <div class="book-title-bar">
      <span class="toolbar-title">{$currentBook?.title || ""}</span>
      <span class="toolbar-author">{$currentBook?.author || ""}</span>
    </div>
  </div>

  <div class="toolbar-right">
    <!-- TOC toggle -->
    <button
      class="tb-btn"
      class:active={$showToc}
      onclick={() => showToc.update((v) => !v)}
      title="Table of Contents (T)"
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M3 12h18M3 6h18M3 18h18" />
      </svg>
    </button>

    <!-- Bookmark -->
    <button
      class="tb-btn"
      class:active={isBookmarked()}
      onclick={toggleBookmark}
      title="Bookmark"
    >
      <svg viewBox="0 0 24 24" fill={isBookmarked() ? "currentColor" : "none"} stroke="currentColor" stroke-width="2">
        <path d="M19 21l-7-5-7 5V5a2 2 0 012-2h10a2 2 0 012 2z" />
      </svg>
    </button>

    <!-- Annotations -->
    <button
      class="tb-btn"
      class:active={$showAnnotations}
      onclick={() => showAnnotations.update((v) => !v)}
      title="Annotations"
    >
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M11 4H4a2 2 0 00-2 2v14a2 2 0 002 2h14a2 2 0 002-2v-7" />
        <path d="M18.5 2.5a2.121 2.121 0 013 3L12 15l-4 1 1-4 9.5-9.5z" />
      </svg>
    </button>

    <!-- Font settings -->
    <div class="font-menu-container">
      <button
        class="tb-btn"
        onclick={(e: MouseEvent) => { e.stopPropagation(); showFontMenu = !showFontMenu; }}
        title="Font settings"
      >
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <text x="4" y="18" font-size="16" fill="currentColor" stroke="none" font-family="serif">Aa</text>
        </svg>
      </button>

      {#if showFontMenu}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div class="font-dropdown" onclick={(e: MouseEvent) => e.stopPropagation()}>
          <div class="font-section">
            <label>Size</label>
            <div class="font-size-controls">
              <button onclick={() => changeFontSize(-2)}>A-</button>
              <span>{$settings.font_size}px</span>
              <button onclick={() => changeFontSize(2)}>A+</button>
            </div>
          </div>

          <div class="font-section">
            <label>Font</label>
            <div class="font-family-controls">
              {#each ["serif", "sans-serif", "monospace"] as family}
                <button
                  class:selected={$settings.font_family === family}
                  onclick={() => changeFontFamily(family)}
                  style="font-family: {family}"
                >
                  {family === "sans-serif" ? "Sans" : family === "monospace" ? "Mono" : "Serif"}
                </button>
              {/each}
            </div>
          </div>

          <div class="font-section">
            <label>Theme</label>
            <div class="theme-controls">
              <button class="theme-btn theme-light" class:selected={$settings.theme === "light"} onclick={() => changeTheme("light")}>
                Light
              </button>
              <button class="theme-btn theme-sepia" class:selected={$settings.theme === "sepia"} onclick={() => changeTheme("sepia")}>
                Sepia
              </button>
              <button class="theme-btn theme-dark" class:selected={$settings.theme === "dark"} onclick={() => changeTheme("dark")}>
                Dark
              </button>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Search -->
    <button class="tb-btn" title="Search in book">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="11" cy="11" r="8" /><path d="m21 21-4.35-4.35" />
      </svg>
    </button>
  </div>
</header>

<style>
  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    height: var(--toolbar-height);
    border-bottom: 1px solid var(--border);
    background: var(--bg-secondary);
    flex-shrink: 0;
  }

  .toolbar-left {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 0;
  }

  .book-title-bar {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .toolbar-title {
    font-size: 14px;
    font-weight: 600;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .toolbar-author {
    font-size: 11px;
    color: var(--text-muted);
  }

  .toolbar-right {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .tb-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border: none;
    border-radius: 6px;
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.15s;
  }

  .tb-btn:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .tb-btn.active {
    background: var(--accent);
    color: white;
  }

  .tb-btn svg {
    width: 20px;
    height: 20px;
  }

  .font-menu-container {
    position: relative;
  }

  .font-dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 4px;
    padding: 12px;
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: 8px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
    z-index: 200;
    min-width: 220px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .font-section {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .font-section label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .font-size-controls {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .font-size-controls button {
    padding: 4px 12px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    cursor: pointer;
    font-size: 14px;
  }

  .font-size-controls span {
    font-size: 14px;
    font-weight: 500;
  }

  .font-family-controls {
    display: flex;
    gap: 6px;
  }

  .font-family-controls button {
    flex: 1;
    padding: 6px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    cursor: pointer;
    font-size: 13px;
    transition: all 0.15s;
  }

  .font-family-controls button.selected {
    border-color: var(--accent);
    background: var(--accent);
    color: white;
  }

  .theme-controls {
    display: flex;
    gap: 6px;
  }

  .theme-btn {
    flex: 1;
    padding: 6px;
    border: 2px solid var(--border);
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
    text-align: center;
    transition: all 0.15s;
  }

  .theme-btn.selected {
    border-color: var(--accent);
  }

  .theme-light {
    background: #ffffff;
    color: #1a1a1a;
  }

  .theme-sepia {
    background: #f4ecd8;
    color: #3d2b1f;
  }

  .theme-dark {
    background: #1a1a2e;
    color: #e0e0e0;
  }
</style>
