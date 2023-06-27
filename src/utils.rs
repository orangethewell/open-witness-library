use wasm_bindgen::prelude::*; 
use rust_embed::RustEmbed;

#[wasm_bindgen]
extern "C" {
     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke")]
      pub async fn sync_invoke_with_args(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke")]
    pub async fn sync_invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke")]
    pub fn invoke_with_args(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke")]
    pub fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "convertFileSrc")]
    pub fn convert_file_src(file: &str, protocol: Option<&str>) -> String;
} 

#[derive(RustEmbed)]
#[folder = "i18n"]
pub struct Localizations;

pub mod pub_utils {
    use serde::{Serialize, Deserialize};
    use std::path::PathBuf;

    // Backend Serializable Object
   #[derive(Serialize, Deserialize, Clone, PartialEq)] 
    pub struct Publication {
        // Publication Access Keys 
        pub category: String,
        pub language: String, 
        pub symbol: String,
        // General Details
        pub title: String,
        pub display_title: String, 
        pub cover_icon_path: PathBuf,
        pub year: i64,
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq)] 
    pub struct Chapter {
        pub id: i64,
        pub class: i64,
        pub section: i64,
        pub number: i64,
        pub title: String,
        pub context_title: String
    }
}


pub mod tauri_wrappers {
    #![allow(non_snake_case)]
    use std::{rc::Rc, cell::RefCell};

    use crate::{utils::pub_utils::{Publication, Chapter}, components::publication::ChapterContent};
    use serde::{Serialize, Deserialize};
    use serde_wasm_bindgen::{to_value, from_value};
    use crate::utils::{
        invoke_with_args,
        log
    };
    

    #[derive(Serialize, Deserialize)]
    struct GetListArgs {
        lang: String,
        category: String,
        startIdx: Option<usize>,
        limit: Option<usize>,
    }

    #[derive(Serialize, Deserialize)]
    struct PubList { arrayof: Vec<Publication> }

    #[derive(Serialize, Deserialize)]
    struct ChapterList {
        arrayof: Vec<Chapter>,
        msg: String
    }

    // pub async fn install_jwpub_file() -> Result<JsValue, JsValue>{
    //     sync_invoke_without_args("pubcatalog_install_jwpub_file").await  
    // } 

    pub async fn get_list_from_category(lang: String, category: String, start_idx: usize, limit: usize) -> Vec<Publication> {
        let args = GetListArgs {
            lang,
            category,
            startIdx: Some(start_idx),
            limit: Some(limit),
        };

        // TODO: Consume value because it's a promise;
        let value = invoke_with_args("pubcatalog_get_list_from", to_value(&args).unwrap());
        let promise = js_sys::Promise::resolve(&value);
        let result = wasm_bindgen_futures::JsFuture::from(promise).await;

        if let Ok(list_object) = result {
            let list: PubList = from_value(list_object).unwrap();
            list.arrayof
        } else {
            vec![]
        }
    }

    #[derive(Serialize, Deserialize)]
    struct MediaArgs {
        lang: String,
        category: String,
        pubSymbol: String,
    }

    #[derive(Serialize, Deserialize)]
    struct ChapterArgs {
        lang: String,
        category: String,
        pubSymbol: String,
        contentId: i64,
    }

    pub fn set_media_location(lang: &str, category: &str, pub_symbol: &str){
        let args = MediaArgs{lang: lang.to_owned(), category: category.to_owned(), pubSymbol: pub_symbol.to_owned()};
        invoke_with_args("pubcatalog_set_media_location", to_value(&args).unwrap());
    }

    pub async fn get_chapter_content(lang: &str, category: &str, pub_symbol: &str, chapter_id: i32) -> ChapterContent {
        let args = ChapterArgs{lang: lang.to_owned(), category: category.to_owned(), pubSymbol: pub_symbol.to_owned(), contentId: chapter_id.into()};
        let content = wasm_bindgen_futures::JsFuture::from(
            js_sys::Promise::resolve(
                &invoke_with_args("pubcatalog_get_chapter_content", to_value(&args).unwrap())
            )
        ).await.unwrap();
        log(&format!("Is string? {}", content.is_string()));
        log(&format!("Is object? {}", content.is_object()));
        log(&format!("{:#?}", content));
        let content: ChapterContent = from_value(content).unwrap_or(ChapterContent::default());
        content
    }

    pub async fn get_summary_from(lang: &str, category: &str, pub_symbol: &str) -> Vec<Chapter>{
        let args = MediaArgs{lang: lang.to_owned(), category: category.to_owned(), pubSymbol: pub_symbol.to_owned()};
        let summary_object = wasm_bindgen_futures::JsFuture::from(
            js_sys::Promise::resolve(
                &invoke_with_args("pubcatalog_get_summary_from", to_value(&args).unwrap())
            )
        ).await.unwrap();

        let summary: ChapterList = from_value(summary_object).unwrap();
        summary.arrayof
    }
}
