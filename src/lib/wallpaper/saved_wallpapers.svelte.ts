import { homeDir, join } from "@tauri-apps/api/path"
import { open, exists, mkdir, readTextFile } from "@tauri-apps/plugin-fs"
import { set_selected_monitor, set_wallpaper } from "./wallpaper_handler.svelte"
import { invoke } from "@tauri-apps/api/core"

export const appdata = ".local/share/wallpaper-engine-gui"
let wallpaper_path = $state("")

export async function manager_init() {
  if (!await exists(await join(await homeDir(), appdata))) {
    await mkdir(await join(await homeDir(), appdata))
  }
}

export async function set_wallpaper_path(path: string) {
  wallpaper_path = path
  const file = await open(await join(await homeDir(), appdata, "saved_wallpapers.json"), { create: true, write: true })
  await file.write(new TextEncoder().encode(JSON.stringify({path: path, wallpapers: []})))
  await file.close()
}

export function get_wallpaper_path() {
  return wallpaper_path
}

export async function apply_saved_wallpapers(): Promise<boolean> {
  const saved = await join(await homeDir(), appdata, "saved_wallpapers.json")
  if (!await exists(saved)) return false;
  const file = JSON.parse(await readTextFile(saved))
  wallpaper_path = file.path
  if (await invoke("check_new_window")) return true;
  for (let wallpaper in file.wallpapers) {
    set_selected_monitor(file.wallpapers[wallpaper].monitor)
    await set_wallpaper(file.wallpapers[wallpaper].id, [""])
  }
  return true;
}

export async function set_saved_wallpapers(id: string, monitor: string) {
  const saved = await join(await homeDir(), appdata, "saved_wallpapers.json")
  const presets = JSON.parse(await readTextFile(saved))
  let set = false
  for (let wallpaper in presets.wallpapers) {
    if (presets.wallpapers[wallpaper].monitor === monitor) {
      presets.wallpapers[wallpaper].id = id
      set = true
    }
  }
  if (!set) {
    presets.wallpapers.push({ id: id, monitor: monitor })
  }
  const file = await open(saved, {
    write: true,
  });
  await file.write(new TextEncoder().encode(JSON.stringify(presets)))
  await file.close()
}