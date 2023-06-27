use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Chapter {
    pub id: i64,
    pub class: i64,
    pub section: i64,
    pub number: i64,
    pub title: String,
    pub context_title: String
}
#[derive(Serialize, Deserialize)]
pub struct ChapterContent {
    pub content: String,
    pub next_exists: bool,
    pub previous_exists: bool
}

#[derive(Serialize, Deserialize)]
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