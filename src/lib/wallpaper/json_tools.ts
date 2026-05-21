import { homeDir, join } from "@tauri-apps/api/path";
import { readTextFile } from "@tauri-apps/plugin-fs";

export async function read_json(id: string) {
  const WALLPAPER_PATH = ".steam/steam/steamapps/workshop/content/431960";
  const json_path = await join(await homeDir(), WALLPAPER_PATH, id, "project.json");

  const json = await readTextFile(json_path);

  return JSON.parse(json);
}