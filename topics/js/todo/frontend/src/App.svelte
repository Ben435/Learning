<script>
    import Actions from './Actions.svelte';
    import TodoItemContainer from './TodoItemContainer.svelte';
    import { todos } from './stores';

    const dummyItems = [
        { content: 'hello world!' },
        { content: 'bye bye!' },
    ];

    todos.set(dummyItems);
</script>

<main>
    <TodoItemContainer/>
    <Actions/>
</main>

<style>
    main {
        background-color: white;
        margin: 0;
        padding: 0;
        height: 100%;
        width: 100%;
    }
</style>
