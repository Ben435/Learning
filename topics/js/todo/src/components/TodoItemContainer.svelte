<script>
    import TodoItem from './TodoItem.svelte';

    export let items = [];
</script>

<div class="todo-container">
    {#each items as item }
    <TodoItem item={item}/>
    {/each}
</div>

<style>
    .todo-container {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
    }
</style>
