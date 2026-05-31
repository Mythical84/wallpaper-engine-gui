<script lang="ts">
  import { homeDir, join } from "@tauri-apps/api/path";
  import { open } from "@tauri-apps/plugin-dialog";
  import { exists } from "@tauri-apps/plugin-fs";

  let { onsubmit } = $props()

  let path = $state("")

  function submit() {
    onsubmit?.(path)
  }

  async function detect() {
    let check_paths = [
      await join(await homeDir(), ".steam/steam"),
      await join(await homeDir(), ".local/share/Steam"),
      await join(await homeDir(), ".var/app/com.valvesoftware.Steam/share/Steam"),
      await join(await homeDir(), "snap/steam/common/.local/share/Steam")
    ]

    for (let i in check_paths) {
      console.log(check_paths[i])
      if (await exists(check_paths[i])) {
        path = check_paths[i]
        return
      }
    }
  }

  async function browse() {
    const file = await open({
      multiple: false,
      directory: true
    })

    if (file !== null) {
      path = file
    }
  }
</script>

<main>
  <p>Enter the path for your wallpaper engine workshop directory</p>
  <input type="text" placeholder="~/steam/steam" bind:value={path}/> 
  <button id="detect" onclick={detect}>Auto-Detect</button>
  <button id="browse" onclick={browse}>Browse</button>
  <button id="submit" onclick={submit}>Submit</button>
</main>

<style>
  main {
    color: #cdd6f4;
    width: 40vw;
    height: 40vh;
    background: #181825;
    z-index: 100;
    position: absolute;
    top: 0;
    left: 0;
    border-color: rgba(108, 112, 134, 0.8);
    border-style: solid;
    border-width: 31vh 30vw 31vh 30vw;
    background-clip: padding-box;
    display: grid;
    grid-template-columns: 25vw .3vw auto .3vw auto;
    grid-template-rows: calc(20vh - 2em) 3vh auto 2em 2em;
    grid-template-areas: 
      "a a a a a"
      "b . c . d"
      ". . . . ."
      ". . . . ."
      ". . . e e";
    padding: 10px;
  }

  p {
    color: #cdd6f4;
    text-align: center;
    grid-area: a;
  }

  input {
    grid-area: b;
    background: #313244;
    border: none;
    border-radius: 8px;
    color: inherit;
    caret-color: #cdd6f4;
    text-indent: 5px;
  }

  input:focus {
    outline: none;
  }

  #detect {
    grid-area: c;
    background: #313244;
    color: inherit;
    border: none;
    border-radius: 10px;
  }

  #detect:active {
    background: #45475a;
  }

  #browse {
    grid-area: d;
    background: #313244;
    color: inherit;
    border: none;
    border-radius: 10px;
  }

  #browse:active {
    background: #45475a;
  }

  #submit {
    grid-area: e;
    background: #313244;
    border: none;
    color: inherit;
    border-radius: 5px;
  }

  #submit:active {
    background: #45475a;
  }
</style>