use core::ops::Deref;

use tauri::{AppHandle, Manager, State, WebviewWindow};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

mod app_state;

#[cfg(debug_assertions)]
#[tauri::command]
fn open_devtools(webview_window: WebviewWindow) {
    webview_window.open_devtools();
}

#[cfg(debug_assertions)]
#[tauri::command]
fn reload_page(webview_window: WebviewWindow) {
    if let Err(error) = webview_window.eval("window.location.reload()") {
        log::error!("executing javascript to reload the page failed: {error}");
    }
}

#[cfg(debug_assertions)]
#[tauri::command]
fn restart_app(app: AppHandle) {
    app.restart();
}

#[tauri::command]
fn open_dialog(webview_window: WebviewWindow) -> String {
    let result = webview_window
        .dialog()
        .message("Hello World!")
        .buttons(MessageDialogButtons::Ok)
        .blocking_show();
    format!("Dialog, was shown. Result: {}", result)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri_plugin_window_state::StateFlags;
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .with_state_flags(StateFlags::all() - StateFlags::VISIBLE)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            #[cfg(debug_assertions)]
            reload_page,
            #[cfg(debug_assertions)]
            open_devtools,
            #[cfg(debug_assertions)]
            restart_app,
            open_dialog,
            crate::app_state::get_app_state,
            crate::app_state::set_app_state,
        ])
        .setup(|app| {
            crate::app_state::SettingsStore::init(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
