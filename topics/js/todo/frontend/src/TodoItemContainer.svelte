<script>
    import TodoItem from './TodoItem.svelte';
    import { todos } from './stores';

    let todoItems = [];
    todos.subscribe(newVals => {
        todoItems = newVals;
    })
</script>

<div class="todo-container">
    {#each todoItems as item }
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
