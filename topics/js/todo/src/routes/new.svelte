<script>
    import { createNewTodo } from './_todo-api';
    import * as sapper from '@sapper/app';

let content = '';

const onSubmit = () => {
    createNewTodo(content)
        .then(() => sapper.goto('/'));
}

</script>

<main class="new-todo">
    <div class="new-todo--actions">
        <h1>New</h1>
    </div>
    <div>
        <textarea contenteditable="true" bind:value={content}></textarea>
    </div>
    <div class="new-todo--actions">
        <button on:click={onSubmit}>Add</button>
    </div>
</main>

<style lang="less">
    .new-todo {
        display: flex;
        flex-direction: column;
        & > {
            flex: 1;
        }
    }
</style>

