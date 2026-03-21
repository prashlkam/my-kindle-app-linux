import { writable } from "svelte/store";
import type { AppSettings } from "$lib/types";
import { getSettings, saveSetting } from "$lib/services/tauri-ipc";

const defaults: AppSettings = {
  theme: "light",
  font_family: "serif",
  font_size: 18,
  line_height: 1.6,
  margin: 40,
  library_path: "",
  view_mode: "grid",
  sort_by: "last_read",
  sort_order: "desc",
};

export const settings = writable<AppSettings>(defaults);

export async function loadSettings() {
  try {
    const s = await getSettings();
    settings.set(s);
    applyTheme(s.theme);
  } catch {
    settings.set(defaults);
  }
}

export async function updateSetting<K extends keyof AppSettings>(
  key: K,
  value: AppSettings[K],
) {
  settings.update((s) => ({ ...s, [key]: value }));
  await saveSetting(key, String(value));
  if (key === "theme") {
    applyTheme(value as string);
  }
}

function applyTheme(theme: string) {
  document.documentElement.setAttribute("data-theme", theme);
}
