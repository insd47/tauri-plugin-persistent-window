use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Runtime,
};

const OPEN: &str = "persistent-window:open";
const QUIT: &str = "persistent-window:quit";

/// Background tray icon for desktops without a dock (Windows, Linux).
///
/// Linux note: most environments do not deliver tray *click* events, so
/// the menu item is the reliable re-open path there.
pub fn build<R: Runtime>(app: &AppHandle<R>) -> crate::Result<()> {
    let name = app.config().product_name;
    let open = MenuItem::with_id(app, OPEN, format!("Open {name}"), true, None::<&str>)?;
    let quit = MenuItem::with_id(app, QUIT, format!("Quit {name}"), true, None::<&str>)?;
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
