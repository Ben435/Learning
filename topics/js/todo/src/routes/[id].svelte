<script context="module">
    export async function preload(page) {
        const { id } = page.params;

        return { id };
    }
</script>

<script>
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
        await editTodo(original.id, {
            id: original.id,
            title,
            content,
        });

        goto('/');
    }
</script>

<main class="edit-todo">
    <div class="edit-todo--title">
        <h1>Edit</h1>
    </div>
    <div class="edit-todo--form">
        {#if original}
        <input bind:value={title}/>
        <textarea contenteditable="true" bind:value={content}/>
        {/if}
    </div>
    <div class="edit-todo--actions">
        <button on:click={onCancel}>Cancel</button>
        <button on:click={onSubmit}>Submit</button>
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
