<script>
    import { goto } from '@sapper/app';
    export let item;

    const onEdit = () => {
        goto(`/${item.id}`);
    }
    const onDelete = () => {
        console.warn('TODO!');
    }
</script>

<div class="todo-item">
    <div class="todo-item--header">
        {#if item.title}
        <h3 class="todo-item--title">{item.title}</h3>
        {/if}
        <div class="todo-item--actions">
            <button on:click={onEdit}>E</button>
            <button on:click={onDelete}>D</button>
        </div>
    </div>
    
    <div class="todo-item--content">
        <p>{item.content}</p>
    </div>
</div>

<style lang="less">
    .todo-item {
        flex: 0 1;
        padding: 0.5em 1em;
        margin: 0.7em;
        box-shadow: 2px 2px 0.5em 0.5em #ddd;
        max-width: 10em;
        min-width: 10em;
        min-height: 10em;

        overflow: hidden;

        &--title {
            margin-top: 0;
            font-size: 1.2em;
        }
        &--header {
            display: flex;
            flex-direction: row;
        }

        &:hover {
            .todo-item--actions {
                & > button {
                    transform: translateY(0px);
                }
            }
        }
        &--actions {
            position: relative;
            flex: 1;
            display: flex;
            flex-direction: row;
            justify-content: flex-end;

            & > button {
                transition: transform 100ms ease-in-out;
                transform: translateY(-50px);

                margin: 0;
                padding: 0.5em;
                border-radius: 1em;
                max-width: 2em;
                height: 2em;
                flex: 1;
                line-height: 0.8;
            }
        }
    }
</style>
