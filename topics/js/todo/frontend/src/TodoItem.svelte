<script>
    export let item;
</script>

<div class="todo-item">
    <p>{item.content}</p>
</div>

<style>
    .todo-item {
        flex: 1;
        flex-grow: 0;
        padding: 1em;
        margin: 0.7em;
        box-shadow: 2px 2px 0.5em 0.5em #ddd;
        max-width: 10em;
        min-width: 10em;
        min-height: 10em;
    }
</style>
