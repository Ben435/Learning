<script>
    export let item;
</script>

<div class="todo-item">
    {#if item.title}
    <h3 class="todo-item--title">{item.title}</h3>
    {/if}
    <p class="todo-item--content">{item.content}</p>
</div>

<style lang="less">
    .todo-item {
        flex: 1;
        flex-grow: 0;
        padding: 1em;
        margin: 0.7em;
        box-shadow: 2px 2px 0.5em 0.5em #ddd;
        max-width: 10em;
        min-width: 10em;
        min-height: 10em;

        &--title {
            margin-top: 0;
            font-size: 1.2em;
        }
    }
</style>
