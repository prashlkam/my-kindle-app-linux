use crate::db::models::AppSettings;
use crate::db::Database;
use rusqlite::params;
use tauri::State;

#[tauri::command]
pub fn get_settings(db: State<'_, Database>) -> Result<AppSettings, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT key, value FROM settings")
        .map_err(|e| e.to_string())?;

    let rows: Vec<(String, String)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let mut settings = AppSettings::default();

    for (key, value) in &rows {
        match key.as_str() {
            "theme" => settings.theme = value.clone(),
            "font_family" => settings.font_family = value.clone(),
            "font_size" => {
                if let Ok(v) = value.parse() {
                    settings.font_size = v;
                }
            }
            "line_height" => {
                if let Ok(v) = value.parse() {
                    settings.line_height = v;
                }
            }
            "margin" => {
                if let Ok(v) = value.parse() {
                    settings.margin = v;
                }
            }
            "library_path" => settings.library_path = value.clone(),
            "view_mode" => settings.view_mode = value.clone(),
            "sort_by" => settings.sort_by = value.clone(),
            "sort_order" => settings.sort_order = value.clone(),
            _ => {}
        }
    }

    Ok(settings)
}

#[tauri::command]
pub fn save_setting(key: String, value: String, db: State<'_, Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = ?2",
        params![key, value],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
