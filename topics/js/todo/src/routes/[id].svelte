<script context="module">
    export async function preload(page) {
        const { id } = page.params;

        return { id };
    }
</script>

<script>
    import { todos } from '../stores';
    import { fetchTodo, editTodo } from "./_todo-api";
    import { goto } from '@sapper/app';

    export let id;
    
    let original;
    let title = '';
    let content = '';

    fetchTodo(id)
        .then(resp => resp.data)
        .then(data => {
            original = data;
            title = data.title || '';
            content = data.content || '';
        });

    const onCancel = () => {
        goto('/');
    };
    const onSubmit = async() => {
        const newItem = {
            id: original.id,
            title,
            content,
        };

        await editTodo(original.id, newItem);

        todos.update(val => ({
            loadingState: val.loadingState,
            items: val.items.map(item => item.id === original.id ? newItem : item),
        }));

        goto('/');
    }
</script>

<main class="edit-todo">
    <div class="edit-todo--title">
        <h1>Edit</h1>
    </div>
    <div class="edit-todo--form">
        {#if original}
        <input class="edit-todo--form__title" bind:value={title}/>
        <textarea class="edit-todo--form__content" contenteditable="true" bind:value={content}/>
        {/if}
    </div>
    <div class="edit-todo--actions">
        <button class="edit-todo--actions__cancel" on:click={onCancel}>Cancel</button>
        <button class="edit-todo--actions__submit" on:click={onSubmit}>Submit</button>
    </div>
</main>

<style lang="less">
    .edit-todo {
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
