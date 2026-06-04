// The plugin exposes no IPC commands; every API is Rust-side.
const COMMANDS: &[&str] = &[];

fn main() {
  tauri_plugin::Builder::new(COMMANDS).build();
}
