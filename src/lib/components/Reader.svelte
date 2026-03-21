<script lang="ts">
  import Toolbar from "./Toolbar.svelte";
  import TOCSidebar from "./TOCSidebar.svelte";
  import AnnotationPanel from "./AnnotationPanel.svelte";
  import DictionaryPopup from "./DictionaryPopup.svelte";
  import {
    currentBook,
    isLoading,
    currentPercentage,
    showToc,
    showAnnotations,
    closeBook,
    updatePosition,
    addHighlight,
    selectionPopup,
    toc,
  } from "$lib/stores/reader";
  import { readerBridge } from "$lib/services/reader-bridge";
  import { onMount, onDestroy } from "svelte";

  let { onclose }: { onclose: () => void } = $props();

  let readerFrame: HTMLIFrameElement;
  let dictPopup = $state<{ word: string; x: number; y: number } | null>(null);

  onMount(() => {
    if (readerFrame) {
      readerBridge.init(readerFrame);

      readerBridge.on("relocated", (data: unknown) => {
        const d = data as { fraction: number; cfi?: string };
        updatePosition(d.cfi || "", d.fraction * 100);
      });

      readerBridge.on("toc-loaded", (data: unknown) => {
        toc.set(data as any[]);
      });

      readerBridge.on("selection", (data: unknown) => {
        const d = data as { text: string; cfi: string; position: { x: number; y: number } };
        selectionPopup.set({
          visible: true,
          x: d.position.x,
          y: d.position.y,
          text: d.text,
          cfi: d.cfi,
        });
      });

      readerBridge.on("word-lookup", (data: unknown) => {
        const d = data as { word: string; position: { x: number; y: number } };
        dictPopup = { word: d.word, x: d.position.x, y: d.position.y };
      });

      readerBridge.on("book-ready", () => {
        readerBridge.getToc();
      });
    }
  });

  onDestroy(() => {
    readerBridge.destroy();
  });

  function handleClose() {
    closeBook();
    onclose();
  }

  function handleKeydown(e: KeyboardEvent) {
    switch (e.key) {
      case "ArrowRight":
      case " ":
        e.preventDefault();
        readerBridge.goNext();
        break;
      case "ArrowLeft":
        e.preventDefault();
        readerBridge.goPrev();
        break;
      case "Escape":
        handleClose();
        break;
      case "t":
        if (!e.ctrlKey && !e.metaKey) showToc.update((v) => !v);
        break;
    }
  }

  function handleHighlight(color: string) {
    const sel = $selectionPopup;
    if (sel) {
      addHighlight(sel.cfi, sel.cfi, sel.text, color);
      selectionPopup.set(null);
    }
  }

  function dismissSelection() {
    selectionPopup.set(null);
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="reader-container">
  <Toolbar onclose={handleClose} />

  <div class="reader-body">
    {#if $showToc}
      <TOCSidebar />
    {/if}

    <div class="reader-main">
      {#if $isLoading}
        <div class="loading-overlay">
          <div class="spinner"></div>
          <p>Opening book...</p>
        </div>
      {/if}

      <iframe
        bind:this={readerFrame}
        src="/reader.html"
        class="reader-iframe"
        title="Book content"
        sandbox="allow-scripts allow-same-origin"
      ></iframe>

      <!-- Progress bar -->
      <div class="progress-bar">
        <div class="progress-fill" style="width: {$currentPercentage}%"></div>
      </div>
      <div class="progress-text">{Math.round($currentPercentage)}%</div>

      <!-- Selection popup -->
      {#if $selectionPopup?.visible}
        <div
          class="selection-popup"
          style="left: {$selectionPopup.x}px; top: {$selectionPopup.y}px"
        >
          <button class="hl-btn hl-yellow" onclick={() => handleHighlight("yellow")} title="Yellow"></button>
          <button class="hl-btn hl-blue" onclick={() => handleHighlight("blue")} title="Blue"></button>
          <button class="hl-btn hl-pink" onclick={() => handleHighlight("pink")} title="Pink"></button>
          <button class="hl-btn hl-orange" onclick={() => handleHighlight("orange")} title="Orange"></button>
          <button class="copy-btn" onclick={() => { navigator.clipboard.writeText($selectionPopup?.text || ""); dismissSelection(); }}>
            Copy
          </button>
        </div>
      {/if}
    </div>

    {#if $showAnnotations}
      <AnnotationPanel />
    {/if}
  </div>

  <!-- Dictionary -->
  {#if dictPopup}
    <DictionaryPopup
      word={dictPopup.word}
      x={dictPopup.x}
      y={dictPopup.y}
      onclose={() => (dictPopup = null)}
    />
  {/if}
</div>

<style>
  .reader-container {
    height: 100%;
    display: flex;
    flex-direction: column;
    background: var(--bg-primary);
  }

  .reader-body {
    flex: 1;
    display: flex;
    overflow: hidden;
    position: relative;
  }

  .reader-main {
    flex: 1;
    position: relative;
    display: flex;
    flex-direction: column;
  }

  .reader-iframe {
    flex: 1;
    width: 100%;
    border: none;
    background: var(--bg-primary);
  }

  .loading-overlay {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: var(--bg-primary);
    z-index: 50;
    gap: 12px;
    color: var(--text-muted);
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--border);
    border-top-color: var(--accent);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .progress-bar {
    height: 3px;
    background: var(--bg-tertiary);
  }

  .progress-fill {
    height: 100%;
    background: var(--accent);
    transition: width 0.3s ease;
  }

  .progress-text {
    position: absolute;
    bottom: 8px;
    right: 12px;
    font-size: 11px;
    color: var(--text-muted);
  }

  .selection-popup {
    position: absolute;
    transform: translate(-50%, -120%);
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 6px 8px;
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
    z-index: 100;
  }

  .hl-btn {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    border: 2px solid transparent;
    cursor: pointer;
    transition: transform 0.1s;
  }

  .hl-btn:hover {
    transform: scale(1.2);
  }

  .hl-yellow { background: var(--highlight-yellow); }
  .hl-blue { background: var(--highlight-blue); }
  .hl-pink { background: var(--highlight-pink); }
  .hl-orange { background: var(--highlight-orange); }

  .copy-btn {
    padding: 4px 8px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    font-size: 12px;
    cursor: pointer;
    margin-left: 4px;
  }

  .copy-btn:hover {
    background: var(--bg-tertiary);
  }
</style>
