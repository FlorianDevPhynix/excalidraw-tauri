use std::sync::Mutex;

use tauri::{AppHandle, Manager, Runtime, State, WebviewWindow};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};
use tauri_plugin_store::{Store, StoreExt};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    pub theme: tauri::Theme,
    pub default_sidebar_docked_preference: bool,
    pub view_mode_enabled: bool,
    pub zen_mode_enabled: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            theme: tauri::Theme::Light,
            default_sidebar_docked_preference: true,
            view_mode_enabled: false,
            zen_mode_enabled: false,
        }
    }
}

impl AppState {
    pub fn from_manager<R: tauri::Runtime>(manager: impl Manager<R>) -> Self {
        Self::from_store(SettingsStore::from_manager(&manager))
    }

    pub fn from_store<R: tauri::Runtime>(store: &impl core::ops::Deref<Target = Store<R>>) -> Self {
        Self {
            theme: store
                .get("theme")
                .and_then(|v| match serde_json::from_value(v) {
                    Ok(theme) => Some(theme),
                    Err(err) => {
                        log::warn!("Failed to deserialize theme from store: {err}");
                        None
                    }
                })
                .unwrap_or_else(|| Self::default().theme),
            default_sidebar_docked_preference: store
                .get("defaultSidebarDockedPreference")
                .and_then(|v| v.as_bool())
                .unwrap_or_else(|| Self::default().default_sidebar_docked_preference),
            view_mode_enabled: store
                .get("viewModeEnabled")
                .and_then(|v| v.as_bool())
                .unwrap_or_else(|| Self::default().view_mode_enabled),
            zen_mode_enabled: store
                .get("zenModeEnabled")
                .and_then(|v| v.as_bool())
                .unwrap_or_else(|| Self::default().zen_mode_enabled),
        }
    }

    pub fn store<R: tauri::Runtime>(&self, store: &impl core::ops::Deref<Target = Store<R>>) {
        store.set("theme", self.theme.to_string());
        store.set(
            "defaultSidebarDockedPreference",
            self.default_sidebar_docked_preference,
        );
        store.set("viewModeEnabled", self.view_mode_enabled);
        store.set("zenModeEnabled", self.zen_mode_enabled);
    }
}

pub struct SettingsStore<R: Runtime = tauri::Wry>(Store<R>);

impl<R: tauri::Runtime> SettingsStore<R> {
    pub fn init(manager: &mut impl Manager<R>) {
        let settings_store = manager
            .store_builder("settings.json")
            .auto_save(std::time::Duration::from_secs(30))
            .build();

        debug_assert!(manager.manage(Self(settings_store)));
    }

    pub fn from_manager<'s, M: Manager<R>>(manager: &'s M) -> &'s Self {
        manager.state::<Self>().inner()
    }

    pub fn from_store(store: Store<R>) -> Self {
        Self(store)
    }
}

impl<R: tauri::Runtime> core::ops::Deref for SettingsStore<R> {
    type Target = Store<R>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[tauri::command]
pub fn get_app_state(state: State<SettingsStore>) -> tauri::Result<AppState> {
    Ok(AppState::from_store(state.inner()))
}

#[tauri::command]
pub async fn set_app_state(
    app: AppHandle,
    state: State<'_, SettingsStore>,
    new_state: AppState,
) -> tauri::Result<()> {
    new_state.store(state.inner());
    log::info!("set_app_state: {:?}", new_state);
    Ok(())
}

/* pub struct AppStateStore(Mutex<AppState>);

impl AppStateStore {
    pub fn new(app_state: AppState) -> Self {
        Self(Mutex::new(app_state))
    }
}

impl core::ops::Deref for AppStateStore {
    type Target = Mutex<AppState>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
} */
