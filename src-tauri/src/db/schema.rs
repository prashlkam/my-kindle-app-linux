use rusqlite::Connection;

pub fn initialize_db(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS books (
            id          TEXT PRIMARY KEY,
            title       TEXT NOT NULL,
            author      TEXT NOT NULL DEFAULT '',
            language    TEXT NOT NULL DEFAULT '',
            publisher   TEXT NOT NULL DEFAULT '',
            format      TEXT NOT NULL,
            file_path   TEXT NOT NULL UNIQUE,
            file_hash   TEXT NOT NULL,
            cover_path  TEXT,
            added_at    TEXT NOT NULL,
            last_read   TEXT,
            metadata    TEXT
        );

        CREATE TABLE IF NOT EXISTS reading_progress (
            book_id     TEXT PRIMARY KEY REFERENCES books(id) ON DELETE CASCADE,
            position    TEXT NOT NULL,
            percentage  REAL NOT NULL DEFAULT 0.0,
            updated_at  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS bookmarks (
            id          TEXT PRIMARY KEY,
            book_id     TEXT NOT NULL REFERENCES books(id) ON DELETE CASCADE,
            position    TEXT NOT NULL,
            label       TEXT,
            created_at  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS annotations (
            id          TEXT PRIMARY KEY,
            book_id     TEXT NOT NULL REFERENCES books(id) ON DELETE CASCADE,
            type        TEXT NOT NULL,
            position    TEXT NOT NULL,
            end_pos     TEXT,
            text        TEXT,
            note        TEXT,
            color       TEXT NOT NULL DEFAULT 'yellow',
            created_at  TEXT NOT NULL,
            updated_at  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS collections (
            id          TEXT PRIMARY KEY,
            name        TEXT NOT NULL UNIQUE,
            created_at  TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS book_collections (
            book_id       TEXT REFERENCES books(id) ON DELETE CASCADE,
            collection_id TEXT REFERENCES collections(id) ON DELETE CASCADE,
            PRIMARY KEY (book_id, collection_id)
        );

        CREATE TABLE IF NOT EXISTS settings (
            key         TEXT PRIMARY KEY,
            value       TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_books_title ON books(title);
        CREATE INDEX IF NOT EXISTS idx_books_author ON books(author);
        CREATE INDEX IF NOT EXISTS idx_annotations_book ON annotations(book_id);
        CREATE INDEX IF NOT EXISTS idx_bookmarks_book ON bookmarks(book_id);
        ",
    )?;
    Ok(())
}
