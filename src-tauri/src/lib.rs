use std::collections::HashMap;
use std::env::home_dir;
use std::fs;
use std::process::{Child, Command, exit};
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use display_info::DisplayInfo;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager, State, WebviewWindowBuilder};

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

                if monitors.len() > last_count && last_count > 0 {
                    apply_saved_wallpapers(app.state::<Mutex<bool>>(), app.state::<Mutex<HashMap<String, Child>>>());
                }

                last_count = monitors.len();
            }

            std::thread::sleep(Duration::from_millis(500));
        }
    });
}

#[tauri::command]
fn apply_saved_wallpapers(new_window: State<'_, Mutex<bool>>, wallpapers: State<'_, Mutex<HashMap<String, Child>>>) -> (bool, String) {
    let mut path = home_dir().unwrap();
    path.push(".local/share/wallpaper-engine-gui/saved_wallpapers.json");
    if !path.exists() {
        return (false, String::new());
    }
    let contents = fs::read_to_string(path).unwrap();
    let parsed = json::parse(&contents).unwrap();
    let path = parsed["path"].to_owned().to_string();

    if new_window.lock().unwrap().clone() { return (true, path); }
    
    let mut map = wallpapers.lock().unwrap();
    for i in 0..parsed["wallpapers"].len() {
        let mut cmd = Command::new("linux-wallpaperengine");
        cmd.args([
            "--screen-root",
            parsed["wallpapers"][i]["monitor"].as_str().unwrap(),
            "--bg",
            parsed["wallpapers"][i]["id"].as_str().unwrap(),
        ]);
        map.insert(parsed["wallpapers"][i]["monitor"].as_str().unwrap().to_owned(), cmd.spawn().unwrap()); 
    }
    return (true, path);
}

#[tauri::command]
fn set_wallpaper(state: State<'_, Mutex<HashMap<String, Child>>>, monitor: &str, id: &str, args: Vec<&str>) {
    let mut map = state.lock().unwrap();
    if map.contains_key(monitor) {
        map.get_mut(monitor).unwrap().kill().unwrap();
    }
    // let mut cmd = Command::new(format!("sh -c linux-wallpaperengine --screen-root {monitor} --{args} --bg {id}"));
    let mut cmd = Command::new("linux-wallpaperengine");
    cmd.args([
        "--screen-root",
        monitor,
        "--bg",
        id
    ]);
    map.insert(monitor.to_owned(), cmd.spawn().unwrap()); 
}


#[tauri::command]
fn check_new_window(state: State<'_, Mutex<bool>>) -> bool {
    return state.lock().unwrap().clone();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            start_monitor_watcher(app.handle().clone());
            app.manage(Mutex::new(HashMap::<String, Child>::new()));
            app.manage(Mutex::new(false));
            let quit_i = MenuItem::with_id(app, "quit", "quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "show", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        let state = app.state::<Mutex<HashMap<String, Child>>>();
                        let mut map = state.lock().unwrap();
                        for v in map.values_mut() {
                            v.kill().unwrap();
                        }
                        exit(0);
                    },
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
                            let state = app.state::<Mutex<bool>>();
                            *state.lock().unwrap() = true;
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
        .invoke_handler(tauri::generate_handler![get_monitors, set_wallpaper, check_new_window, apply_saved_wallpapers])
        .build(tauri::generate_context!())?
        .run(|_, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
    Ok(())
}