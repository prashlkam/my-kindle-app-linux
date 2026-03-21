use crate::db::models::ReadingProgress;
use crate::db::Database;
use chrono::Utc;
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub fn get_reading_progress(
    book_id: String,
    db: State<'_, Database>,
) -> Result<Option<ReadingProgress>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let result = conn.query_row(
        "SELECT book_id, position, percentage, updated_at FROM reading_progress WHERE book_id = ?1",
        params![book_id],
        |row| {
            Ok(ReadingProgress {
                book_id: row.get(0)?,
                position: row.get(1)?,
                percentage: row.get(2)?,
                updated_at: row.get(3)?,
            })
        },
    );

    match result {
        Ok(progress) => Ok(Some(progress)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn save_reading_progress(
    book_id: String,
    position: String,
    percentage: f64,
    db: State<'_, Database>,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO reading_progress (book_id, position, percentage, updated_at)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(book_id) DO UPDATE SET position = ?2, percentage = ?3, updated_at = ?4",
        params![book_id, position, percentage, now],
    )
    .map_err(|e| e.to_string())?;

    // Update last_read on the book
    conn.execute(
        "UPDATE books SET last_read = ?1 WHERE id = ?2",
        params![now, book_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
