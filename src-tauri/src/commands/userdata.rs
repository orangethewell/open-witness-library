use tokio::sync::Mutex;

use crate::userdata;
use super::catalogue::CatalogManager;

pub struct UserDataManager {
    pub user_data: Mutex<userdata::UserData>,
}

// This function will create a new user mark,
// but instead of calling a dedicated function
// to do this, I will create a command that will
// manage all data, since it need access to both
// userdata manager and catalog manager for MEPS
// id retrieving.
#[tauri::command]
pub async fn userdata_add_user_mark(
    catalog_manager: tauri::State<'_, CatalogManager>,
    manager: tauri::State<'_, UserDataManager>,
    document_id: i32,
    paragraph_identifier: i32,
    start_token: i32,
    end_token: i32,
) -> Result<(), ()> {
    let mut catalog = catalog_manager.catalog.lock().await;
    let mut userdata = manager.user_data.lock().await;

    let document_meps: i32;
    let issue_tag_number: String;

    if let Some(publication) = catalog.get_current_publication() {
        if let Ok(Some(document)) = publication.get_document_by_id(document_id) {
            document_meps = document.meps_document_id;
        } else {
            return Err(());
        }

        if let Ok(Some(publication_meta)) = publication.get_publication_meta() {
            issue_tag_number = publication_meta.issue_tag_number;
        } else {
            return Err(());
        }
    }

    Ok(())
}

#[tauri::command]
pub fn userdata_extend_user_mark(
    catalog_manager: tauri::State<'_, CatalogManager>,
    manager: tauri::State<'_, UserDataManager>,
    document_id: i32,
    paragraph_identifier: i32,
    start_token: i32,
    end_token: i32,
    user_mark_guid: String
) {

}

#[tauri::command]
pub fn userdata_delete_user_mark(
    manager: tauri::State<'_, UserDataManager>,
    user_mark_guid: String,
) {

}