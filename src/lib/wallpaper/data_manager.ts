import { homeDir, join } from "@tauri-apps/api/path"
import { open, exists, mkdir, readTextFile } from "@tauri-apps/plugin-fs"
import { set_selected_monitor, set_wallpaper } from "./wallpaper_handler.svelte"

export const appdata = ".local/share/wallpaper-engine-gui"

export async function manager_init() {
  if (!await exists(await join(await homeDir(), appdata))) {
    await mkdir(await join(await homeDir(), appdata))
  }
}

export async function apply_saved_wallpapers() {
  const saved = await join(await homeDir(), appdata, "saved_wallpapers.json")
  if (!await exists(saved)) return;
  const file = JSON.parse(await readTextFile(saved))
  for (let wallpaper in file.wallpapers) {
    set_selected_monitor(file.wallpapers[wallpaper].monitor)
    await set_wallpaper(file.wallpapers[wallpaper].id, "")
  }
}

export async function set_saved_wallpapers(id: string, monitor: string) {
  const saved = await join(await homeDir(), appdata, "saved_wallpapers.json")
  if (!await exists(saved)) {
    const file = await open(saved, {
      create: true,
      write: true
    });
    const papers = {wallpapers: [{ id: id, monitor: monitor }]}
    await file.write(new TextEncoder().encode(JSON.stringify(papers)))
    await file.close()
  } else {
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
}