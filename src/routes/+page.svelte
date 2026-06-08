<script lang="ts">
	import { homeDir, join } from "@tauri-apps/api/path";
	import Card from "../lib/Card.svelte";
	import { readDir, readTextFile } from "@tauri-apps/plugin-fs";
	import { Wallpaper } from "$lib/wallpaper/wallpaper";
	import MenuBar from "$lib/MenuBar.svelte";
	import SideBar from "$lib/SideBar.svelte";
	import PathFinder from "$lib/PathFinder.svelte";
	import { apply_saved_wallpapers, manager_init } from "$lib/wallpaper/saved_wallpapers.svelte";
	import { set_wallpaper_path, get_wallpaper_path } from "$lib/wallpaper/saved_wallpapers.svelte";

	let selected_wallpaper = $state()
	let loaded = $state(true)
	let loaded_wallpapers: Wallpaper[] = $state([])

	async function read_json(id: string) {
		const json_path = await join(get_wallpaper_path(), id, "project.json");

		const json = await readTextFile(json_path);

		return JSON.parse(json);
	}

	async function get_img_paths() {
		await manager_init()
		loaded = await apply_saved_wallpapers();
		if (!loaded) {
			return;
		}
		var wallpapers: Wallpaper[] = [];
		const dirPath = get_wallpaper_path();
		const dirs = await readDir(dirPath);

		for (var i = 0; i < dirs.length; i++) {
			var json = await read_json(dirs[i].name)
			var wallpaper = new Wallpaper(json, dirs[i].name, await join(dirPath, dirs[i].name, json.preview))
			wallpapers.push(wallpaper)
		}
		loaded_wallpapers = wallpapers
	}

	function card_click(wallpaper: Wallpaper) {
		selected_wallpaper = wallpaper
	}

	async function path_submit(steam_path: string) {
		steam_path = await join(steam_path.replace("~", await homeDir() + "/"), "steamapps/workshop/content/431960")
		await set_wallpaper_path(steam_path)

		await get_img_paths()
	}

</script>

<main>
	<div id="main_container">
		<MenuBar />
		<div id="card_container">
			{#await get_img_paths()}
				...Reading Files
			{:then}
				{#each loaded_wallpapers as wallpaper}
					<Card {wallpaper} onclick={card_click} />
				{/each}
			{/await}
		</div>
		<SideBar wallpaper={selected_wallpaper}/>
	</div>
	{#if !loaded}
		<div id="path_container">
			<PathFinder onsubmit={path_submit}/>
		</div>
	{/if}
</main>

<style>
	:global(body) {
		background: #1e1e2e;
		overflow: hidden;
	}

	main {
		background: #1e1e2e;
	}

	#main_container {
		display: grid;
		grid-template-columns: 70vw 27vw;
		grid-template-rows: 6vh 93vh;
		grid-template-areas:
			"a c"
			"b c"
	}

	#card_container {
		grid-area: b;
		display: grid;
		grid-template-columns: auto auto auto auto auto auto;
		row-gap: 0.5714285714vw;
		overflow-y: scroll;
		overflow-x: hidden;
	}
</style>
