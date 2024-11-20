#[tauri::command]
pub async fn settings_set_webview_theme(webview_window: tauri::WebviewWindow, theme: String) {
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        webview_window.set_theme(match theme.as_str() {
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
