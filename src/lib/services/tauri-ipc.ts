import { invoke } from "@tauri-apps/api/core";
import type {
  Book,
  ReadingProgress,
  Bookmark,
  Annotation,
  Collection,
  AppSettings,
} from "$lib/types";

// Book commands
export async function getBooks(): Promise<Book[]> {
  return invoke("get_books");
}

export async function getBook(id: string): Promise<Book> {
  return invoke("get_book", { id });
}

export async function importBook(filePath: string): Promise<Book> {
  return invoke("import_book", { filePath });
}

export async function importBooks(filePaths: string[]): Promise<Book[]> {
  return invoke("import_books", { filePaths });
}

export async function deleteBook(
  id: string,
  deleteFile: boolean,
): Promise<void> {
  return invoke("delete_book", { id, deleteFile });
}

export async function readBookFile(
  filePath: string,
): Promise<number[]> {
  return invoke("read_book_file", { filePath });
}

export async function searchBooks(query: string): Promise<Book[]> {
  return invoke("search_books", { query });
}

// Reading progress
export async function getReadingProgress(
  bookId: string,
): Promise<ReadingProgress | null> {
  return invoke("get_reading_progress", { bookId });
}

export async function saveReadingProgress(
  bookId: string,
  position: string,
  percentage: number,
): Promise<void> {
  return invoke("save_reading_progress", { bookId, position, percentage });
}

// Bookmarks
export async function getBookmarks(bookId: string): Promise<Bookmark[]> {
  return invoke("get_bookmarks", { bookId });
}

export async function addBookmark(
  bookId: string,
  position: string,
  label: string | null,
): Promise<Bookmark> {
  return invoke("add_bookmark", { bookId, position, label });
}

export async function deleteBookmark(id: string): Promise<void> {
  return invoke("delete_bookmark", { id });
}

// Annotations
export async function getAnnotations(bookId: string): Promise<Annotation[]> {
  return invoke("get_annotations", { bookId });
}

export async function getAllAnnotations(): Promise<Annotation[]> {
  return invoke("get_all_annotations");
}

export async function addAnnotation(
  bookId: string,
  type: string,
  position: string,
  endPos: string | null,
  text: string | null,
  note: string | null,
  color: string,
): Promise<Annotation> {
  return invoke("add_annotation", {
    bookId,
    annotationType: type,
    position,
    endPos,
    text,
    note,
    color,
  });
}

export async function updateAnnotation(
  id: string,
  note: string | null,
  color: string,
): Promise<void> {
  return invoke("update_annotation", { id, note, color });
}

export async function deleteAnnotation(id: string): Promise<void> {
  return invoke("delete_annotation", { id });
}

// Collections
export async function getCollections(): Promise<Collection[]> {
  return invoke("get_collections");
}

export async function createCollection(name: string): Promise<Collection> {
  return invoke("create_collection", { name });
}

export async function deleteCollection(id: string): Promise<void> {
  return invoke("delete_collection", { id });
}

export async function addBookToCollection(
  bookId: string,
  collectionId: string,
): Promise<void> {
  return invoke("add_book_to_collection", { bookId, collectionId });
}

export async function removeBookFromCollection(
  bookId: string,
  collectionId: string,
): Promise<void> {
  return invoke("remove_book_from_collection", { bookId, collectionId });
}

export async function getCollectionBooks(
  collectionId: string,
): Promise<Book[]> {
  return invoke("get_collection_books", { collectionId });
}

// Settings
export async function getSettings(): Promise<AppSettings> {
  return invoke("get_settings");
}

export async function saveSetting(key: string, value: string): Promise<void> {
  return invoke("save_setting", { key, value });
}

// Export
export async function exportAnnotationsMarkdown(
  bookId: string,
): Promise<string> {
  return invoke("export_annotations_markdown", { bookId });
}

export async function exportAnnotationsJson(bookId: string): Promise<string> {
  return invoke("export_annotations_json", { bookId });
}
