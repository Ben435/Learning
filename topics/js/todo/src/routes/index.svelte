<script>
	import { fetchTodos } from './_todo-api';
    import Actions from '../components/Actions.svelte';
    import TodoItemContainer from '../components/TodoItemContainer.svelte';
	import { todos } from '../stores';

	fetchTodos()
		.then(resp => resp.data)
		.then(todoItems => {
			console.log('Setting todos!', todoItems);
			todos.set(todoItems);
		});
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
