import { Child, Command } from "@tauri-apps/plugin-shell"

var wallpapers = new Map<string, Child>()
var selected_monitor: string = $state("");

export async function set_wallpaper(id: string) {
  console.log(selected_monitor, id)
  const cmd = Command.create("exec-sh", [
    '-c',
    //TODO: Add back --screen-root and --bg for final release
    `linux-wallpaperengine ${id}`
  ])
  var child = wallpapers.get(selected_monitor)
  if (child !== undefined) child.kill();
  wallpapers.set(selected_monitor, await cmd.spawn())
}

export function set_selected_monitor(monitor: string) {
  selected_monitor = monitor
}