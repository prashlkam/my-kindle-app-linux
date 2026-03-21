use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub language: String,
    pub publisher: String,
    pub format: String,
    pub file_path: String,
    pub file_hash: String,
    pub cover_path: Option<String>,
    pub added_at: String,
    pub last_read: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingProgress {
    pub book_id: String,
    pub position: String,
    pub percentage: f64,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub book_id: String,
    pub position: String,
    pub label: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub id: String,
    pub book_id: String,
    #[serde(rename = "type")]
    pub annotation_type: String,
    pub position: String,
    pub end_pos: Option<String>,
    pub text: Option<String>,
    pub note: Option<String>,
    pub color: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub font_family: String,
    pub font_size: u32,
    pub line_height: f64,
    pub margin: u32,
    pub library_path: String,
    pub view_mode: String,
    pub sort_by: String,
    pub sort_order: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "light".into(),
            font_family: "serif".into(),
            font_size: 18,
            line_height: 1.6,
            margin: 40,
            library_path: String::new(),
            view_mode: "grid".into(),
            sort_by: "last_read".into(),
            sort_order: "desc".into(),
        }
    }
}
