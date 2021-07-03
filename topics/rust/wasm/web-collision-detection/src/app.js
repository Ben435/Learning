
const app = () => {
    const canvas = document.getElementById('drawspace');
    const ctx = canvas.getContext('2d');

    import('collision-detection')
        .then(wasm => {
            wasm.init()

            const manager = wasm.get_2d_manager()

            const circle_id = manager.add_circle(0, 0, 5);

            ctx.beginPath();
            ctx.arc(100, 100, 5, 0, 2 * Math.PI);
            ctx.stroke();
        });
}


export default app;
