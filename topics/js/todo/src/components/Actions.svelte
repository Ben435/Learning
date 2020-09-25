<script>
    import { goto, prefetch } from '@sapper/app';

    let open = false;

    const onToggleActions = () => open = !open;

    const onAdd = () => {
        goto('/new');
    };

    const onHoverAdd = () => {
        prefetch('/new');
    }

    const onArchive = () => {
        goto('/archive')
    };

    const onHoverArchive = () => {
        prefetch('/archive');
    }
</script>

<div class="actions">
    <div class={`actions--container ${open ? 'open' : ''}`}>
        <button class="actions--action" on:mouseover={onHoverAdd} on:click={onAdd}>Add</button>
        <button class="actions--action" on:mouseover={onHoverArchive} on:click={onArchive}>Archive</button>
    </div>
    <div class="actions--opener">
        <button class="actions--action" on:click={onToggleActions}>Open</button>
    </div>
</div>

<style lang='less'>
    @import '../mixins.less';

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
            min-width: 4em;
            min-height: 4em;
            overflow: hidden;
            flex: 1;
            align-items: center;
            justify-content: center;
            margin: 1em 0 0 0;
            padding: 0;
            box-shadow: 0.1em 0.1em 0.1em 0.3em #eee;
            border: 0;
            border-radius: 90px;

            .button-mixin();
        }
    }
</style>
