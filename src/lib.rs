//! Persist Tauri windows across close.
//!
//! A window marked persistent hides instead of being destroyed when closed, and
//! is brought back from the tray (Windows/Linux) or the dock (macOS).
//!
//! Responsibilities are split per module: [`registry`] owns state, [`lifecycle`]
//! enforces the hide-on-close policy, [`reopen`] restores windows, [`tray`] owns
//! the tray UI, and [`ext`] exposes the per-window API. This file is the
//! composition root that wires them onto the Tauri builder.

mod error;
mod ext;
mod registry;

#[cfg(desktop)]
mod lifecycle;
#[cfg(desktop)]
mod reopen;
#[cfg(all(desktop, not(target_os = "macos")))]
mod tray;

pub use error::{Error, Result};
pub use ext::PersistentExt;
#[cfg(desktop)]
pub use reopen::{reopen, single_instance};

use registry::Registry;
use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

/// Initialize the plugin. Register it once on the Tauri builder.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("persistent-window")
    .setup(|app, _api| {
      app.manage(Registry::default());

      // No dock on these platforms → provide a tray to reopen from.
      #[cfg(all(desktop, not(target_os = "macos")))]
      tray::build(app)?;

      Ok(())
    })
    .on_window_ready(|window| {
      // Attach the hide-on-close policy to every window; it only fires for
      // windows the app has marked persistent.
      #[cfg(desktop)]
      {
        let target = window.clone();
        window.on_window_event(move |event| lifecycle::on_window_event(&target, event));
      }
      #[cfg(not(desktop))]
      let _ = window;
    })
    .on_event(|app, event| {
      // macOS reopens via the dock; other desktops via the tray (see `tray`).
      #[cfg(target_os = "macos")]
      if let tauri::RunEvent::Reopen { .. } = event {
        reopen::reopen(app);
      }
      #[cfg(not(target_os = "macos"))]
      let _ = (app, event);
    })
    .build()
}
