<script lang="ts">
	import { homeDir, join } from "@tauri-apps/api/path";
	import Card from "../lib/Card.svelte";
	import { readDir } from "@tauri-apps/plugin-fs";
	import { Wallpaper } from "$lib/wallpaper/wallpaper";
	import { read_json } from "$lib/wallpaper/json_tools";
	import MenuBar from "$lib/MenuBar.svelte";

	const WALLPAPER_PATH = ".steam/steam/steamapps/workshop/content/431960";


	async function get_img_paths(): Promise<{img: string, wallpaper: Wallpaper}[]> {
		var imgs: {img: string, wallpaper: Wallpaper}[] = [];
		const dirPath = await join(await homeDir(), WALLPAPER_PATH);
		const dirs = await readDir(dirPath);

		for (var i = 0; i < dirs.length; i++) {
			var wallpaper = new Wallpaper(await read_json(dirs[i].name))
			var preview = wallpaper.preview;
			imgs.push({img: await join(dirPath, dirs[i].name, preview), wallpaper: wallpaper})
		}

		return imgs;
	}
</script>

<main>
	<MenuBar />
	<div id="container">
		{#await get_img_paths()}
			...Reading Files
		{:then imgs}
			{#each imgs as img}
				<Card img={img.img} wallpaper={img.wallpaper} />
			{/each}
		{/await}
	</div>
	<div id="controls"></div>
</main>

<style>
	:global(body) {
		overflow: hidden;
		background: #1e1e2e;
	}

	main {
		background: #1e1e2e;
		display: flex;
		justify-content: space-around;
		flex-wrap: wrap;
	}

	#container {
		display: grid;
		grid-template-columns: auto auto auto auto auto auto;
		row-gap: 0.5714285714vw;
		width: 70vw;
		height: 90vh;
		overflow-y: scroll;
		overflow-x: hidden;
	}

	#controls {
		width: 27vw;
		height: 100vh;
		background-color: #11111b;
	}
</style>
