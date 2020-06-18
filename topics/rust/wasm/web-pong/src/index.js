
const paddleWidth = 10;
const paddleHeight = 50;
const ballRadius = 5;

const courtWidth = 500;
const courtHeight = 500;

let currentKeyDowns = new Set()

const main = async () => {
    const cnvs = document.getElementById("gamespace");
    cnvs.width = courtWidth;
    cnvs.height = courtHeight;

    cnvs.addEventListener("keydown", ev => {
        if ([38, 40].find(code => ev.keyCode === code)) {
            currentKeyDowns.add(ev.keyCode)
        }
    })

    cnvs.addEventListener("keyup", ev => {
        if ([38, 40].find(code => ev.keyCode === code)) {
            currentKeyDowns.delete(ev.keyCode)
        }
    })

    const ctx = cnvs.getContext('2d');

    const wasm_pong = await import('wasm-pong/wasm_pong');

    wasm_pong.init();

    const gameState = wasm_pong.GameState.new(courtWidth, courtHeight, paddleWidth, paddleHeight);

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
    gameState.tick(stepTime, currentKeyDowns.values());

    ctx.clearRect(0, 0, width, height);
    ctx.lineWidth = 5;

    drawCourt(ctx);

    drawBall(ctx, gameState);
    drawPlayerPaddle(ctx, gameState);
    drawAiPaddle(ctx, gameState);
}

const drawCourt = (ctx) => {    
    ctx.strokeRect(0, 0, courtWidth, courtHeight);
    ctx.stroke();
}

const drawBall = (ctx, gameState) => {
    const ballPos = gameState.get_ball_position();

    ctx.beginPath();
    ctx.arc(ballPos.get_x(), ballPos.get_y(), ballRadius, 0, 2 * Math.PI);
    ctx.stroke();
}

const drawPlayerPaddle = (ctx, gameState) => {
    const playerPaddlePos = gameState.get_player_paddle_position();
    ctx.beginPath();
    ctx.fillRect(playerPaddlePos.get_x(), playerPaddlePos.get_y(), paddleWidth, paddleHeight);
    ctx.stroke();
}

const drawAiPaddle = (ctx, gameState) => {
    const aiPaddlePos = gameState.get_ai_paddle_position();

    ctx.beginPath();
    ctx.fillRect(aiPaddlePos.get_x(), aiPaddlePos.get_y(), paddleWidth, paddleHeight);
    ctx.stroke();
}

main();
