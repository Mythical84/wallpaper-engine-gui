<script lang="ts">
	import { homeDir, join } from "@tauri-apps/api/path";
	import Card from "../lib/Card.svelte";
	import { readDir } from "@tauri-apps/plugin-fs";
	import { Wallpaper } from "$lib/wallpaper/wallpaper";
	import { read_json } from "$lib/wallpaper/json_tools";
	import MenuBar from "$lib/MenuBar.svelte";
	import SideBar from "$lib/SideBar.svelte";
    import { init } from "$lib/wallpaper/data_manager";

	const WALLPAPER_PATH = ".steam/steam/steamapps/workshop/content/431960";

	let selected_wallpaper = $state()

	async function get_img_paths(): Promise<Wallpaper[]> {
		await init()
		var wallpapers: Wallpaper[] = [];
		const dirPath = await join(await homeDir(), WALLPAPER_PATH);
		const dirs = await readDir(dirPath);

		for (var i = 0; i < dirs.length; i++) {
			var json = await read_json(dirs[i].name)
			var wallpaper = new Wallpaper(json, dirs[i].name, await join(dirPath, dirs[i].name, json.preview))
			wallpapers.push(wallpaper)
		}

		return wallpapers;
	}

	function click(wallpaper: Wallpaper) {
		selected_wallpaper = wallpaper
	}

</script>

<main>
	<MenuBar />
	<div id="container">
		{#await get_img_paths()}
			...Reading Files
		{:then wallpapers}
			{#each wallpapers as wallpaper}
				<Card {wallpaper} onclick={click} />
			{/each}
		{/await}
	</div>
	<SideBar wallpaper={selected_wallpaper}/>
</main>

<style>
	:global(body) {
		background: #1e1e2e;
		overflow: hidden;
	}

	main {
		background: #1e1e2e;
		display: grid;
		grid-template-columns: 70vw 27vw;
		grid-template-rows: 6vh 93vh;
		grid-template-areas:
			"a c"
			"b c"
	}

	#container {
		grid-area: b;
		display: grid;
		grid-template-columns: auto auto auto auto auto auto;
		row-gap: 0.5714285714vw;
		overflow-y: scroll;
		overflow-x: hidden;
	}
</style>
