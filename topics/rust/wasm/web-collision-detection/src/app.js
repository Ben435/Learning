
const app = () => {
    import('collision-detection')
        .then(wasm => {
            wasm.greet('asdf')
        });
}

export default app;
