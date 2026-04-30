#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .on_window_event(|event| {
        let window = event.window();
        if window.label() != "main" {
            let _ = window.close();
        }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
