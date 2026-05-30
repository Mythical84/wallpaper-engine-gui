import { Child, Command } from "@tauri-apps/plugin-shell"
import { apply_saved_wallpapers, set_saved_wallpapers } from "./data_manager";
import { listen } from "@tauri-apps/api/event"

var wallpapers = new Map<string, Child>()
var selected_monitor: string = $state("");

export async function set_wallpaper(id: string, args: string) {
  if (selected_monitor == "") return
  const cmd = Command.create("exec-sh", [
    '-c',
    //TODO: Add back --screen-root and --bg for final release
    `linux-wallpaperengine --screen-root ${selected_monitor} ${args} --bg ${id}`
  ])
  var child = wallpapers.get(selected_monitor)
  if (child !== undefined) child.kill();
  wallpapers.set(selected_monitor, await cmd.spawn())
  await set_saved_wallpapers(id, selected_monitor)
}

export function set_selected_monitor(monitor: string) {
  selected_monitor = monitor
}

listen<string[]>("monitor-connected", (event) => {
  for (let i = 0; i < event.payload.length; i++) {
    apply_saved_wallpapers()
  }
})