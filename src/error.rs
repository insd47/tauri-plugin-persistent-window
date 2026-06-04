pub type Result<T> = std::result::Result<T, Error>;

/// Errors raised while wiring up the plugin (e.g. building the tray icon).
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Tauri(#[from] tauri::Error),
}
