pub mod commands;
pub mod db;
pub mod library;
pub mod plugins;

use db::Database;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            let database = Database::new(app_data_dir).expect("Failed to initialize database");
            app.manage(database);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Books
            commands::books::get_books,
            commands::books::get_book,
            commands::books::import_book,
            commands::books::import_books,
            commands::books::delete_book,
            commands::books::read_book_file,
            commands::books::search_books,
            // Reading progress
            commands::reading::get_reading_progress,
            commands::reading::save_reading_progress,
            // Bookmarks
            commands::annotations::get_bookmarks,
            commands::annotations::add_bookmark,
            commands::annotations::delete_bookmark,
            // Annotations
            commands::annotations::get_annotations,
            commands::annotations::get_all_annotations,
            commands::annotations::add_annotation,
            commands::annotations::update_annotation,
            commands::annotations::delete_annotation,
            // Collections
            commands::annotations::get_collections,
            commands::annotations::create_collection,
            commands::annotations::delete_collection,
            commands::annotations::add_book_to_collection,
            commands::annotations::remove_book_from_collection,
            commands::annotations::get_collection_books,
            // Settings
            commands::settings::get_settings,
            commands::settings::save_setting,
            // Export
            commands::annotations::export_annotations_markdown,
            commands::annotations::export_annotations_json,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
