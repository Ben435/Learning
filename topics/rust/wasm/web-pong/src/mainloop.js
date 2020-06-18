import { ballRadius, currentKeys, courtHeight, courtWidth, paddleWidth, paddleHeight } from './globals';

const maxTail = 50;
const runningFps = new Array(maxTail);
let curIndex = 0;
export const stepFunc = (ctx, gameState, width, height) => stepTime => {
    runningFps[curIndex] = stepTime;
    curIndex = (curIndex + 1) % maxTail
    const curAvgFps = runningFps.reduce((a, b) => a + b, 0) / maxTail;

    gameState.tick(stepTime, Array.from(currentKeys));

    ctx.clearRect(0, 0, width, height);

    ctx.font = '12px serif';
    ctx.fillText(`${(1000/curAvgFps).toFixed(2)}fps`, 10, 20);

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