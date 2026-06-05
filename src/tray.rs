use tauri::{
  menu::{Menu, MenuItem},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  AppHandle, Runtime,
};

const OPEN: &str = "persistent-window:open";
const QUIT: &str = "persistent-window:quit";

/// Background tray icon for desktops without a dock (Windows, Linux).
///
/// Left click and the "열기" menu item reopen the app; "종료" quits. The app's
/// default window icon is used when available.
///
/// Linux note: tray *click* events are not delivered by most environments, so
/// the menu item is the reliable reopen path there.
pub fn build<R: Runtime>(app: &AppHandle<R>) -> crate::Result<()> {
  let open = MenuItem::with_id(app, OPEN, "Open", true, None::<&str>)?;
  let quit = MenuItem::with_id(app, QUIT, "Quit", true, None::<&str>)?;
  let menu = Menu::with_items(app, &[&open, &quit])?;

  let mut builder = TrayIconBuilder::<R>::new()
    .menu(&menu)
    .show_menu_on_left_click(false)
    .on_menu_event(|app, event| match event.id.as_ref() {
      OPEN => crate::reopen(app),
      QUIT => app.exit(0),
      _ => {}
    })
    .on_tray_icon_event(|tray, event| {
      if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
      } = event
      {
        crate::reopen(tray.app_handle());
      }
    });

  if let Some(icon) = app.default_window_icon() {
    builder = builder.icon(icon.clone());
  }

  builder.build(app)?;
  Ok(())
}
