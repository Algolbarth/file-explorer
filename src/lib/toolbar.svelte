<script lang="ts">
    export let current_path: string;
    export let history: string[];
    export let openDir: Function;

    function goBack() {
        if (history.length > 1) {
            const prev = history.pop();
            openDir(prev, true);
        }
    }
</script>

<div class="toolbar">
    <button class="return" on:click={goBack}>Return</button>

    <div class="path">
        {current_path}
    </div>
</div>

<style>
    .toolbar {
        display: flex;
        justify-content: left;
        align-items: center;
        padding: 0.5vh;
    }

    .return {
        background: rgb(32, 140, 255);
        border: solid;
    }

    .return:hover {
        background: rgb(22, 84, 150);
        color: inherit;
    }

    .path {
        background: rgb(50, 50, 50);
        width: 100%;
        margin-left: 0.5vw;
        padding: 0.5vh;
    }
</style>
