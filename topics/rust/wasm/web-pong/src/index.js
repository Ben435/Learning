
const main = async () => {
    const cnvs = document.getElementById("gamespace");
    cnvs.width = cnvs.height = 500;
    // cnvs.addEventListener('keydown')

    const ctx = cnvs.getContext('2d');

    const wasm_pong = await import('wasm-pong/wasm_pong');

    const gameState = wasm_pong.GameState.new(500, 500, 1, 2);
    const ballPos = gameState.get_ball_position();
    const aiPaddlePos = gameState.get_ai_paddle_position();
    const playerPaddlePos = gameState.get_player_paddle_position();

    renderAtFps(30, stepFunc(ctx, gameState, cnvs.width, cnvs.height));
};

const renderAtFps = (fps, cb) => {
    // First render
    cb(0);

    let prevFrameTime = 0;
    let timeBetweenFrames = 1000 / fps;

    const animate = newTime => {
        const timeElapsedBetweenFrames = newTime - prevFrameTime;
        if (timeElapsedBetweenFrames > timeBetweenFrames) {
            cb(timeElapsedBetweenFrames);

            prevFrameTime = newTime;
        }
        window.requestAnimationFrame(animate);
    }
    window.requestAnimationFrame(animate);
}

const stepFunc = (ctx, gameState, width, height) => stepTime => {
    gameState.tick(stepTime);

    ctx.clearRect(0, 0, width, height);

    drawBall(ctx, gameState);
}

const drawBall = (ctx, gameState) => {
    const newBallPos = gameState.get_ball_position();

    ctx.beginPath();
    ctx.arc(newBallPos.get_x(), newBallPos.get_y(), 5, 0, 2 * Math.PI)
    ctx.stroke();
}

main();
