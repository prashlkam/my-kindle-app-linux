<script lang="ts">
  import { annotations, showAnnotations, addNote, removeAnnotation } from "$lib/stores/reader";
  import { readerBridge } from "$lib/services/reader-bridge";
  import {
    exportAnnotationsMarkdown,
    exportAnnotationsJson,
  } from "$lib/services/tauri-ipc";
  import { currentBook } from "$lib/stores/reader";

  let editingId = $state<string | null>(null);
  let noteText = $state("");

  function goToAnnotation(position: string) {
    readerBridge.goToPosition(position);
  }

  function startEditNote(id: string, existingNote: string | null) {
    editingId = id;
    noteText = existingNote || "";
  }

  async function saveNote() {
    if (editingId) {
      await addNote(editingId, noteText);
      editingId = null;
      noteText = "";
    }
  }

  function cancelEdit() {
    editingId = null;
    noteText = "";
  }

  async function exportMd() {
    const book = $currentBook;
    if (book) {
      const md = await exportAnnotationsMarkdown(book.id);
      await navigator.clipboard.writeText(md);
    }
  }

  function colorClass(color: string): string {
    return `hl-${color}`;
  }
</script>

<aside class="annotation-panel">
  <div class="panel-header">
    <h2>Annotations</h2>
    <div class="panel-actions">
      <button class="action-btn" onclick={exportMd} title="Copy as Markdown">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
          <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
          <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" />
        </svg>
      </button>
      <button class="close-btn" onclick={() => showAnnotations.set(false)}>
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" width="16" height="16">
          <path d="M18 6L6 18M6 6l12 12" />
        </svg>
      </button>
    </div>
  </div>

  <div class="annotation-list">
    {#each $annotations as ann (ann.id)}
      <div class="annotation-item">
        <button class="ann-text-btn" onclick={() => goToAnnotation(ann.position)}>
          <span class="ann-highlight {colorClass(ann.color)}">
            "{ann.text || "..."}"
          </span>
        </button>

        {#if editingId === ann.id}
          <div class="note-editor">
            <textarea bind:value={noteText} placeholder="Add a note..." rows="3"></textarea>
            <div class="note-actions">
              <button class="save-btn" onclick={saveNote}>Save</button>
              <button class="cancel-btn" onclick={cancelEdit}>Cancel</button>
            </div>
          </div>
        {:else if ann.note}
          <p class="ann-note">{ann.note}</p>
        {/if}

        <div class="ann-footer">
          <button class="small-btn" onclick={() => startEditNote(ann.id, ann.note)}>
            {ann.note ? "Edit note" : "Add note"}
          </button>
          <button class="small-btn danger" onclick={() => removeAnnotation(ann.id)}>
            Remove
          </button>
        </div>
      </div>
    {/each}

    {#if $annotations.length === 0}
      <p class="empty-msg">No annotations yet. Select text to highlight.</p>
    {/if}
  </div>
</aside>

<style>
  .annotation-panel {
    width: var(--sidebar-width);
    height: 100%;
    border-left: 1px solid var(--border);
    background: var(--bg-secondary);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border);
  }

  .panel-header h2 {
    font-size: 14px;
    font-weight: 600;
  }

  .panel-actions {
    display: flex;
    gap: 4px;
  }

  .action-btn, .close-btn {
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

  .action-btn:hover, .close-btn:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }

  .annotation-list {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }

  .annotation-item {
    padding: 10px;
    border-bottom: 1px solid var(--border);
  }

  .ann-text-btn {
    background: none;
    border: none;
    cursor: pointer;
    text-align: left;
    padding: 0;
    width: 100%;
  }

  .ann-highlight {
    display: block;
    font-size: 13px;
    line-height: 1.5;
    padding: 4px 6px;
    border-radius: 3px;
    font-style: italic;
  }

  .hl-yellow { background: var(--highlight-yellow); }
  .hl-blue { background: var(--highlight-blue); }
  .hl-pink { background: var(--highlight-pink); }
  .hl-orange { background: var(--highlight-orange); }

  .ann-note {
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: 6px;
    padding-left: 6px;
    border-left: 2px solid var(--accent);
  }

  .note-editor {
    margin-top: 6px;
  }

  .note-editor textarea {
    width: 100%;
    padding: 6px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg-primary);
    color: var(--text-primary);
    font-size: 12px;
    resize: vertical;
    font-family: inherit;
  }

  .note-actions {
    display: flex;
    gap: 6px;
    margin-top: 4px;
  }

  .save-btn {
    padding: 4px 10px;
    background: var(--accent);
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 12px;
    cursor: pointer;
  }

  .cancel-btn {
    padding: 4px 10px;
    background: var(--bg-tertiary);
    color: var(--text-primary);
    border: none;
    border-radius: 4px;
    font-size: 12px;
    cursor: pointer;
  }

  .ann-footer {
    display: flex;
    gap: 8px;
    margin-top: 6px;
  }

  .small-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 11px;
    cursor: pointer;
    padding: 0;
  }

  .small-btn:hover {
    color: var(--text-primary);
  }

  .small-btn.danger:hover {
    color: #ef4444;
  }

  .empty-msg {
    text-align: center;
    color: var(--text-muted);
    font-size: 13px;
    padding: 24px 16px;
  }
</style>
