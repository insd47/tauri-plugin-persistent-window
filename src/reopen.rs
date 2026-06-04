use tauri::{AppHandle, Manager, Runtime, WebviewWindow};

use crate::registry::Registry;

/// Restore the app from a click on its persistent presence (tray / dock).
///
/// Acts only when no window is visible — "all windows closed, bring the last
/// one back". A no-op while any window is still on screen.
pub fn reopen<R: Runtime>(app: &AppHandle<R>) {
  if first_visible(app).is_none() {
    restore_last(app);
  }
}

/// Drop-in callback for `tauri-plugin-single-instance`.
///
/// A second launch means the user explicitly asked for the app, so this focuses
/// an already-visible window, or restores the last hidden one if none are shown.
/// It depends only on `tauri` types, so this crate never depends on the
/// single-instance plugin.
///
/// ```ignore
/// tauri_plugin_single_instance::init(tauri_plugin_persistent_window::single_instance)
/// ```
pub fn single_instance<R: Runtime>(app: &AppHandle<R>, _args: Vec<String>, _cwd: String) {
  match first_visible(app) {
    Some(window) => {
      let _ = window.set_focus();
    }
    None => restore_last(app),
  }
}

fn restore_last<R: Runtime>(app: &AppHandle<R>) {
  let registry = app.state::<Registry>();
  let Some(label) = registry.last_hidden() else {
    return;
  };

  if let Some(window) = app.get_webview_window(&label) {
    let _ = window.show();
    let _ = window.set_focus();
  }
  registry.forget_hidden(&label);
}

fn first_visible<R: Runtime>(app: &AppHandle<R>) -> Option<WebviewWindow<R>> {
  app
    .webview_windows()
    .into_values()
    .find(|window| window.is_visible().unwrap_or(false))
}
