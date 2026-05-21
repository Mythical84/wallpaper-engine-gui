use display_info::DisplayInfo;

#[tauri::command]
fn get_monitors() -> Vec<String> {
    let mut monitors: Vec<String> = Vec::<String>::new();
    let infos = DisplayInfo::all().unwrap();
    for info in infos {
        monitors.push(info.name);
    }

    return monitors
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_monitors])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
