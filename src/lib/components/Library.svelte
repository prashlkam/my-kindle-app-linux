<script lang="ts">
  import BookCard from "./BookCard.svelte";
  import { sortedBooks, loadLibrary, importBookFile, performSearch, searchQuery } from "$lib/stores/library";
  import { settings, updateSetting } from "$lib/stores/settings";
  import { openBook } from "$lib/stores/reader";
  import { open } from "@tauri-apps/plugin-dialog";
  import type { Book } from "$lib/types";

  let { onsettings }: { onsettings: () => void } = $props();

  let searchInput = $state("");
  let dragOver = $state(false);

  async function handleImport() {
    const files = await open({
      multiple: true,
      filters: [
        {
          name: "Ebooks",
          extensions: ["epub", "mobi", "azw", "azw3", "fb2", "cbz", "cbr", "pdf"],
        },
      ],
    });
    if (files) {
      const paths = Array.isArray(files) ? files : [files];
      for (const p of paths) {
        if (p) await importBookFile(p);
      }
    }
  }

  function handleSearch() {
    performSearch(searchInput);
  }

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    dragOver = false;
    const files = e.dataTransfer?.files;
    if (files) {
      for (let i = 0; i < files.length; i++) {
        const path = (files[i] as unknown as { path?: string })?.path;
        if (path) importBookFile(path);
      }
    }
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    dragOver = true;
  }

  function handleDragLeave() {
    dragOver = false;
  }

  function toggleViewMode() {
    updateSetting("view_mode", $settings.view_mode === "grid" ? "list" : "grid");
  }

  function cycleSortBy() {
    const options: Array<"title" | "author" | "added_at" | "last_read"> = [
      "title", "author", "added_at", "last_read",
    ];
    const idx = options.indexOf($settings.sort_by);
    updateSetting("sort_by", options[(idx + 1) % options.length]);
  }

  function handleBookClick(book: Book) {
    openBook(book);
  }
</script>

<div
  class="library"
  class:drag-over={dragOver}
  ondrop={handleDrop}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  role="main"
>
  <!-- Header -->
  <header class="library-header">
    <div class="header-left">
      <h1 class="app-title">Kindled</h1>
      <span class="book-count">{$sortedBooks.length} books</span>
    </div>

    <div class="header-center">
      <div class="search-box">
        <svg class="search-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8" /><path d="m21 21-4.35-4.35" />
        </svg>
        <input
          type="text"
          placeholder="Search library..."
          bind:value={searchInput}
          oninput={handleSearch}
        />
        {#if searchInput}
          <button class="clear-search" onclick={() => { searchInput = ""; performSearch(""); }}>
            &times;
          </button>
        {/if}
      </div>
    </div>

    <div class="header-right">
      <button class="header-btn" onclick={cycleSortBy} title="Sort by: {$settings.sort_by}">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M3 6h18M3 12h12M3 18h6" />
        </svg>
      </button>
      <button class="header-btn" onclick={toggleViewMode} title="{$settings.view_mode === 'grid' ? 'List view' : 'Grid view'}">
        {#if $settings.view_mode === "grid"}
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="3" width="7" height="7" /><rect x="14" y="3" width="7" height="7" />
            <rect x="3" y="14" width="7" height="7" /><rect x="14" y="14" width="7" height="7" />
          </svg>
        {:else}
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M8 6h13M8 12h13M8 18h13M3 6h.01M3 12h.01M3 18h.01" />
          </svg>
        {/if}
      </button>
      <button class="header-btn import-btn" onclick={handleImport}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 15v4a2 2 0 01-2 2H5a2 2 0 01-2-2v-4M7 10l5 5 5-5M12 15V3" />
        </svg>
        Import
      </button>
      <button class="header-btn" onclick={onsettings} title="Settings">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="3" />
          <path d="M12 1v2m0 18v2M4.22 4.22l1.42 1.42m12.72 12.72 1.42 1.42M1 12h2m18 0h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42" />
        </svg>
      </button>
    </div>
  </header>

  <!-- Book Grid/List -->
  <div class="library-content">
    {#if $sortedBooks.length === 0}
      <div class="empty-state">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="empty-icon">
          <path d="M4 19.5A2.5 2.5 0 016.5 17H20" />
          <path d="M6.5 2H20v20H6.5A2.5 2.5 0 014 19.5v-15A2.5 2.5 0 016.5 2z" />
        </svg>
        <h2>Your library is empty</h2>
        <p>Import ebooks or drag & drop files here</p>
        <button class="import-cta" onclick={handleImport}>Import Books</button>
      </div>
    {:else}
      <div class="book-{$settings.view_mode}">
        {#each $sortedBooks as book (book.id)}
          <BookCard
            {book}
            viewMode={$settings.view_mode}
            onclick={() => handleBookClick(book)}
          />
        {/each}
      </div>
    {/if}
  </div>
</div>

<style>
  .library {
    height: 100%;
    display: flex;
    flex-direction: column;
    transition: background-color 0.2s;
  }

  .library.drag-over {
    background-color: var(--bg-tertiary);
  }

  .library-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 20px;
    border-bottom: 1px solid var(--border);
    background-color: var(--bg-secondary);
    gap: 16px;
  }

  .header-left {
    display: flex;
    align-items: baseline;
    gap: 12px;
    flex-shrink: 0;
  }

  .app-title {
    font-size: 20px;
    font-weight: 700;
    color: var(--accent);
  }

  .book-count {
    font-size: 13px;
    color: var(--text-muted);
  }

  .header-center {
    flex: 1;
    max-width: 400px;
  }

  .search-box {
    position: relative;
    display: flex;
    align-items: center;
  }

  .search-icon {
    position: absolute;
    left: 10px;
    width: 16px;
    height: 16px;
    color: var(--text-muted);
    pointer-events: none;
  }

  .search-box input {
    width: 100%;
    padding: 8px 32px 8px 34px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg-primary);
    color: var(--text-primary);
    font-size: 14px;
    outline: none;
    transition: border-color 0.2s;
  }

  .search-box input:focus {
    border-color: var(--accent);
  }

  .clear-search {
    position: absolute;
    right: 8px;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    font-size: 18px;
    padding: 0 4px;
  }

  .header-right {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
  }

  .header-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--bg-primary);
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 13px;
    transition: all 0.15s;
  }

  .header-btn:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .header-btn svg {
    width: 18px;
    height: 18px;
  }

  .import-btn {
    background: var(--accent);
    color: white;
    border-color: var(--accent);
  }

  .import-btn:hover {
    background: var(--accent-hover);
  }

  .library-content {
    flex: 1;
    overflow-y: auto;
    padding: 20px;
  }

  .book-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
    gap: 20px;
  }

  .book-list {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: var(--text-muted);
    gap: 12px;
  }

  .empty-icon {
    width: 64px;
    height: 64px;
    opacity: 0.4;
  }

  .empty-state h2 {
    font-size: 20px;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .empty-state p {
    font-size: 14px;
  }

  .import-cta {
    margin-top: 8px;
    padding: 10px 24px;
    background: var(--accent);
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    cursor: pointer;
    transition: background-color 0.15s;
  }

  .import-cta:hover {
    background: var(--accent-hover);
  }
</style>
