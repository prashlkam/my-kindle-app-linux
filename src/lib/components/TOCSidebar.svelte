<script lang="ts">
  import { toc, showToc } from "$lib/stores/reader";
  import { readerBridge } from "$lib/services/reader-bridge";
  import type { TocItem } from "$lib/types";

  function goTo(item: TocItem) {
    readerBridge.goToHref(item.href);
  }
</script>

<aside class="toc-sidebar">
  <div class="toc-header">
    <h2>Contents</h2>
    <button class="close-btn" onclick={() => showToc.set(false)}>
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M18 6L6 18M6 6l12 12" />
      </svg>
    </button>
  </div>

  <nav class="toc-list">
    {#each $toc as item}
      <button class="toc-item" onclick={() => goTo(item)}>
        {item.label}
      </button>
      {#if item.subitems}
        {#each item.subitems as sub}
          <button class="toc-item toc-subitem" onclick={() => goTo(sub)}>
            {sub.label}
          </button>
        {/each}
      {/if}
    {/each}

    {#if $toc.length === 0}
      <p class="toc-empty">No table of contents available</p>
    {/if}
  </nav>
</aside>

<style>
  .toc-sidebar {
    width: var(--sidebar-width);
    height: 100%;
    border-right: 1px solid var(--border);
    background: var(--bg-secondary);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .toc-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border);
  }

  .toc-header h2 {
    font-size: 14px;
    font-weight: 600;
  }

  .close-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    border-radius: 4px;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
  }

  .close-btn:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .close-btn svg {
    width: 16px;
    height: 16px;
  }

  .toc-list {
    flex: 1;
    overflow-y: auto;
    padding: 8px 0;
  }

  .toc-item {
    display: block;
    width: 100%;
    padding: 8px 16px;
    border: none;
    background: none;
    color: var(--text-primary);
    text-align: left;
    font-size: 13px;
    cursor: pointer;
    line-height: 1.4;
    transition: background-color 0.1s;
  }

  .toc-item:hover {
    background: var(--bg-tertiary);
  }

  .toc-subitem {
    padding-left: 32px;
    font-size: 12px;
    color: var(--text-secondary);
  }

  .toc-empty {
    padding: 16px;
    text-align: center;
    color: var(--text-muted);
    font-size: 13px;
  }
</style>
