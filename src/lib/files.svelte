<script lang="ts">
    export let files: FileInfo[];
    export let openDir: Function;
</script>

<div class="files">
    {#each files as file}
        <button class="item" on:click={() => file.is_dir && openDir(file.path)}>
            <img
                src={file.is_dir ? "directory.png" : "file.png"}
                alt={file.is_dir ? "directory" : "file"}
            />
            {file.name}
        </button>

        <br />
    {/each}
</div>

<style>
    .files {
        height: 94vh;
        padding: 0.5vh;
        overflow-y: scroll;
    }

    img {
        width: 1em;
    }
</style>
