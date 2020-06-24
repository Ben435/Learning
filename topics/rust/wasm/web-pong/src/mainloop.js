import { currentKeys, courtWidth, courtHeight } from './globals';

export const stepFunc = (ctx, gameState, width, height) => stepTime => {
    gameState.tick(stepTime, Array.from(currentKeys));

    const rects = gameState.get_rects();
    const score = gameState.get_score();

    ctx.clearRect(0, 0, width, height);

    drawFps(ctx, stepTime);

    ctx.lineWidth = 1;
    drawCourt(ctx, width, height);
    drawScore(ctx, score[0], score[1], width)
    rects.forEach(rect => drawRect(ctx, width, height, rect));
}

const maxTail = 50;
const runningFps = new Array(maxTail);
let curIndex = 0;
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
    ctx.strokeRect(0, 0, width-1, height-1);
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
const drawRect = (ctx, width, height, rect) => {
    const width_multiplier = width / courtWidth;
    const height_multiplier = height / courtHeight;
    ctx.beginPath();
    ctx.fillRect(
        rect[0] * width_multiplier, 
        rect[1] * height_multiplier, 
        rect[2] * width_multiplier, 
        rect[3] * height_multiplier,
    );
    ctx.stroke();
}