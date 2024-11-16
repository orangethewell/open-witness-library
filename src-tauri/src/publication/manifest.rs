use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
pub struct Manifest {
    pub name: String,
    pub hash: String,
    pub timestamp: String,
    pub version: i32,
    pub expanded_size: i32,
    pub content_format: String,
    pub html_validated: bool,
    pub meps_platform_version: f32,
    pub meps_build_number: i32,
    pub publication: Publication,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Publication {
    pub file_name: String,
    pub title: String,
    pub short_title: String,
    pub display_title: String,
    pub reference_title: String,
    pub language: i32,
    pub year: i32,
    pub issue_id: i32,
    pub issue_number: i32,
    pub publication_type: String,
    pub symbol: String,
    pub images: Vec<Image>,
    pub categories: Vec<String>,
    pub attributes: Vec<String>,
    pub issue_properties: IssueProperties,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub signature: String,
    pub file_name: String,
    pub image_type: String,
    pub attribute: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueProperties {
    pub title: String,
    pub cover_title: String,
    pub symbol: String,
    pub undated_symbol: String,
}
