#![allow(unused_must_use)]

use std::collections::HashMap;
use std::fs;
use jwpub::extension::ChapterContent;
use serde::{Serialize, Deserialize};
use tauri::Manager;
use tauri::State;
use tauri::http::{ResponseBuilder};
use url::Url;
use tauri::async_runtime::{Mutex};

mod jwpub;

struct PubManager {
    catalog: Mutex<jwpub::PubCatalog>
}

// # Catalog API
// ----------------------------------------------------
/// Open system dialog for selecting a `.jwpub` file. This file will be installed automatically.
#[tauri::command]
async fn pubcatalog_install_jwpub_file<'r>(manager: State<'r, PubManager>) -> Result<(), ()> {
    todo!()
}

#[derive(Serialize, Deserialize)]
struct PublicationList {
    arrayof: Vec<jwpub::extension::Publication>
}

#[derive(Serialize, Deserialize)]
struct ChapterList {
    arrayof: Vec<jwpub::extension::Chapter>,
    msg: String
}

/// Get the list of publication from certain category.
#[tauri::command]
fn pubcatalog_get_list_from<'r>(manager: State<'r, PubManager>, lang: String, category: String, start_idx: usize, limit: usize) -> PublicationList {
    let manager = tauri::async_runtime::block_on(manager.catalog.lock());
    println!("COMMAND REQUEST: Catalog => get list from '{lang}/{category}'");

    PublicationList {
        arrayof: manager.get_list_from_category(lang, category, Some(start_idx), Some(limit))

    }
}

/// Get the chapter summary from a publication.
#[tauri::command]
fn pubcatalog_get_summary_from<'r>(manager: State<'r, PubManager>, lang: String, category: String, pub_symbol: String) -> ChapterList {
    let manager = tauri::async_runtime::block_on(manager.catalog.lock());
    println!("COMMAND REQUEST: Fetch Catalog | Get summary from '{lang}/{category}/{pub_symbol}'");

    if let Ok(summary) = manager.get_summary_from(lang, category, pub_symbol) {
        ChapterList {
            arrayof: summary,
            msg: "OK".to_string()
        }
    } else {
        ChapterList {
            arrayof: vec![],
            msg: "ERR".to_string()
        }
    }
}

/// Get chapter content from a defined publication.
#[tauri::command]
fn pubcatalog_get_chapter_content<'r>(manager: State<'r, PubManager>, lang: String, category: String, pub_symbol: String, content_id: i64) -> ChapterContent {
    let mut manager = tauri::async_runtime::block_on(manager.catalog.lock());
    println!("COMMAND REQUEST: Fetch Catalog | <Symbol: {pub_symbol}>: ContentId: {content_id}");
    
    let content = manager.get_chapter_content(lang, category, pub_symbol, content_id);
    content
} 

/// Set media location for `jwpub-media` URI.
#[tauri::command]
fn pubcatalog_set_media_location<'r>(manager: State<'r, PubManager>, lang: String, category: String, pub_symbol: String) {
  let mut manager = tauri::async_runtime::block_on(manager.catalog.lock());
  println!("COMMAND REQUEST: Update media-location | <Symbol: {pub_symbol}; Lang: {lang}; Category: {category}>");

  manager.set_media_location(&lang, &category, &pub_symbol);
}
// ----------------------------------------------------


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            pubcatalog_install_jwpub_file,
            pubcatalog_get_list_from,
            pubcatalog_get_summary_from,
            pubcatalog_get_chapter_content,
            pubcatalog_set_media_location,
        ])
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.set_title("Open Witness Library");
            app.manage(PubManager { 
                catalog: Mutex::new(
                    jwpub::PubCatalog::new(
                        app.path().local_data_dir().unwrap()
                        .join("open-witness-library")))
                    }
                );
            Ok(())
        })
        .register_uri_scheme_protocol("jwpub-media", |app, req| {
            println!("URI Request");
            let request: Url = req.uri().parse().unwrap();
            let image_request = request.host_str().unwrap();
            let media_location = tauri::async_runtime::block_on(app.state::<PubManager>().catalog.lock()).media_location.clone();

            let content: Vec<u8> = {
                if let Ok(data) = fs::read(media_location.join(image_request)) {
                    data
                } else {
                    vec![]
                }
            };

            ResponseBuilder::new()
                .header("Origin", "*")
                .mimetype("text/html")
                .body(content)
        }) // TODO: Refactor jwpub using discoveries from Document contents
        // TODO: Use MepsDocumentId table for opening other pub parts
        .register_uri_scheme_protocol("jwpub", |app, req| {
            println!("Request to URI");
            let request: Url = req.uri().parse().unwrap();
            // Uri will be valid if it is like this:
            // jwpub://localhost/language/category/pub 
            // Example: jwpub:///T/bk/lfb
            let arguments = request.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
            println!("{:#?}", arguments);
            if arguments.len() <= 3 {
                let query: HashMap<String, String> = request.query_pairs().map(|i| (i.0.to_string(), i.1.to_string())).collect();
                let pub_manager = app.state::<PubManager>();
                let mut manager = tauri::async_runtime::block_on(pub_manager.catalog.lock());

                if !query.is_empty() {
                    let content = manager.get_chapter_content(arguments[0].to_owned(), arguments[1].to_owned(), arguments[2].to_owned(), query.get("contentId").unwrap().parse::<i64>().unwrap_or_default()).content;
                    ResponseBuilder::new()
                        .header("Access-Control-Allow-Origin", "*")
                        .mimetype("text/html")
                        .body(content.as_bytes().to_vec())
                } else {
                    ResponseBuilder::new()
                        .header("Access-Control-Allow-Origin", "*")
                        .mimetype("application/json")
                        .body(serde_json::to_value(manager.get_summary_from(arguments[0].to_owned(), arguments[1].to_owned(), arguments[2].to_owned()).unwrap())
                            .unwrap()
                            .to_string()
                            .as_bytes()
                            .to_vec())
                }
            } else if arguments.len() >= 4 {
                let pub_manager = app.state::<PubManager>();
                let mut manager = tauri::async_runtime::block_on(pub_manager.catalog.lock());

                if arguments[3] == "cover" {
                    let content = manager.get_cover_icon(arguments[0].to_owned(), arguments[1].to_owned(), arguments[2].to_owned());
                    ResponseBuilder::new()
                        .header("Access-Control-Allow-Origin", "*")
                        .mimetype("image/jpg")
                        .body(content)

                } else {
                    ResponseBuilder::new()
                        .status(404)
                        .body(Vec::new())
                }
            } else {
                ResponseBuilder::new()
                    .status(404)
                    .body(Vec::new())

            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
