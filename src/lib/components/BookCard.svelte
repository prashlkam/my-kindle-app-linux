<script lang="ts">
  import type { Book } from "$lib/types";
  import { removeBook } from "$lib/stores/library";

  let { book, viewMode, onclick }: {
    book: Book;
    viewMode: "grid" | "list";
    onclick: () => void;
  } = $props();

  let showMenu = $state(false);

  function formatDate(dateStr: string | null): string {
    if (!dateStr) return "";
    const d = new Date(dateStr);
    return d.toLocaleDateString(undefined, { month: "short", day: "numeric", year: "numeric" });
  }

  function getFormatBadge(format: string): string {
    return format.toUpperCase();
  }

  function handleContextMenu(e: MouseEvent) {
    e.preventDefault();
    showMenu = !showMenu;
  }

  async function handleDelete() {
    showMenu = false;
    if (confirm(`Delete "${book.title}" from your library?`)) {
      await removeBook(book.id, false);
    }
  }
</script>

{#if viewMode === "grid"}
  <button
    class="book-card-grid"
    onclick={onclick}
    oncontextmenu={handleContextMenu}
  >
    <div class="cover-container">
      {#if book.cover_path}
        <img src="asset://localhost/{book.cover_path}" alt="{book.title} cover" class="cover-img" />
      {:else}
        <div class="cover-placeholder">
          <span class="cover-title">{book.title}</span>
          <span class="cover-author">{book.author}</span>
        </div>
      {/if}
      <span class="format-badge">{getFormatBadge(book.format)}</span>
    </div>
    <div class="book-info">
      <h3 class="book-title" title={book.title}>{book.title}</h3>
      <p class="book-author">{book.author || "Unknown"}</p>
    </div>

    {#if showMenu}
      <div class="context-menu">
        <button onclick={handleDelete}>Delete from library</button>
      </div>
    {/if}
  </button>
{:else}
  <button
    class="book-card-list"
    onclick={onclick}
    oncontextmenu={handleContextMenu}
  >
    <div class="list-cover">
      {#if book.cover_path}
        <img src="asset://localhost/{book.cover_path}" alt="" class="list-cover-img" />
      {:else}
        <div class="list-cover-placeholder">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
            <path d="M4 19.5A2.5 2.5 0 016.5 17H20" />
            <path d="M6.5 2H20v20H6.5A2.5 2.5 0 014 19.5v-15A2.5 2.5 0 016.5 2z" />
          </svg>
        </div>
      {/if}
    </div>
    <div class="list-info">
      <h3 class="book-title">{book.title}</h3>
      <p class="book-author">{book.author || "Unknown"}</p>
    </div>
    <span class="list-format">{getFormatBadge(book.format)}</span>
    <span class="list-date">{formatDate(book.added_at)}</span>

    {#if showMenu}
      <div class="context-menu">
        <button onclick={handleDelete}>Delete from library</button>
      </div>
    {/if}
  </button>
{/if}

<style>
  .book-card-grid {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 8px;
    cursor: pointer;
    background: none;
    border: none;
    padding: 0;
    text-align: left;
    color: var(--text-primary);
    transition: transform 0.15s;
  }

  .book-card-grid:hover {
    transform: translateY(-2px);
  }

  .cover-container {
    position: relative;
    width: 100%;
    aspect-ratio: 2/3;
    border-radius: 6px;
    overflow: hidden;
    background: var(--bg-tertiary);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  }

  .cover-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .cover-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 12px;
    gap: 8px;
    background: linear-gradient(135deg, var(--accent), var(--accent-hover));
    color: white;
  }

  .cover-title {
    font-size: 13px;
    font-weight: 600;
    text-align: center;
    line-height: 1.3;
    display: -webkit-box;
    -webkit-line-clamp: 4;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .cover-author {
    font-size: 11px;
    opacity: 0.8;
    text-align: center;
  }

  .format-badge {
    position: absolute;
    top: 6px;
    right: 6px;
    padding: 2px 6px;
    background: rgba(0, 0, 0, 0.6);
    color: white;
    font-size: 10px;
    font-weight: 600;
    border-radius: 3px;
  }

  .book-info {
    padding: 0 2px;
  }

  .book-title {
    font-size: 13px;
    font-weight: 600;
    line-height: 1.3;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .book-author {
    font-size: 12px;
    color: var(--text-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  /* List view */
  .book-card-list {
    position: relative;
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 8px 12px;
    border-radius: 6px;
    cursor: pointer;
    background: none;
    border: none;
    color: var(--text-primary);
    text-align: left;
    width: 100%;
    transition: background-color 0.15s;
  }

  .book-card-list:hover {
    background: var(--bg-secondary);
  }

  .list-cover {
    width: 36px;
    height: 52px;
    flex-shrink: 0;
    border-radius: 3px;
    overflow: hidden;
    background: var(--bg-tertiary);
  }

  .list-cover-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .list-cover-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
  }

  .list-cover-placeholder svg {
    width: 20px;
    height: 20px;
  }

  .list-info {
    flex: 1;
    min-width: 0;
  }

  .list-info .book-title {
    font-size: 14px;
  }

  .list-info .book-author {
    font-size: 12px;
  }

  .list-format {
    font-size: 11px;
    color: var(--text-muted);
    padding: 2px 8px;
    background: var(--bg-tertiary);
    border-radius: 4px;
    flex-shrink: 0;
  }

  .list-date {
    font-size: 12px;
    color: var(--text-muted);
    flex-shrink: 0;
    width: 100px;
    text-align: right;
  }

  .context-menu {
    position: absolute;
    top: 100%;
    right: 0;
    z-index: 100;
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: 6px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    overflow: hidden;
  }

  .context-menu button {
    display: block;
    width: 100%;
    padding: 8px 16px;
    border: none;
    background: none;
    color: var(--text-primary);
    font-size: 13px;
    cursor: pointer;
    text-align: left;
    white-space: nowrap;
  }

  .context-menu button:hover {
    background: var(--bg-secondary);
  }
</style>
