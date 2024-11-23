use std::path::PathBuf;

use tauri::http::Response;

pub fn is_base_assets_present<'a>(data_path: &'a PathBuf) -> Result<(), Vec<String>> {
    let styles_path = vec![
        data_path.join("collector.css"),
        data_path.join(PathBuf::from_iter([
            "fonts",
            "jw-icons-external-1970474.ttf",
        ])),
        data_path.join(PathBuf::from_iter([
            "fonts",
            "jw-icons-external-1970474.woff",
        ])),
    ];

    let mut missing_paths = vec![];

    for path in &styles_path {
        if !path.exists() {
            missing_paths.push(path.to_string_lossy().to_string());
        }
    }

    if missing_paths.is_empty() {
        Ok(())
    } else {
        Err(missing_paths)
    }
}

pub fn appdata_handler<'a>(
    resolved_path: &'a PathBuf,
) -> Result<Response<Vec<u8>>, Box<dyn std::error::Error>> {
    let media_data = std::fs::read(resolved_path)?;
    Ok(Response::builder()
        .status(200)
        .header("Access-Control-Allow-Origin", "*")
        .header("Origin", "*")
        .body(media_data)?)
}
