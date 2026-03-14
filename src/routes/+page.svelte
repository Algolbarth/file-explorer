<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import Sidebar from "$lib/sidebar.svelte";
    import "../styles/app.css";
    import Toolbar from "$lib/toolbar.svelte";
    import Files from "$lib/files.svelte";

    let disks: DiskInfo[] = [];
    let files: FileInfo[] = [];
    let current_path: string = "";
    let history: string[] = [];

    onMount(async () => {
        disks = await invoke("list_disks");

        openDisk(disks[0].mount);
    });

    let openDisk = (mount: string) => {
        history = [];
        openDir(mount);
    };

    let openDir = async (path: string, is_back: boolean = false) => {
        try {
            files = await invoke("read_directory", { path });

            if (!is_back) {
                history.push(current_path);
            }

            current_path = path;
        } catch (e) {
            console.error(e);
        }
    };
</script>

<div class="container">
    <Sidebar bind:disks bind:openDisk />

    <div class="main">
        <Toolbar bind:current_path bind:history bind:openDir />

        <Files bind:files bind:openDir />
    </div>
</div>

<style>
    .container {
        position: fixed;
        top: 0;
        left: 0;

        display: grid;
        grid-template-columns: 20vw 80vw;

        width: 100vw;
        height: 100vh;

        font-family: sans-serif;
    }

    .main {
        display: grid;
        grid-template-rows: 0.5fr 9.5fr;
    }
</style>
