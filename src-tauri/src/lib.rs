pub mod utils;
pub mod publib;
pub mod commands;

use publib::extension::ChapterContent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use tauri::async_runtime::Mutex;
use tauri::{http::Response, Manager, State};

use tauri_plugin_log::fern::colors::{ColoredLevelConfig, Color};
#[macro_use]
use commands::catalogue;

#[macro_use]
extern crate log;
use colored::Colorize;

// /// Set media location for `jwpub-media` URI.
// #[tauri::command]
// fn pubcatalog_set_media_location<'r>(
//     manager: State<'r, PubManager>,
//     lang: String,
//     category: String,
//     pub_symbol: String,
// ) {
//     let mut manager = tauri::async_runtime::block_on(manager.catalog.lock());
//     println!("COMMAND REQUEST: Update media-location | <Symbol: {pub_symbol}; Lang: {lang}; Category: {category}>");

//     manager.set_media_location(&lang, &category, &pub_symbol);
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let colors = ColoredLevelConfig::default()
        .info(Color::Blue)
        .debug(Color::Green)
        .trace(Color::Cyan);

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_log::Builder::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{} - [{}][{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    colors.color(record.level()),
                    {if record.target().len() > 0 {record.target()} else {"open-witness-library"}}.magenta(),
                    message
                ))
            })
            .build()
        )
        .invoke_handler(tauri::generate_handler![
            catalogue::catalog_install_jwpub_file,
            catalogue::catalog_get_list_from_type,
            catalogue::catalog_get_count_from_type,
            catalogue::catalog_get_publication_view_from,
            catalogue::catalog_get_document_by_id,
            catalogue::catalog_get_document_content,
            catalogue::catalog_get_images_of_type,
        ])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("Open Witness Library");
            debug!(
                target: "open-witness-library",
                "app data path is {}",
                app.path()
                    .local_data_dir()
                    .unwrap()
                    .join("open-witness-library")
                    .display()
                    .to_string()
                    .green()
            );
            app.manage(catalogue::CatalogManager {
                catalog: Mutex::new(publib::Catalog::init(
                    app.path()
                        .local_data_dir()
                        .unwrap()
                        .join("open-witness-library")
                        .join("publications"),
                ).expect("Couldn't initialize catalog")),
            });
            Ok(())
        })
        // .register_uri_scheme_protocol("jwpub-media", |ctx, req| {
        // }) // TODO: Refactor jwpub using discoveries from Document contents
        // TODO: Use MepsDocumentId table for opening other pub parts
        // https://b.jw-cdn.org/apis/pub-media/GETPUBMEDIALINKS?output=json&docid={MepsDocumentId}&langwritten=T
        // https://app.jw-cdn.org/catalogs/publications/v4/manifest.json
        // https://app.jw-cdn.org/catalogs/publications/v4/{current_from_manifest.json_above}/catalog.db.gz
        // .register_uri_scheme_protocol("jwpub", |ctx, req| {
        // })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
