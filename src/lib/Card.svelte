<script lang="ts">
	import { convertFileSrc } from "@tauri-apps/api/core";
	import { set_wallpaper } from "./wallpaper/wallpaper_handler.svelte";

	let { img, wallpaper } = $props()

	// svelte-ignore state_referenced_locally
	const assetUrl = convertFileSrc(img);

	// svelte-ignore state_referenced_locally
	const id = img.split("/")[img.split("/").length - 2]

	async function click() {
		await set_wallpaper(id)
	}
</script>

<main>
	<a href="#top" onclick={click}>
		<img src={assetUrl} alt="sadge"/>
	</a>
</main>

<style>
	main {
		width: 11vw;
		height: 11vw;
		float: left;
		position: relative;
		overflow: hidden;
	}

	a:hover::after {
		content: '';
		border: 1px solid #74c7ec;
		position: absolute;
		z-index: 100;
		width: calc(100% - 2px);
		height: calc(100% - 2px);
		right: 0px;
		top: 0px;
	}

	main:hover img {
		transform: scale(1.15);
		transition: transform .3s;
	}

	a {
		background: none;
		border: none;
		width: inherit;
		height: inherit;
	}

	img {
		width: inherit;
		height: inherit;
		transition: transform .3s;
	}
</style>
