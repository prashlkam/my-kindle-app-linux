use crate::db::models::Book;
use chrono::Utc;
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub fn import_single_book(
    conn: &Connection,
    file_path: &str,
    app_data_dir: &Path,
) -> Result<Book, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(format!("File not found: {}", file_path).into());
    }

    // Read file and compute hash
    let file_data = std::fs::read(path)?;
    let hash = format!("{:x}", Sha256::digest(&file_data));

    // Check for duplicates
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) > 0 FROM books WHERE file_hash = ?1",
            params![hash],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if exists {
        return Err("Book already exists in library".into());
    }

    // Extract metadata from filename
    let file_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown");

    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let (title, author) = parse_filename(file_name);

    // Copy file to library directory
    let library_dir = app_data_dir.join("library");
    std::fs::create_dir_all(&library_dir)?;

    let dest_filename = format!("{}.{}", Uuid::new_v4(), extension);
    let dest_path = library_dir.join(&dest_filename);
    std::fs::copy(path, &dest_path)?;

    // Extract cover if possible
    let cover_path = extract_cover(&file_data, &extension, app_data_dir);

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let dest_path_str = dest_path.to_string_lossy().to_string();

    let book = Book {
        id: id.clone(),
        title,
        author,
        language: String::new(),
        publisher: String::new(),
        format: extension,
        file_path: dest_path_str,
        file_hash: hash,
        cover_path: cover_path.map(|p| p.to_string_lossy().to_string()),
        added_at: now,
        last_read: None,
        metadata: None,
    };

    conn.execute(
        "INSERT INTO books (id, title, author, language, publisher, format, file_path, file_hash, cover_path, added_at, last_read, metadata)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            book.id,
            book.title,
            book.author,
            book.language,
            book.publisher,
            book.format,
            book.file_path,
            book.file_hash,
            book.cover_path,
            book.added_at,
            book.last_read,
            book.metadata,
        ],
    )?;

    Ok(book)
}

fn parse_filename(name: &str) -> (String, String) {
    // Try common patterns: "Author - Title", "Title - Author", "Title (Author)"
    if let Some(idx) = name.find(" - ") {
        let left = name[..idx].trim().to_string();
        let right = name[idx + 3..].trim().to_string();
        // Heuristic: if left looks like an author name (shorter, has capitalized words)
        if left.len() < right.len() {
            return (right, left);
        }
        return (left, right);
    }

    if let Some(start) = name.find('(') {
        if let Some(end) = name.find(')') {
            let title = name[..start].trim().to_string();
            let author = name[start + 1..end].trim().to_string();
            return (title, author);
        }
    }

    // Replace underscores and hyphens with spaces for title
    let title = name.replace('_', " ").replace('-', " ");
    (title, String::new())
}

fn extract_cover(
    _file_data: &[u8],
    _extension: &str,
    _app_data_dir: &Path,
) -> Option<PathBuf> {
    // Cover extraction from EPUB/MOBI requires parsing the file format.
    // foliate-js handles this on the frontend side.
    // For now, we'll let the frontend extract and display covers.
    // A future enhancement could use a Rust EPUB/MOBI parser here.
    None
}
