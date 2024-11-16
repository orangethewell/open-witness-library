pub mod utils;

pub mod publication;

use publication::extension::ChapterContent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use tauri::async_runtime::Mutex;
use tauri::{http::Response, Manager, State};
use url::Url;

use tauri_plugin_log::fern::colors::{ColoredLevelConfig, Color};

#[macro_use]
extern crate log;
use colored::Colorize;

struct PubManager {
    catalog: Mutex<publication::PubCatalog>,
}

// # Catalog API
// ----------------------------------------------------
/// Open system dialog for selecting a `.jwpub` file. This file will be installed automatically.
#[tauri::command]
fn pubcatalog_install_jwpub_file<'r>(manager: State<'r, PubManager>, pub_path: String) -> Result<(), ()> {
    let manager = tauri::async_runtime::block_on(manager.catalog.lock());
    manager.install_publication(pub_path);
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct PublicationList {
    arrayof: Vec<publication::extension::Publication>,
}

#[derive(Serialize, Deserialize)]
struct ChapterList {
    arrayof: Vec<publication::extension::Chapter>,
    msg: String,
}

/// Get the list of publication from certain category.
#[tauri::command]
fn pubcatalog_get_list_from<'r>(
    manager: State<'r, PubManager>,
    lang: String,
    category: String,
    start_idx: usize,
    limit: usize,
) -> PublicationList {
    let manager = tauri::async_runtime::block_on(manager.catalog.lock());
    println!("COMMAND REQUEST: Catalog => get list from '{lang}/{category}'");

    PublicationList {
        arrayof: manager.get_list_from_category(lang, category, Some(start_idx), Some(limit)),
    }
}

/// Get the chapter summary from a publication.
#[tauri::command]
fn pubcatalog_get_summary_from<'r>(
    manager: State<'r, PubManager>,
    lang: String,
    category: String,
    pub_symbol: String,
) -> ChapterList {
    let manager = tauri::async_runtime::block_on(manager.catalog.lock());
    println!("COMMAND REQUEST: Fetch Catalog | Get summary from '{lang}/{category}/{pub_symbol}'");

    if let Ok(summary) = manager.get_summary_from(lang, category, pub_symbol) {
        ChapterList {
            arrayof: summary,
            msg: "OK".to_string(),
        }
    } else {
        ChapterList {
            arrayof: vec![],
            msg: "ERR".to_string(),
        }
    }
}

/// Get chapter content from a defined publication.
#[tauri::command]
fn pubcatalog_get_chapter_content<'r>(
    manager: State<'r, PubManager>,
    lang: String,
    category: String,
    pub_symbol: String,
    content_id: i64,
) -> ChapterContent {
    let mut manager = tauri::async_runtime::block_on(manager.catalog.lock());
    println!("COMMAND REQUEST: Fetch Catalog | <Symbol: {pub_symbol}>: ContentId: {content_id}");

    let content = manager.get_chapter_content(lang, category, pub_symbol, content_id);
    content
}

/// Set media location for `jwpub-media` URI.
#[tauri::command]
fn pubcatalog_set_media_location<'r>(
    manager: State<'r, PubManager>,
    lang: String,
    category: String,
    pub_symbol: String,
) {
    let mut manager = tauri::async_runtime::block_on(manager.catalog.lock());
    println!("COMMAND REQUEST: Update media-location | <Symbol: {pub_symbol}; Lang: {lang}; Category: {category}>");

    manager.set_media_location(&lang, &category, &pub_symbol);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut colors = ColoredLevelConfig::default()
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
            pubcatalog_install_jwpub_file,
            pubcatalog_get_list_from,
            pubcatalog_get_summary_from,
            pubcatalog_get_chapter_content,
            pubcatalog_set_media_location,
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
            app.manage(PubManager {
                catalog: Mutex::new(publication::PubCatalog::new(
                    app.path()
                        .local_data_dir()
                        .unwrap()
                        .join("open-witness-library"),
                )),
            });
            Ok(())
        })
        .register_uri_scheme_protocol("jwpub-media", |ctx, req| {
            println!("DEBUG: JWPUB_MEDIA Request");
            let app = ctx.app_handle();
            let request = req.uri();
            let image_request = request.path().replace("/", "");
            let media_location =
                tauri::async_runtime::block_on(app.state::<PubManager>().catalog.lock())
                    .media_location
                    .clone();
            println!("DEBUG: request parameters = {:?}", request);
            println!("DEBUG: media requested = {:?}", image_request);
            println!("DEBUG: media location = {:?}", media_location);

            let content: Vec<u8> = {
                if let Ok(data) = fs::read(media_location.join(image_request)) {
                    data
                } else {
                    vec![]
                }
            };

            println!("DEBUG: media size = {:?}", content.len());

            Response::builder()
                .header("Origin", "*")
                .header("Content-Type", "image/jpg")
                .body(content)
                .unwrap()
        }) // TODO: Refactor jwpub using discoveries from Document contents
        // TODO: Use MepsDocumentId table for opening other pub parts
        .register_uri_scheme_protocol("jwpub", |ctx, req| {
            println!("Request to URI");
            let app = ctx.app_handle();
            let request = req.uri();
            // Uri will be valid if it is like this:
            // jwpub://localhost/language/category/pub
            // Example: jwpub:///T/bk/lfb
            let clean_path = request.path().replacen("/", "", 1);
            let arguments: Vec<&str> = clean_path.split("/").collect();
            println!("{:#?}", arguments);
            if arguments.len() <= 3 {
                let query: HashMap<String, String> = request
                    .query()
                    .unwrap()
                    .split("&")
                    .map(|i| {
                        (
                            i.split("=").collect::<Vec<_>>()[0].to_string(),
                            i.split("=").collect::<Vec<_>>()[1].to_string(),
                        )
                    })
                    .collect();
                let pub_manager = app.state::<PubManager>();
                let mut manager = tauri::async_runtime::block_on(pub_manager.catalog.lock());

                if !query.is_empty() {
                    let content = manager
                        .get_chapter_content(
                            arguments[0].to_owned(),
                            arguments[1].to_owned(),
                            arguments[2].to_owned(),
                            query
                                .get("contentId")
                                .unwrap()
                                .parse::<i64>()
                                .unwrap_or_default(),
                        )
                        .content;
                    Response::builder()
                        .header("Access-Control-Allow-Origin", "*")
                        .header("Content-Type", "text/html")
                        .body(content.as_bytes().to_vec())
                        .unwrap()
                } else {
                    Response::builder()
                        .header("Access-Control-Allow-Origin", "*")
                        .header("Content-Type", "application/json")
                        .body(
                            serde_json::to_value(
                                manager
                                    .get_summary_from(
                                        arguments[0].to_owned(),
                                        arguments[1].to_owned(),
                                        arguments[2].to_owned(),
                                    )
                                    .unwrap(),
                            )
                            .unwrap()
                            .to_string()
                            .as_bytes()
                            .to_vec(),
                        )
                        .unwrap()
                }
            } else if arguments.len() >= 4 {
                let pub_manager = app.state::<PubManager>();
                let mut manager = tauri::async_runtime::block_on(pub_manager.catalog.lock());

                if arguments[3] == "cover" {
                    let content = manager.get_cover_icon(
                        arguments[0].to_owned(),
                        arguments[1].to_owned(),
                        arguments[2].to_owned(),
                    );
                    Response::builder()
                        .header("Access-Control-Allow-Origin", "*")
                        .header("Content-Type", "image/jpg")
                        .body(content)
                        .unwrap()
                } else {
                    Response::builder().status(404).body(Vec::new()).unwrap()
                }
            } else {
                Response::builder().status(404).body(Vec::new()).unwrap()
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
