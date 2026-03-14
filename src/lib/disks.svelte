<script lang="ts">
    export let disks: DiskInfo[];
    export let openDisk: Function;
</script>

<div class="disks">
    <h3>Disks</h3>

    {#each disks as disk, index}
        {#if index > 0}
            <hr />
        {/if}

        <button class="item" on:click={() => openDisk(disk.mount)}>
            <b>{disk.name}</b> ({disk.mount})
        </button>
    {/each}
</div>
