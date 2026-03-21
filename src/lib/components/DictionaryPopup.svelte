<script lang="ts">
  let { word, x, y, onclose }: {
    word: string;
    x: number;
    y: number;
    onclose: () => void;
  } = $props();

  let definition = $state("Looking up...");
  let loading = $state(true);

  $effect(() => {
    lookupWord(word);
  });

  async function lookupWord(w: string) {
    loading = true;
    try {
      const resp = await fetch(
        `https://api.dictionaryapi.dev/api/v2/entries/en/${encodeURIComponent(w)}`
      );
      if (resp.ok) {
        const data = await resp.json();
        const meanings = data[0]?.meanings || [];
        if (meanings.length > 0) {
          const m = meanings[0];
          const partOfSpeech = m.partOfSpeech || "";
          const def = m.definitions?.[0]?.definition || "No definition found";
          definition = `(${partOfSpeech}) ${def}`;
        } else {
          definition = "No definition found";
        }
      } else {
        definition = "Word not found";
      }
    } catch {
      definition = "Lookup failed — check your connection";
    } finally {
      loading = false;
    }
  }
</script>

<div class="dict-popup" style="left: {x}px; top: {y}px">
  <div class="dict-header">
    <strong class="dict-word">{word}</strong>
    <button class="dict-close" onclick={onclose}>&times;</button>
  </div>
  <p class="dict-def" class:loading>
    {definition}
  </p>
</div>

<style>
  .dict-popup {
    position: fixed;
    z-index: 300;
    width: 280px;
    padding: 12px;
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-radius: 8px;
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.2);
    transform: translate(-50%, 8px);
  }

  .dict-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;
  }

  .dict-word {
    font-size: 16px;
    color: var(--text-primary);
  }

  .dict-close {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 18px;
    cursor: pointer;
    padding: 0 4px;
  }

  .dict-def {
    font-size: 13px;
    line-height: 1.5;
    color: var(--text-secondary);
  }

  .dict-def.loading {
    color: var(--text-muted);
    font-style: italic;
  }
</style>
