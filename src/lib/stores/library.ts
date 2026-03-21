import { writable, derived } from "svelte/store";
import type { Book, Collection } from "$lib/types";
import {
  getBooks,
  searchBooks,
  getCollections,
  importBook,
  deleteBook as deleteBookIpc,
} from "$lib/services/tauri-ipc";
import { settings } from "./settings";

export const books = writable<Book[]>([]);
export const collections = writable<Collection[]>([]);
export const searchQuery = writable("");
export const selectedCollection = writable<string | null>(null);

export const sortedBooks = derived(
  [books, settings, searchQuery],
  ([$books, $settings, $query]) => {
    let filtered = $books;

    if ($query) {
      const q = $query.toLowerCase();
      filtered = filtered.filter(
        (b) =>
          b.title.toLowerCase().includes(q) ||
          b.author.toLowerCase().includes(q),
      );
    }

    const sorted = [...filtered].sort((a, b) => {
      const key = $settings.sort_by;
      const va = a[key] ?? "";
      const vb = b[key] ?? "";
      if (va < vb) return -1;
      if (va > vb) return 1;
      return 0;
    });

    if ($settings.sort_order === "desc") sorted.reverse();
    return sorted;
  },
);

export async function loadLibrary() {
  try {
    const [bookList, collectionList] = await Promise.all([
      getBooks(),
      getCollections(),
    ]);
    books.set(bookList);
    collections.set(collectionList);
  } catch (e) {
    console.error("Failed to load library:", e);
  }
}

export async function importBookFile(filePath: string): Promise<Book | null> {
  try {
    const book = await importBook(filePath);
    books.update((list) => [book, ...list]);
    return book;
  } catch (e) {
    console.error("Failed to import book:", e);
    return null;
  }
}

export async function removeBook(id: string, deleteFile: boolean) {
  try {
    await deleteBookIpc(id, deleteFile);
    books.update((list) => list.filter((b) => b.id !== id));
  } catch (e) {
    console.error("Failed to delete book:", e);
  }
}

export async function performSearch(query: string) {
  searchQuery.set(query);
  if (query.trim()) {
    try {
      const results = await searchBooks(query);
      books.set(results);
    } catch {
      // Fallback to client-side filtering (already handled by derived store)
    }
  } else {
    await loadLibrary();
  }
}
