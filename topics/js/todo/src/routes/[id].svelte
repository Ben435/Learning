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

<main>
    <h1>Edit</h1>
    {#if original}
    <input bind:value={title}/>
    <textarea contenteditable="true" bind:value={content}/>
    {/if}
    <button on:click={onCancel}>Cancel</button>
    <button on:click={onSubmit}>Submit</button>
</main>
