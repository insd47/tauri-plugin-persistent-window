use tauri::{Manager, Runtime, Window, WindowEvent};

use crate::registry::Registry;

/// Hide-on-close policy.
///
/// A persistent window prevents its own close and hides instead, recording
/// itself as the most recently hidden window. Non-persistent windows close
/// normally.
pub fn on_window_event<R: Runtime>(window: &Window<R>, event: &WindowEvent) {
  let WindowEvent::CloseRequested { api, .. } = event else {
    return;
  };

  let registry = window.state::<Registry>();
  if registry.is_persistent(window.label()) {
    api.prevent_close();
    let _ = window.hide();
    registry.mark_hidden(window.label());
  }
}
