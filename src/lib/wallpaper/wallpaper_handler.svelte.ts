import { apply_saved_wallpapers, set_saved_wallpapers } from "./saved_wallpapers.svelte";
import { listen } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/core";

var wallpaper_directory: string = $state("");
var selected_monitor: string = $state("");

export async function set_wallpaper(id: string, args: string[]) {
  if (selected_monitor == "") return
  await invoke('set_wallpaper', { monitor: selected_monitor, id: id, args: args })
  await set_saved_wallpapers(id, selected_monitor)
}

export function set_selected_monitor(monitor: string) {
  selected_monitor = monitor
}

export function set_wallpaper_directory(directory: string) {
  wallpaper_directory = directory
}

export function get_wallpaper_directory(): string {
  return wallpaper_directory;
}

listen<string[]>("monitor-connected", (event) => {
  for (let i = 0; i < event.payload.length; i++) {
    apply_saved_wallpapers()
  }
})
