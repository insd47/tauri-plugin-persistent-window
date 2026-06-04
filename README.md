# tauri-plugin-persistent-window

Persist Tauri windows across close. A window marked **persistent** hides instead
of being destroyed when the user closes it, keeping the app alive in the
background, and is brought back from the **tray** (Windows/Linux) or the **dock**
(macOS).

## Install

```toml
# Cargo.toml
[dependencies]
tauri-plugin-persistent-window = "0.1"
```

## Usage

```rust
use tauri_plugin_persistent_window::PersistentExt;

tauri::Builder::default()
    .plugin(tauri_plugin_persistent_window::init())
    .setup(|app| {
        let window = app.get_webview_window("main").unwrap();
        window.set_persistent(true); // closing it now hides instead of quitting
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
```

That's the whole setup. Closing a persistent window hides it; clicking the tray
icon (Windows/Linux) or the dock icon (macOS) brings the most recently hidden
window back.

## API

- `window.set_persistent(bool)` / `window.is_persistent()` — per-window toggle
  (on `WebviewWindow`).
- `reopen(&AppHandle)` — restore the last hidden window **if none are visible**.
  Wired internally to the tray and the dock; exposed for custom triggers.
- `single_instance(&AppHandle, Vec<String>, String)` — drop-in callback for the
  single-instance plugin (see below).

## Reopen triggers

| Trigger | Handled by | Needs single-instance? |
| --- | --- | --- |
| Tray icon click / menu (Windows, Linux) | this plugin | no |
| Dock icon click (macOS) | this plugin (`RunEvent::Reopen`) | no |
| **Re-launching the executable** (desktop shortcut, Start menu) while hidden | — | **yes** |

Clicking the running app's tray/dock presence is fully covered. The one gap is
**launching a fresh copy** from a shortcut while the app sits hidden in the tray:
that spawns a *separate process* the tray cannot intercept. This only matters on
**Windows/Linux** — macOS reuses the running instance and fires `Reopen` instead.

To cover it, add [`tauri-plugin-single-instance`](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/single-instance)
(it must be the **first** plugin) and hand it this crate's ready-made callback:

```rust
let mut builder = tauri::Builder::default();

#[cfg(desktop)]
{
    builder = builder.plugin(tauri_plugin_single_instance::init(
        tauri_plugin_persistent_window::single_instance,
    ));
}

builder
    .plugin(tauri_plugin_persistent_window::init())
    // ...
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
```

Need to handle launch args yourself? Wrap it:

```rust
tauri_plugin_single_instance::init(|app, argv, cwd| {
    handle_args(&argv, &cwd);
    tauri_plugin_persistent_window::single_instance(app, argv, cwd);
})
```

## Platform notes

- **macOS** — reopens via the dock; no tray is created. Activation policy is left
  as `Regular` so the dock icon stays clickable.
- **Windows** — a tray icon (app icon) is created; left click reopens, right click
  shows the menu.
- **Linux** — a tray icon is created, but most environments do not deliver tray
  *click* events, so use the tray **menu** ("열기") to reopen.

The tray uses the app's default window icon when one is configured.

## License

MIT © insd47
