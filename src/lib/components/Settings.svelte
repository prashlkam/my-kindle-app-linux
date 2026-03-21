<script lang="ts">
  import { settings, updateSetting } from "$lib/stores/settings";

  let { onclose }: { onclose: () => void } = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="settings-overlay" onclick={onclose}>
  <div class="settings-panel" onclick={(e: MouseEvent) => e.stopPropagation()}>
    <div class="settings-header">
      <h2>Settings</h2>
      <button class="close-btn" onclick={onclose}>&times;</button>
    </div>

    <div class="settings-body">
      <!-- Appearance -->
      <section class="settings-section">
        <h3>Appearance</h3>

        <div class="setting-row">
          <label>Theme</label>
          <select
            value={$settings.theme}
            onchange={(e) => updateSetting("theme", (e.target as HTMLSelectElement).value as "light" | "dark" | "sepia")}
          >
            <option value="light">Light</option>
            <option value="dark">Dark</option>
            <option value="sepia">Sepia</option>
          </select>
        </div>

        <div class="setting-row">
          <label>Default View</label>
          <select
            value={$settings.view_mode}
            onchange={(e) => updateSetting("view_mode", (e.target as HTMLSelectElement).value as "grid" | "list")}
          >
            <option value="grid">Grid</option>
            <option value="list">List</option>
          </select>
        </div>
      </section>

      <!-- Reading -->
      <section class="settings-section">
        <h3>Reading</h3>

        <div class="setting-row">
          <label>Font Family</label>
          <select
            value={$settings.font_family}
            onchange={(e) => updateSetting("font_family", (e.target as HTMLSelectElement).value)}
          >
            <option value="serif">Serif</option>
            <option value="sans-serif">Sans-serif</option>
            <option value="monospace">Monospace</option>
          </select>
        </div>

        <div class="setting-row">
          <label>Font Size</label>
          <div class="range-control">
            <input
              type="range"
              min="12"
              max="32"
              step="1"
              value={$settings.font_size}
              oninput={(e) => updateSetting("font_size", parseInt((e.target as HTMLInputElement).value))}
            />
            <span>{$settings.font_size}px</span>
          </div>
        </div>

        <div class="setting-row">
          <label>Line Height</label>
          <div class="range-control">
            <input
              type="range"
              min="1.0"
              max="2.5"
              step="0.1"
              value={$settings.line_height}
              oninput={(e) => updateSetting("line_height", parseFloat((e.target as HTMLInputElement).value))}
            />
            <span>{$settings.line_height}</span>
          </div>
        </div>

        <div class="setting-row">
          <label>Margin</label>
          <div class="range-control">
            <input
              type="range"
              min="10"
              max="100"
              step="5"
              value={$settings.margin}
              oninput={(e) => updateSetting("margin", parseInt((e.target as HTMLInputElement).value))}
            />
            <span>{$settings.margin}px</span>
          </div>
        </div>
      </section>

      <!-- Library -->
      <section class="settings-section">
        <h3>Library</h3>

        <div class="setting-row">
          <label>Sort By</label>
          <select
            value={$settings.sort_by}
            onchange={(e) => updateSetting("sort_by", (e.target as HTMLSelectElement).value as any)}
          >
            <option value="title">Title</option>
            <option value="author">Author</option>
            <option value="added_at">Date Added</option>
            <option value="last_read">Last Read</option>
          </select>
        </div>

        <div class="setting-row">
          <label>Sort Order</label>
          <select
            value={$settings.sort_order}
            onchange={(e) => updateSetting("sort_order", (e.target as HTMLSelectElement).value as "asc" | "desc")}
          >
            <option value="asc">Ascending</option>
            <option value="desc">Descending</option>
          </select>
        </div>
      </section>

      <!-- About -->
      <section class="settings-section">
        <h3>About</h3>
        <p class="about-text">
          <strong>Kindled</strong> v0.1.0<br />
          A native Linux ebook reader for Kindle formats.<br />
          Supports EPUB, MOBI, AZW3, FB2, CBZ, PDF.
        </p>
      </section>
    </div>
  </div>
</div>

<style>
  .settings-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 500;
  }

  .settings-panel {
    width: 480px;
    max-height: 80vh;
    background: var(--bg-primary);
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .settings-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border);
  }

  .settings-header h2 {
    font-size: 18px;
    font-weight: 600;
  }

  .close-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 24px;
    cursor: pointer;
    padding: 0 4px;
  }

  .close-btn:hover {
    color: var(--text-primary);
  }

  .settings-body {
    flex: 1;
    overflow-y: auto;
    padding: 16px 20px;
  }

  .settings-section {
    margin-bottom: 24px;
  }

  .settings-section h3 {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 12px;
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 0;
  }

  .setting-row label {
    font-size: 14px;
    color: var(--text-primary);
  }

  .setting-row select {
    padding: 6px 10px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    font-size: 13px;
    cursor: pointer;
  }

  .range-control {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .range-control input[type="range"] {
    width: 140px;
    accent-color: var(--accent);
  }

  .range-control span {
    font-size: 13px;
    color: var(--text-secondary);
    min-width: 40px;
    text-align: right;
  }

  .about-text {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.6;
  }
</style>
