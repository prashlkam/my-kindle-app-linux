use crate::db::models::Book;
use crate::db::Database;
use crate::library::import;
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub fn get_books(db: State<'_, Database>) -> Result<Vec<Book>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, title, author, language, publisher, format, file_path, file_hash,
             cover_path, added_at, last_read, metadata FROM books ORDER BY added_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let books = stmt
        .query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                language: row.get(3)?,
                publisher: row.get(4)?,
                format: row.get(5)?,
                file_path: row.get(6)?,
                file_hash: row.get(7)?,
                cover_path: row.get(8)?,
                added_at: row.get(9)?,
                last_read: row.get(10)?,
                metadata: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(books)
}

#[tauri::command]
pub fn get_book(id: String, db: State<'_, Database>) -> Result<Book, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.query_row(
        "SELECT id, title, author, language, publisher, format, file_path, file_hash,
         cover_path, added_at, last_read, metadata FROM books WHERE id = ?1",
        params![id],
        |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                language: row.get(3)?,
                publisher: row.get(4)?,
                format: row.get(5)?,
                file_path: row.get(6)?,
                file_hash: row.get(7)?,
                cover_path: row.get(8)?,
                added_at: row.get(9)?,
                last_read: row.get(10)?,
                metadata: row.get(11)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_book(
    file_path: String,
    db: State<'_, Database>,
    app_handle: tauri::AppHandle,
) -> Result<Book, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    import::import_single_book(&conn, &file_path, &app_data_dir).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_books(
    file_paths: Vec<String>,
    db: State<'_, Database>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<Book>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let mut books = Vec::new();
    for path in &file_paths {
        match import::import_single_book(&conn, path, &app_data_dir) {
            Ok(book) => books.push(book),
            Err(e) => eprintln!("Failed to import {}: {}", path, e),
        }
    }
    Ok(books)
}

#[tauri::command]
pub fn delete_book(
    id: String,
    delete_file: bool,
    db: State<'_, Database>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    if delete_file {
        if let Ok(path) = conn.query_row(
            "SELECT file_path FROM books WHERE id = ?1",
            params![id],
            |row| row.get::<_, String>(0),
        ) {
            std::fs::remove_file(&path).ok();
        }
    }

    // Also remove cover file if exists
    if let Ok(Some(cover)) = conn.query_row(
        "SELECT cover_path FROM books WHERE id = ?1",
        params![id],
        |row| row.get::<_, Option<String>>(0),
    ) {
        std::fs::remove_file(&cover).ok();
    }

    conn.execute("DELETE FROM books WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn read_book_file(file_path: String) -> Result<Vec<u8>, String> {
    std::fs::read(&file_path).map_err(|e| format!("Failed to read file: {}", e))
}

#[tauri::command]
pub fn search_books(query: String, db: State<'_, Database>) -> Result<Vec<Book>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let pattern = format!("%{}%", query);
    let mut stmt = conn
        .prepare(
            "SELECT id, title, author, language, publisher, format, file_path, file_hash,
             cover_path, added_at, last_read, metadata
             FROM books WHERE title LIKE ?1 OR author LIKE ?1
             ORDER BY added_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let books = stmt
        .query_map(params![pattern], |row| {
            Ok(Book {
                id: row.get(0)?,
                title: row.get(1)?,
                author: row.get(2)?,
                language: row.get(3)?,
                publisher: row.get(4)?,
                format: row.get(5)?,
                file_path: row.get(6)?,
                file_hash: row.get(7)?,
                cover_path: row.get(8)?,
                added_at: row.get(9)?,
                last_read: row.get(10)?,
                metadata: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(books)
}
