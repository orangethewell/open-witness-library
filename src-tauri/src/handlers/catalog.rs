use tauri::http::Response;
use tokio::sync::MutexGuard;

use crate::publib::Catalog;

pub fn jwpub_media_handler<'a>(
    manager: &mut MutexGuard<'a, Catalog>,
    filename: &'a str,
) -> Result<Response<Vec<u8>>, Box<dyn std::error::Error>> {
    let media_data = manager
        .get_current_publication()
        .expect("There's no publication open, unreachable")
        .get_multimedia_data(filename.to_owned())?;
    let extension = filename
        .split(".")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .to_string();

    let mime_type = match extension.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        _ => "application/octet-stream",
    };

    Ok(Response::builder()
        .status(200)
        .header("Origin", "*")
        .header("Content-Type", mime_type)
        .body(media_data)?)
}
