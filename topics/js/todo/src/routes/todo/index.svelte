<script>
    import Actions from '../../components/Actions.svelte';
    import TodoItemContainer from '../../components/TodoItemContainer.svelte';
    import { todos } from '../../stores';

    const dummyItems = [
        { title: 'test', content: 'hello world!' },
        { content: 'bye bye!' },
    ];

    todos.set(dummyItems);
</script>

<svelte:head>
	<title>Todos</title>
</svelte:head>

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
