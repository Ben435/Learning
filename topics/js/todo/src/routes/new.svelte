<script>
    import { createNewTodo } from './_todo-api';
    import { todos } from '../stores';
    import * as sapper from '@sapper/app';

    let title = '';
    let content = '';

    const onSubmit = async() => {
        const newTodo = await createNewTodo({ title, content }).then(resp => resp.data)

        todos.update(val => ({
            loadingState: val.loadingState,
            items: val.items.concat([newTodo]),
        }));
        sapper.goto('/');
    }

</script>

<main class="new-todo">
    <div class="new-todo--title">
        <h1>New</h1>
    </div>
    <div class="new-todo--form">
        <input class="new-todo--form__title" bind:value={title}/>
        <textarea class="new-todo--form__content" contenteditable="true" bind:value={content}></textarea>
    </div>
    <div class="new-todo--actions">
        <button class="new-todo--actions__submit" on:click={onSubmit}>Add</button>
    </div>
</main>

<style lang="less">
    .new-todo {
        display: flex;
        flex-direction: column;
        & > {
            flex: 1;
        }

        &--form {
            display: flex;
            flex-direction: column;
        }
    }
</style>

