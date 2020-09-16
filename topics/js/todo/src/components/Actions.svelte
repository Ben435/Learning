<script>

    let open = false;

    const onToggleActions = () => open = !open;

    const onAdd = () => {
        
    };

    const onViewArchive = () => {

    };
</script>
<div class="actions">
    <div class={`actions--container ${open ? 'open' : ''}`}>
        <a class="actions--action" href='new'>Add</a>
        <a class="actions--action" href='archive'>Archive</a>
    </div>
    <button class="actions--open" on:click={onToggleActions}>Open</button>
</div>

<style lang='less'>
    .actions {
        position: fixed;
        bottom: 2em;
        right: 2em;
        &--container {
            max-width: 4em;
            display: flex;
            flex-direction: column;
            &:not(.open) {
                display: none;
            }
        }
        &--action {
            width: 3em;
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
            &:hover {
                background-image: linear-gradient(-45deg, #fff, #fff);
            }
        }
        &--open {
            margin: 1em 0 0 0;
            padding: 1.2em;
            box-shadow: 0.1em 0.1em 0.1em 0.3em #eee;
            border: 0;
            border-radius: 90px;
        }
    }
</style>
