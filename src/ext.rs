use tauri::{Manager, Runtime, WebviewWindow};

use crate::registry::Registry;

/// Per-window persistence controls on [`WebviewWindow`].
pub trait PersistentExt {
  /// When `true`, closing this window hides it (keeping the app running) instead
  /// of destroying it. Restore it via [`crate::reopen`] (tray / dock).
  fn set_persistent(&self, persistent: bool);

  /// Whether this window is currently persistent.
  fn is_persistent(&self) -> bool;
}

impl<R: Runtime> PersistentExt for WebviewWindow<R> {
  fn set_persistent(&self, persistent: bool) {
    self.state::<Registry>().set(self.label(), persistent);
  }

  fn is_persistent(&self) -> bool {
    self.state::<Registry>().is_persistent(self.label())
  }
}
