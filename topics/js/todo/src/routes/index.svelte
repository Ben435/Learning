<script>
    import { goto } from '@sapper/app';
	import { fetchTodos, deleteTodo } from './_todo-api';
    import Actions from '../components/Actions.svelte';
    import TodoItemContainer from '../components/TodoItemContainer.svelte';
    import { LoadingStates, todos } from '../stores';

    let todoItems = [];
    todos.subscribe(newTodos => {
        todoItems = newTodos.items;

        if (newTodos.loadingState === LoadingStates.NEVER_LOADED) {
            todos.set({
                loadingState: LoadingStates.IS_LOADING,
                items: newTodos.items,
            })
            fetchTodos()
                .then(resp => resp.data)
                .then(todoItems => {
                    console.log('Setting todos!', todoItems);
                    todos.set({
                        loadingState: LoadingStates.LOADED,
                        items: todoItems,
                    });
                });
        }
    });
    
    const onEditCallback = item => {
        goto(`/${item.id}`);
    };
    const onDeleteCallback = async(item) => {
        await deleteTodo(item.id);

        todos.update(prev => ({
            loadingState: prev.loadingState,
            items: prev.items.filter(prevItem => prevItem.id !== item.id),
        }));
    };
	
</script>

<svelte:head>
	<title>Todos</title>
</svelte:head>

<main>
    <TodoItemContainer 
        items={todoItems}
        onEditCallback={onEditCallback}
        onDeleteCallback={onDeleteCallback}
    />
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
