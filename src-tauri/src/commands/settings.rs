#[tauri::command]
pub async fn settings_set_webview_theme(webview_window: tauri::WebviewWindow, theme: String) {
    webview_window.set_theme(match theme.as_str() {
        "dark" => Some(tauri::Theme::Dark),
        "light" => Some(tauri::Theme::Light),
        _ => None,
    });
}