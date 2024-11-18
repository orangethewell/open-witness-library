use colored::Colorize;
use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex;

use crate::publib::{self, catalog::{CollectionImage, CollectionPublication}, publication::ContentTables, tables::{PublicationViewItem, PublicationViewItemDocument}};

const TARGET: &'static str = "commands::catalog";
pub struct CatalogManager {
    pub catalog: Mutex<publib::Catalog>,
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn catalog_install_jwpub_file(manager: tauri::State<'_, CatalogManager>, file_path: String) -> Result<(), String> {
    debug!(
        target: TARGET, 
        "{}: {} => Install JWPUB file: {}", 
        "COMMAND_REQUEST".bright_green(),
        "Catalog".bright_magenta(),
        file_path.green()
    );
    let mut catalog = manager.catalog.lock().await;
    catalog.install_jwpub_file(file_path).map_err(|err| err.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn catalog_get_count_from_type(manager: tauri::State<'_, CatalogManager>, publication_type: String) -> Result<i32, String> {
    debug!(
        target: TARGET, 
        "{}: {} => get list count from type {}", 
        "COMMAND_REQUEST".bright_green(),
        "Catalog".bright_magenta(),
        publication_type.green()
    );
    let catalog = manager.catalog.lock().await;
    catalog.get_count_of_type(&publication_type).map_err(|err| err.to_string())

}

// Can change for language support
#[tauri::command]
pub async fn catalog_get_list_from_type(manager: tauri::State<'_, CatalogManager>, publication_type: String) -> Result<Vec<CollectionPublication>, String> {
    debug!(
        target: TARGET, 
        "{}: {} => get list from type {}", 
        "COMMAND_REQUEST".bright_green(),
        "Catalog".bright_magenta(),
        publication_type.green()
    );
    let catalog = manager.catalog.lock().await;
    catalog.get_list_from_type(&publication_type).map_err(|err| err.to_string())
}

#[derive(Serialize, Deserialize)]
pub struct PublicationViewResponse {
    publication_view_items: Vec<PublicationViewItem>,
    publication_view_items_documents: Vec<PublicationViewItemDocument>,
}

#[tauri::command]
pub async fn catalog_get_publication_view_from(manager: tauri::State<'_, CatalogManager>, filename_symbol: String) -> Result<PublicationViewResponse, String> {
    debug!(
        target: TARGET, 
        "{}: {} => get publication view for {}", 
        "COMMAND_REQUEST".bright_green(),
        "Catalog -> Publication".bright_magenta(),
        filename_symbol.green()
    );
    let mut catalog = manager.catalog.lock().await;
    catalog.open_publication_connection(filename_symbol).map_err(|err| err.to_string())?;
    if let Some(publication) = catalog.get_current_publication() {
        return Ok(PublicationViewResponse {
            publication_view_items: publication.get_view_items().map_err(|err| err.to_string())?,
            publication_view_items_documents: publication.get_view_items_documents().map_err(|err| err.to_string())?,
        })
    }

    Err("This publication doesn't exist or wasn't open.".to_owned())
}

#[tauri::command]
pub async fn catalog_get_images_of_type(manager: tauri::State<'_, CatalogManager>, image_type: String, publication_id: i64) -> Result<Vec<CollectionImage>, String> {
    debug!(
        target: TARGET, 
        "{}: {} => get images from publication ID {} of type \"{}\"", 
        "COMMAND_REQUEST".bright_green(),
        "Catalog -> Publication".bright_magenta(),
        publication_id.to_string().yellow(),
        image_type.green()
    );

    let catalog = manager.catalog.lock().await;
    catalog.get_images_of_type(&image_type, publication_id).map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn catalog_get_document_content(manager: tauri::State<'_, CatalogManager>, document_id: i32) -> Result<Option<String>, String> {
    debug!(
        target: TARGET, 
        "{}: {} => get document content decrypted from {}", 
        "COMMAND_REQUEST".bright_green(),
        "Catalog -> Publication".bright_magenta(),
        document_id.to_string().yellow()
    );
    let mut catalog = manager.catalog.lock().await;
    if let Some(publication) = catalog.get_current_publication() {
        return Ok(publication.get_content_text_from(ContentTables::Document, document_id).map_err(|err| err.to_string())?)
    }

    Err("There aren't a publication open.".to_owned())
}