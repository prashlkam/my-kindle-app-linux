import { writable, get } from "svelte/store";
import type { Book, TocItem, Bookmark, Annotation } from "$lib/types";
import {
  readBookFile,
  getReadingProgress,
  saveReadingProgress,
  getBookmarks,
  getAnnotations,
  addBookmark as addBookmarkIpc,
  deleteBookmark as deleteBookmarkIpc,
  addAnnotation as addAnnotationIpc,
  updateAnnotation as updateAnnotationIpc,
  deleteAnnotation as deleteAnnotationIpc,
} from "$lib/services/tauri-ipc";
import { readerBridge } from "$lib/services/reader-bridge";

export const currentBook = writable<Book | null>(null);
export const isReading = writable(false);
export const isLoading = writable(false);
export const toc = writable<TocItem[]>([]);
export const currentPosition = writable<string | null>(null);
export const currentPercentage = writable(0);
export const bookmarks = writable<Bookmark[]>([]);
export const annotations = writable<Annotation[]>([]);
export const showToc = writable(false);
export const showAnnotations = writable(false);
export const selectionPopup = writable<{
  visible: boolean;
  x: number;
  y: number;
  text: string;
  cfi: string;
} | null>(null);

export async function openBook(book: Book) {
  isLoading.set(true);
  currentBook.set(book);
  isReading.set(true);

  try {
    const fileData = await readBookFile(book.file_path);
    const buffer = new Uint8Array(fileData).buffer;
    const fileName = book.file_path.split(/[/\\]/).pop() || "book";

    readerBridge.openBook(buffer, fileName);

    const [progress, bookmarkList, annotationList] = await Promise.all([
      getReadingProgress(book.id),
      getBookmarks(book.id),
      getAnnotations(book.id),
    ]);

    bookmarks.set(bookmarkList);
    annotations.set(annotationList);

    if (progress) {
      currentPosition.set(progress.position);
      currentPercentage.set(progress.percentage);
      readerBridge.goToPosition(progress.position);
    }

    // Restore highlights
    annotationList
      .filter((a) => a.type === "highlight")
      .forEach((a) => readerBridge.addHighlight(a.position, a.color));
  } catch (e) {
    console.error("Failed to open book:", e);
  } finally {
    isLoading.set(false);
  }
}

export function closeBook() {
  const pos = get(currentPosition);
  const pct = get(currentPercentage);
  const book = get(currentBook);

  if (book && pos) {
    saveReadingProgress(book.id, pos, pct).catch(console.error);
  }

  currentBook.set(null);
  isReading.set(false);
  toc.set([]);
  currentPosition.set(null);
  currentPercentage.set(0);
  bookmarks.set([]);
  annotations.set([]);
  showToc.set(false);
  showAnnotations.set(false);
  selectionPopup.set(null);
}

export function updatePosition(position: string, percentage: number) {
  currentPosition.set(position);
  currentPercentage.set(percentage);

  const book = get(currentBook);
  if (book) {
    saveReadingProgress(book.id, position, percentage).catch(console.error);
  }
}

export async function toggleBookmark() {
  const book = get(currentBook);
  const pos = get(currentPosition);
  if (!book || !pos) return;

  const existing = get(bookmarks).find((b) => b.position === pos);
  if (existing) {
    await deleteBookmarkIpc(existing.id);
    bookmarks.update((list) => list.filter((b) => b.id !== existing.id));
  } else {
    const bm = await addBookmarkIpc(book.id, pos, null);
    bookmarks.update((list) => [...list, bm]);
  }
}

export async function addHighlight(
  cfi: string,
  endCfi: string,
  text: string,
  color: string = "yellow",
) {
  const book = get(currentBook);
  if (!book) return;

  const annotation = await addAnnotationIpc(
    book.id,
    "highlight",
    cfi,
    endCfi,
    text,
    null,
    color,
  );
  annotations.update((list) => [...list, annotation]);
  readerBridge.addHighlight(cfi, color);
}

export async function addNote(annotationId: string, note: string) {
  const ann = get(annotations).find((a) => a.id === annotationId);
  if (!ann) return;

  await updateAnnotationIpc(annotationId, note, ann.color);
  annotations.update((list) =>
    list.map((a) => (a.id === annotationId ? { ...a, note } : a)),
  );
}

export async function removeAnnotation(id: string) {
  const ann = get(annotations).find((a) => a.id === id);
  if (!ann) return;

  await deleteAnnotationIpc(id);
  readerBridge.removeHighlight(ann.position);
  annotations.update((list) => list.filter((a) => a.id !== id));
}
