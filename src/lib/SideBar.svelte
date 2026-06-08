<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { set_wallpaper } from "./wallpaper/wallpaper_handler.svelte";

  let { wallpaper } = $props()

  let preview: any = $state()

  $effect(() => {
    if (wallpaper != undefined) {
      preview = convertFileSrc(wallpaper.preview)
    }
  })

  async function click() {
    await set_wallpaper(wallpaper.id.toString(), [""])
  }

</script>

<main>
  {#if wallpaper !== undefined}
    <img src={preview} alt="sadge" />
    <p>{wallpaper.title}</p>
    <button onclick={click}>
      Apply
    </button>
  {/if}
</main>

<style>
  main {
    grid-area: c;
    width: 27vw;
    height: 100vh;
    background-color: #11111b;
  }

  img {
    width: 15vw;
    height: 15vw;
    margin: auto;
    margin-top: 20px;
    display: block;
  }

  p {
    text-align: center;
    color: #cdd6f4;
  }

  button {
    display: block;
    margin: auto;
    background-color: #74c7ec;
    border: none;
    width: 15vw;
    height: 3vw;
  }

  button:active {
    background-color: #89b4fa;
  }
</style>
