<script lang="ts">
    import Disks from "./disks.svelte";

    export let disks: DiskInfo[];
    export let openDisk: Function;
</script>

<div class="sidebar">
    <Disks bind:disks bind:openDisk />
</div>

<style>
    .sidebar {
        width: 19vw;
        height: 100%;

        border-right: 1px solid #ddd;

        padding: 0.5vw;
    }
</style>
