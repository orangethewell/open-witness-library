pub mod commands;
pub mod handlers;
pub mod publib;
pub mod utils;

use commands::catalogue::CatalogManager;
use handlers::catalog::jwpub_media_handler;
use handlers::www::appdata_handler;
use tauri::async_runtime::Mutex;
use tauri::{http::Response, Manager};

use commands::{catalogue, settings};
use tauri_plugin_log::fern::colors::{Color, ColoredLevelConfig};

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
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{} - [{}][{}] {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                        colors.color(record.level()),
                        {
                            if record.target().len() > 0 {
                                record.target()
                            } else {
                                "open-witness-library"
                            }
                        }
                        .magenta(),
                        message
                    ))
                })
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            catalogue::catalog_install_jwpub_from_archive,
            catalogue::catalog_install_jwpub_file,
            catalogue::catalog_get_list_from_type,
            catalogue::catalog_get_count_from_type,
            catalogue::catalog_open_connection,
            catalogue::catalog_get_publication_view_from,
            catalogue::catalog_check_document_exists,
            catalogue::catalog_get_documents,
            catalogue::catalog_get_document_by_id,
            catalogue::catalog_get_document_content,
            catalogue::catalog_get_images_of_type,
            settings::settings_set_webview_theme,
            settings::settings_base_assets_present,
            settings::settings_download_base_assets,
        ])
        .setup(|app| {
            info!(
                target: "open-witness-library",
                "app data path is {}",
                app.path()
                    .app_local_data_dir()
                    .unwrap()
                    .display()
                    .to_string()
                    .green()
            );
            app.manage(catalogue::CatalogManager {
                catalog: Mutex::new(
                    publib::Catalog::init(
                        app.path()
                            .app_local_data_dir()
                            .unwrap()
                            .join("publications"),
                    )
                    .expect("Couldn't initialize catalog"),
                ),
            });
            Ok(())
        })
        .register_uri_scheme_protocol("jwpub-media", |ctx, req| {
            let app = ctx.app_handle();
            let filename = req.uri().path().replace("/", "");
            let state = app.state::<CatalogManager>();
            let mut manager = tauri::async_runtime::block_on(state.catalog.lock());

            match jwpub_media_handler(&mut manager, &filename).map_err(|err| err.to_string()) {
                Ok(response) => response,
                Err(err) => {
                    error!(
                        target: "jwpub-media::handler",
                        "Error handling jwpub-media request: {}",
                        err.red()
                    );
                    Response::builder()
                        .status(500)
                        .body(err.into_bytes())
                        .unwrap()
                }
            }
        })
        .register_uri_scheme_protocol("appdata", |ctx, req| {
            let app = ctx.app_handle();
            let file: String = req.uri().path().replacen("/", "", 1);
            let mut filepath = app.path().app_local_data_dir().unwrap().join("www");
            filepath.extend(file.split("/"));

            match appdata_handler(&filepath).map_err(|err| err.to_string()) {
                Ok(response) => response,
                Err(err) => {
                    error!(
                        target: "appdata::handler",
                        "Error handling appdata request: {}",
                        err.red()
                    );
                    Response::builder()
                        .status(500)
                        .body(err.into_bytes())
                        .unwrap()
                }
            }
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
