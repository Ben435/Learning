<script>
    import TodoItem from './TodoItem.svelte';

    export let items = [];
    export let onEditCallback = () => {};
    export let onDeleteCallback = () => {};
</script>

<div class="todo-container">
    {#each items as item }
    <TodoItem item={item} onEdit={() => onEditCallback(item)} onDelete={() => onDeleteCallback(item)}/>
    {/each}
</div>

<style>
    .todo-container {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
    }
</style>
