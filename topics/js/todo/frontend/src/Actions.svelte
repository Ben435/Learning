<script>

    let open = false;

    const onToggleActions = () => open = !open;

    const onAdd = () => {

    };

    const onViewArchive = () => {

    };
</script>
<div class="actions">
    <div class={`actions-container ${open ? 'open' : ''}`}>
        <button class="action" on:click={onAdd}>Add</button>
        <button class="action" on:click={onViewArchive}>Archive</button>
    </div>
    <button class="action" on:click={onToggleActions}>Open</button>
</div>

<style>
    .actions {
        position: fixed;
        bottom: 2em;
        right: 2em;
    }
    .actions-container {
        max-width: 4em;
        display: flex;
        flex-direction: column;
    }
    .actions-container:not(.open) {
        display: none;
    }
    .action {
        width: 7em;
        overflow: hidden;
        flex: 1;
        align-items: center;
        margin: 1em 0 0 0;
        padding: 1.2em;
        box-shadow: 0.1em 0.1em 0.1em 0.3em #eee;
        border: 0;
        border-radius: 90px;

        background-image: linear-gradient(-45deg, #eee, #fff);
        transition: background-image 1s;
    }

    .action:hover {
        background-image: linear-gradient(-45deg, #fff, #fff);
    }
</style>
