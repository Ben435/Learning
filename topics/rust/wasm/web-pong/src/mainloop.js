import { currentKeys } from './globals';

const maxTail = 50;
const runningFps = new Array(maxTail);
let curIndex = 0;
export const stepFunc = (ctx, gameState, width, height) => stepTime => {
    gameState.tick(stepTime, Array.from(currentKeys));

    const rects = gameState.get_rects();
    const score = gameState.get_score();

    ctx.clearRect(0, 0, width, height);

    drawFps(ctx, stepTime);

    ctx.lineWidth = 1;
    drawCourt(ctx, width, height);
    drawScore(ctx, score[0], score[1], width)
    rects.forEach(rect => drawRect(ctx, rect));
}

const drawFps = (ctx, stepTime) => {
    runningFps[curIndex] = stepTime;
    curIndex = (curIndex + 1) % maxTail
    const curAvgFps = runningFps.reduce((a, b) => a + b, 0) / maxTail;

    ctx.beginPath();
    ctx.font = '12px serif';
    ctx.fillText(`${(1000/curAvgFps).toFixed(2)}fps`, 10, 20);
    ctx.stroke();
}

const drawCourt = (ctx, width, height) => {    
    ctx.strokeRect(0, 0, width, height);
    ctx.stroke();
}

const drawScore = (ctx, player_score, ai_score, width) => {
    ctx.beginPath();
    ctx.font = '12px serif';
    // These offsets are arbitrary, just going by eye.
    // Puts the text in the centre top, and puts a box around it.
    ctx.fillText(`${player_score} : ${ai_score}`, width/2 - 12, 20);
    ctx.rect(width/2 - 20, 0, 40, 30)
    ctx.stroke();
}

// Rect in form [x, y, width, height]
const drawRect = (ctx, rect) => {
    ctx.beginPath();
    ctx.fillRect(rect[0], rect[1], rect[2], rect[3]);
    ctx.stroke();
}