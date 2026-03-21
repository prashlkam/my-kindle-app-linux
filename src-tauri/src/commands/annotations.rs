use crate::db::models::{Annotation, Bookmark, Collection};
use crate::db::Database;
use chrono::Utc;
use rusqlite::params;
use tauri::State;
use uuid::Uuid;

// --- Bookmarks ---

#[tauri::command]
pub fn get_bookmarks(book_id: String, db: State<'_, Database>) -> Result<Vec<Bookmark>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, book_id, position, label, created_at
             FROM bookmarks WHERE book_id = ?1 ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let bookmarks = stmt
        .query_map(params![book_id], |row| {
            Ok(Bookmark {
                id: row.get(0)?,
                book_id: row.get(1)?,
                position: row.get(2)?,
                label: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(bookmarks)
}

#[tauri::command]
pub fn add_bookmark(
    book_id: String,
    position: String,
    label: Option<String>,
    db: State<'_, Database>,
) -> Result<Bookmark, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO bookmarks (id, book_id, position, label, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, book_id, position, label, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(Bookmark {
        id,
        book_id,
        position,
        label,
        created_at: now,
    })
}

#[tauri::command]
pub fn delete_bookmark(id: String, db: State<'_, Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM bookmarks WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// --- Annotations ---

#[tauri::command]
pub fn get_annotations(
    book_id: String,
    db: State<'_, Database>,
) -> Result<Vec<Annotation>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, book_id, type, position, end_pos, text, note, color, created_at, updated_at
             FROM annotations WHERE book_id = ?1 ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let annotations = stmt
        .query_map(params![book_id], |row| {
            Ok(Annotation {
                id: row.get(0)?,
                book_id: row.get(1)?,
                annotation_type: row.get(2)?,
                position: row.get(3)?,
                end_pos: row.get(4)?,
                text: row.get(5)?,
                note: row.get(6)?,
                color: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(annotations)
}

#[tauri::command]
pub fn get_all_annotations(db: State<'_, Database>) -> Result<Vec<Annotation>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, book_id, type, position, end_pos, text, note, color, created_at, updated_at
             FROM annotations ORDER BY created_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let annotations = stmt
        .query_map([], |row| {
            Ok(Annotation {
                id: row.get(0)?,
                book_id: row.get(1)?,
                annotation_type: row.get(2)?,
                position: row.get(3)?,
                end_pos: row.get(4)?,
                text: row.get(5)?,
                note: row.get(6)?,
                color: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(annotations)
}

#[tauri::command]
pub fn add_annotation(
    book_id: String,
    annotation_type: String,
    position: String,
    end_pos: Option<String>,
    text: Option<String>,
    note: Option<String>,
    color: String,
    db: State<'_, Database>,
) -> Result<Annotation, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO annotations (id, book_id, type, position, end_pos, text, note, color, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![id, book_id, annotation_type, position, end_pos, text, note, color, now, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(Annotation {
        id,
        book_id,
        annotation_type,
        position,
        end_pos,
        text,
        note,
        color,
        created_at: now.clone(),
        updated_at: now,
    })
}

#[tauri::command]
pub fn update_annotation(
    id: String,
    note: Option<String>,
    color: String,
    db: State<'_, Database>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "UPDATE annotations SET note = ?1, color = ?2, updated_at = ?3 WHERE id = ?4",
        params![note, color, now, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn delete_annotation(id: String, db: State<'_, Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM annotations WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// --- Collections ---

#[tauri::command]
pub fn get_collections(db: State<'_, Database>) -> Result<Vec<Collection>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, created_at FROM collections ORDER BY name")
        .map_err(|e| e.to_string())?;

    let collections = stmt
        .query_map([], |row| {
            Ok(Collection {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(collections)
}

#[tauri::command]
pub fn create_collection(name: String, db: State<'_, Database>) -> Result<Collection, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO collections (id, name, created_at) VALUES (?1, ?2, ?3)",
        params![id, name, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(Collection {
        id,
        name,
        created_at: now,
    })
}

#[tauri::command]
pub fn delete_collection(id: String, db: State<'_, Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM collections WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn add_book_to_collection(
    book_id: String,
    collection_id: String,
    db: State<'_, Database>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT OR IGNORE INTO book_collections (book_id, collection_id) VALUES (?1, ?2)",
        params![book_id, collection_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn remove_book_from_collection(
    book_id: String,
    collection_id: String,
    db: State<'_, Database>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "DELETE FROM book_collections WHERE book_id = ?1 AND collection_id = ?2",
        params![book_id, collection_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_collection_books(
    collection_id: String,
    db: State<'_, Database>,
) -> Result<Vec<crate::db::models::Book>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT b.id, b.title, b.author, b.language, b.publisher, b.format, b.file_path,
             b.file_hash, b.cover_path, b.added_at, b.last_read, b.metadata
             FROM books b INNER JOIN book_collections bc ON b.id = bc.book_id
             WHERE bc.collection_id = ?1 ORDER BY b.title",
        )
        .map_err(|e| e.to_string())?;

    let books = stmt
        .query_map(params![collection_id], |row| {
            Ok(crate::db::models::Book {
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

// --- Export ---

#[tauri::command]
pub fn export_annotations_markdown(
    book_id: String,
    db: State<'_, Database>,
) -> Result<String, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let book_title: String = conn
        .query_row(
            "SELECT title FROM books WHERE id = ?1",
            params![book_id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT text, note, color, type, created_at FROM annotations
             WHERE book_id = ?1 ORDER BY created_at",
        )
        .map_err(|e| e.to_string())?;

    let mut md = format!("# Annotations: {}\n\n", book_title);

    let rows = stmt
        .query_map(params![book_id], |row| {
            Ok((
                row.get::<_, Option<String>>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })
        .map_err(|e| e.to_string())?;

    for row in rows {
        let (text, note, _color, ann_type, created_at) = row.map_err(|e| e.to_string())?;
        if ann_type == "highlight" {
            if let Some(t) = &text {
                md.push_str(&format!("> {}\n", t));
            }
            if let Some(n) = &note {
                md.push_str(&format!("\n**Note:** {}\n", n));
            }
            md.push_str(&format!("\n*{}*\n\n---\n\n", &created_at[..10]));
        }
    }

    Ok(md)
}

#[tauri::command]
pub fn export_annotations_json(
    book_id: String,
    db: State<'_, Database>,
) -> Result<String, String> {
    let annotations = get_annotations(book_id, db)?;
    serde_json::to_string_pretty(&annotations).map_err(|e| e.to_string())
}
