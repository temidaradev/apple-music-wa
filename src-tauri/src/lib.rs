// #[cfg(target_os = "linux")]
// use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            // #[cfg(target_os = "linux")]
            // {
            //     let app = _app;
            //     use webkit2gtk::SettingsExt;
            //     use webkit2gtk::WebViewExt;

            //     if let Some(window) = app.get_webview_window("main") {
            //         let _ = window.with_webview(|webview| {
            //             #[cfg(target_os = "linux")]
            //             {
            //                 use webkit2gtk::WebViewExt;

            //                 if let Some(settings) = webview.settings() {
            //                     settings.set_enable_encrypted_media(true);

            //                     settings.set_enable_media_capabilities(true);

            //                     settings.set_hardware_acceleration_policy(
            //                         webkit2gtk::HardwareAccelerationPolicy::Always,
            //                     );

            //                     settings.set_enable_webgl(true);

            //                     settings.set_enable_media_stream(true);

            //                     settings.set_enable_javascript(true);

            //                     settings.set_allow_modal_dialogs(true);
            //                 }
            //             }
            //         });
            //     }
            // }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
