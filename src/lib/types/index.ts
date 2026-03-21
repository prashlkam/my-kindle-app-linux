export interface Book {
  id: string;
  title: string;
  author: string;
  language: string;
  publisher: string;
  format: string;
  file_path: string;
  file_hash: string;
  cover_path: string | null;
  added_at: string;
  last_read: string | null;
  metadata: string | null;
}

export interface ReadingProgress {
  book_id: string;
  position: string;
  percentage: number;
  updated_at: string;
}

export interface Bookmark {
  id: string;
  book_id: string;
  position: string;
  label: string | null;
  created_at: string;
}

export interface Annotation {
  id: string;
  book_id: string;
  type: "highlight" | "note";
  position: string;
  end_pos: string | null;
  text: string | null;
  note: string | null;
  color: string;
  created_at: string;
  updated_at: string;
}

export interface Collection {
  id: string;
  name: string;
  created_at: string;
}

export interface AppSettings {
  theme: "light" | "dark" | "sepia";
  font_family: string;
  font_size: number;
  line_height: number;
  margin: number;
  library_path: string;
  view_mode: "grid" | "list";
  sort_by: "title" | "author" | "added_at" | "last_read";
  sort_order: "asc" | "desc";
}

export interface ReaderState {
  book: Book | null;
  toc: TocItem[];
  position: string | null;
  percentage: number;
  is_loading: boolean;
}

export interface TocItem {
  label: string;
  href: string;
  subitems?: TocItem[];
}

export type ViewRoute = "library" | "reader" | "settings";
