use std::process::exit;
use std::time::Duration;

use display_info::DisplayInfo;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Emitter, Manager, WebviewWindowBuilder};

#[tauri::command]
fn get_monitors() -> Vec<String> {
    let mut monitors: Vec<String> = Vec::<String>::new();
    let infos = DisplayInfo::all().unwrap();
    for info in infos {
        monitors.push(info.name);
    }

    return monitors;
}

fn start_monitor_watcher(app: AppHandle) {
    std::thread::spawn(move || {
        let mut last_count = 0;

        loop {
            if let Some(window) = app.get_webview_window("main") {
                let monitors = window.available_monitors().unwrap_or_default();

                if monitors.len() > last_count {
                    let _ = app.emit("monitor-connected", get_monitors());
                }

                last_count = monitors.len();
            }

            std::thread::sleep(Duration::from_millis(500));
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            start_monitor_watcher(app.handle().clone());
            let quit_i = MenuItem::with_id(app, "quit", "quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "show", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => exit(0),
                    "show" => {
                        if app.get_webview_window("main") == None {
                            WebviewWindowBuilder::new(
                                app,
                                "main",
                                tauri::WebviewUrl::App("index.html".into())
                            )
                            .title("wallpaper engine gui")
                            .inner_size(1200.0, 900.0)
                            .build()
                            .unwrap();
                        }
                    }
                    _ => {
                        panic!("Unsupported menu option")
                    }
                })
                .build(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                window.destroy().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![get_monitors])
        .build(tauri::generate_context!())?
        .run(|_, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
    Ok(())
}
