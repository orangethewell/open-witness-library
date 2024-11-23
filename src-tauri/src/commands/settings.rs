use std::{fs, io::Write, path::PathBuf};

use futures_util::StreamExt;
use reqwest::Client;
use tauri::{Emitter, Manager};

use crate::handlers::www::is_base_assets_present;

#[tauri::command]
pub async fn settings_download_base_assets(app: tauri::AppHandle) -> Result<(), String> {
    let font1_response = reqwest::get("https://assetsnffrgf-a.akamaihd.net/assets/ct/1add6d1d93/fonts/jw-icons-external-1970474.woff").await.map_err(|e| e.to_string())?;
    let font2_response = reqwest::get("https://assetsnffrgf-a.akamaihd.net/assets/ct/1add6d1d93/fonts/jw-icons-external-1970474.ttf").await.map_err(|e| e.to_string())?;
    let collector_response =
        reqwest::get("https://assetsnffrgf-a.akamaihd.net/assets/ct/1add6d1d93/collector.css")
            .await
            .map_err(|e| e.to_string())?;

    let total_size = font1_response.content_length().unwrap_or(0)
        + font2_response.content_length().unwrap_or(0)
        + collector_response.content_length().unwrap_or(1);

    let mut downloaded: u64 = 0;

    let data_filepath = app.path().app_local_data_dir().unwrap().join("www");
    let fonts_filepath = data_filepath.join("fonts");

    if !fonts_filepath.exists() {
        fs::create_dir_all(&fonts_filepath).expect("Failed to create fonts directory");
    }

    let mut font1_file = fs::File::create(fonts_filepath.join("jw-icons-external-1970474.woff"))
        .expect("Failed to create font1 file");
    let mut font2_file = fs::File::create(fonts_filepath.join("jw-icons-external-1970474.ttf"))
        .expect("Failed to create font2 file");
    let mut collector_file = fs::File::create(data_filepath.join("collector.css"))
        .expect("Failed to create collector file");

    let mut stream = font1_response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        font1_file.write_all(&chunk).map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;

        app.emit(
            "download-progress",
            downloaded as f64 / total_size as f64 * 100.0,
        )
        .map_err(|e| e.to_string())?;
    }

    let mut stream = font2_response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        font2_file.write_all(&chunk).map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;

        app.emit(
            "download-progress",
            downloaded as f64 / total_size as f64 * 100.0,
        )
        .map_err(|e| e.to_string())?;
    }

    let mut stream = collector_response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        collector_file
            .write_all(&chunk)
            .map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;

        app.emit(
            "download-progress",
            downloaded as f64 / total_size as f64 * 100.0,
        )
        .map_err(|e| e.to_string())?;
    }

    downloaded = total_size;

    app.emit(
        "download-progress",
        downloaded as f64 / total_size as f64 * 100.0,
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn settings_base_assets_present(app: tauri::AppHandle) -> Result<(), Vec<String>> {
    let app_data_dir = app.path().app_local_data_dir().unwrap().join("www");

    is_base_assets_present(&app_data_dir)
}

#[tauri::command]
pub fn settings_set_webview_theme(webview_window: tauri::WebviewWindow, theme: String) {
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        let _ = webview_window.set_theme(match theme.as_str() {
            "dark" => Some(tauri::Theme::Dark),
            "light" => Some(tauri::Theme::Light),
            _ => None,
        });
    }

    // Opcional: tratar o caso para plataformas não desktop
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        println!("The command 'settings_set_webview_theme' is desktop-only.");
    }
}
