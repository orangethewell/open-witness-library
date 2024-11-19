use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    pub name: String,
    pub hash: String,
    pub timestamp: String,
    pub version: i32,
    pub expanded_size: i32,
    pub content_format: Option<String>,
    pub html_validated: bool,
    pub meps_platform_version: f32,
    pub meps_build_number: i32,
    pub publication: PublicationManifest,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublicationManifest {
    pub file_name: String,
    #[serde(rename = "type")]
    pub type_id: i32,

    pub title: String,
    pub short_title: String,
    pub display_title: String,
    pub reference_title: String,
    pub undated_reference_title: String,

    pub title_rich: Option<String>,
    pub display_title_rich: Option<String>,
    pub reference_title_rich: Option<String>,
    pub undated_reference_title_rich: Option<String>,

    pub symbol: String,
    pub unique_english_symbol: String,
    pub unique_symbol: String,
    pub undated_symbol: String,
    pub english_symbol: String,

    pub language: i32,
    pub hash: String,
    pub timestamp: String,
    pub min_platform_version: i32,
    pub schema_version: i32,
    pub year: i32,
    pub issue_id: i32,
    pub issue_number: Option<i32>,
    pub variation: String,
    pub publication_type: String,

    pub root_symbol: String,
    pub root_year: i32,
    pub root_language: i32,

    pub images: Vec<Image>,
    pub categories: Vec<String>,
    pub attributes: Vec<String>,
    pub issue_attributes: Vec<String>,
    pub issue_properties: IssueProperties,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub signature: String,
    pub file_name: String,
    #[serde(rename = "type")]
    pub image_type: String,
    pub attribute: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IssueProperties {
    pub title: String,
    pub undated_title: String,
    pub cover_title: String,
    pub title_rich: Option<String>,
    pub undated_title_rich: Option<String>,
    pub cover_title_rich: Option<String>,
    pub symbol: String,
    pub undated_symbol: String,
}
