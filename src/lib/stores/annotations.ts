import { writable } from "svelte/store";
import type { Annotation } from "$lib/types";
import { getAllAnnotations } from "$lib/services/tauri-ipc";

export const allAnnotations = writable<Annotation[]>([]);

export async function loadAllAnnotations() {
  try {
    const list = await getAllAnnotations();
    allAnnotations.set(list);
  } catch (e) {
    console.error("Failed to load annotations:", e);
  }
}
