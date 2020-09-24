import { writable } from 'svelte/store';

export const LoadingStates = {
    NEVER_LOADED: 'NEVER_LOADED',
    IS_LOADING: 'IS_LOADING',
    LOADED: 'LOADED',
}

export const todos = writable({
    loadingState: LoadingStates.NEVER_LOADED,
    items: [],
});
