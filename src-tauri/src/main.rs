#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Manager, WindowBuilder, WindowUrl};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
        let init_script = r#"
            // Block window.open but return a mock object to prevent video player crashes
            window.open = function(url, target, features) { 
                return {
                    closed: false,
                    close: function() { this.closed = true; },
                    focus: function() {},
                    blur: function() {},
                    postMessage: function() {},
                    location: { href: url || '' },
                    document: { write: function() {}, close: function() {} }
                }; 
            };
            // Intercept clicks on links that try to open in a new tab/window
            document.addEventListener('click', function(e) {
                var target = e.target;
                while (target && target.tagName !== 'A') {
                    target = target.parentNode;
                }
                if (target && target.target === '_blank') {
                    e.preventDefault();
                    e.stopPropagation();
                }
            }, true);
        "#;

        WindowBuilder::new(
            app,
            "main",
            WindowUrl::App("index.html".into())
        )
        .title("Vert")
        .inner_size(1200.0, 800.0)
        .min_inner_size(800.0, 600.0)
        .resizable(true)
        .fullscreen(false)
        .decorations(false)
        .transparent(false)
        .initialization_script(init_script)
        .build()?;

        Ok(())
    })
    .on_window_event(|event| {
        let window = event.window();
        if window.label() != "main" {
            let _ = window.close();
        }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
