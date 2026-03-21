<script lang="ts">
  import "./app.css";
  import Library from "$lib/components/Library.svelte";
  import Reader from "$lib/components/Reader.svelte";
  import Settings from "$lib/components/Settings.svelte";
  import { isReading } from "$lib/stores/reader";
  import { loadSettings } from "$lib/stores/settings";
  import { loadLibrary } from "$lib/stores/library";
  import { onMount } from "svelte";
  import type { ViewRoute } from "$lib/types";

  let currentView: ViewRoute = $state("library");
  let showSettings = $state(false);

  onMount(async () => {
    await loadSettings();
    await loadLibrary();
  });

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      if (showSettings) {
        showSettings = false;
      }
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="app-container">
  {#if $isReading}
    <Reader onclose={() => (currentView = "library")} />
  {:else}
    <Library
      onsettings={() => (showSettings = true)}
    />
  {/if}

  {#if showSettings}
    <Settings onclose={() => (showSettings = false)} />
  {/if}
</div>

<style>
  .app-container {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-primary);
  }
</style>
