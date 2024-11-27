#![allow(dead_code)]

pub struct BlockRange {
    // Primary key
    pub id: i32,

    pub block_type: i32,
    pub identifier: i32,

    pub start_token: i32,
    pub end_token: i32,

    // Foreign key for `UserMark` table
    pub user_mark_id: i32
}

pub struct Bookmark {
    // Primary key
    pub id: i32,

    // Foreign key for `Location` table
    pub location_id: i32,

    // Foreign key for `Location` table
    pub publication_location_id: i32,

    pub slot: i32,
    pub title: String,
    pub snippet: String,

    pub block_type: i32,
    pub block_identifier: i32,
}

pub struct IndependentMedia {
    // Primary key
    pub id: i32,

    pub original_filename: String,
    pub file_path: String,
    pub mime_type: String,
    pub hash: String
}

pub struct InputField {
    // Primary key | Foreign Key to `Location` table
    pub location_id: i32,

    // Primary key
    pub text_tag: i32,

    pub value: String,
}

pub struct LastModified(String);

pub struct Location {
    // Primary key
    pub id: i32,

    pub book_number: i32,
    pub chapter_number: i32,
    pub document_id: i32,
    pub track: i32,
    pub issue_tag_number: i32,

    pub key_symbol: String,
    pub meps_language: i32,
    pub type_id: i32,
    pub title: String
}

pub struct Note {
    // Primary key
    pub id: i32,

    pub guid: String,
    // Foreign key to `UserMark` table
    pub user_mark_id: i32,
    // Foreign key to `Location` table
    pub location_id: i32,

    pub title: String,
    pub content: String,

    pub last_modified: String,
    pub created: String,

    pub block_type: i32,
    pub block_identifier: i32,
}

pub struct PlaylistItem {
    // Primary key  
    pub id: i32,

    pub label: String,
    pub start_trim_offset_ticks: i32,
    pub end_trim_offset_ticks: i32,

    pub accuracy: i32,
    pub end_action: i32,
    // Foreign key to `IndependentMedia` table
    pub thumbnail_file_path: String,
}

pub struct PlaylistItemAccuracy {
    // Primary key  
    pub id: i32,
    pub description: String,
}

pub struct PlaylistItemIndependentMediaMap {
    // Primary key  
    pub id: i32,
    
    // Primary Key | Foreign key to `IndependentMedia` table
    pub independent_media_id: i32,

    pub duration_ticks: i32,
}

pub struct PlaylistItemLocationMap {
    // Primary key  
    pub id: i32,
    
    // Primary Key | Foreign key to `Location` table
    pub location_id: i32,

    major_multimedia_type: i32,

    base_duration_ticks: i32,
}

pub struct PlaylistItemMarker {
    // Primary key
    pub id: i32,

    playlist_item_id: i32,
    label: String,
    start_time_ticks: i32,
    duration_ticks: i32,
    end_transition_duration_ticks: i32,
}

pub struct PlaylistItemMarkerBibleVerseMap {
    // Primary key
    pub playlist_item_marker_id: i32,

    pub verse_id: i32,
}

pub struct PlaylistItemMarkerParagraphMap {
    // Primary key
    pub playlist_item_marker_id: i32,

    pub meps_document_id: i32,
    pub paragraph_index: i32,
    pub marker_index_within_paragraph: i32,
}

pub struct Tag {
    // Primary key
    pub id: i32,
    pub type_id: i32,

    pub name: String,
}

pub struct TagMap {
    // Primary key
    pub id: i32,

    // Foreign key to `PlaylistItem` table
    playlist_item_id: i32,

    // Foreign key to `Location` table
    location_id: i32,

    // Foreign key to `Note` table
    note_id: i32,

    // Foreign key to `Tag` table
    tag_id: i32,

    position: i32
}

pub struct UserMark {
    // Primary key
    pub id: i32,
    pub color_index: i32,
    // Foreign key to `Location` table
    pub location_id: i32,

    pub style_index: i32,
    pub user_mark_guid: String,
    pub version: i32
}