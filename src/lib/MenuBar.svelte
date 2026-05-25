<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
    import { set_selected_monitor } from "./wallpaper/wallpaper_handler.svelte";

  async function get_monitors(): Promise<string[]> {
    return await invoke('get_monitors')
  }

  var value = $state("");

  function change() {
    set_selected_monitor(value)
  }
</script>

<main>
  {#await get_monitors()}
    Loading...
  {:then monitors}
    <select name="monitors" onchange={change} bind:value={value}>
      {#each monitors as monitor}
        <option value={monitor}>{monitor}</option>
      {/each}
    </select>
  {/await}
</main>

<style>
  main {
    grid-area: a;
    width: 100vw;
    height: 6vh;
  }
</style>