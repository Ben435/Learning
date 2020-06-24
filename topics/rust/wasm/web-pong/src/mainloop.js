import { currentKeys, courtHeight, courtWidth } from './globals';

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

    ctx.lineWidth = 1;
    const rects = gameState.get_rects();
    drawCourt(ctx);
    rects.forEach(rect => drawRect(ctx, rect));
}

const drawCourt = (ctx) => {    
    ctx.strokeRect(0, 0, courtWidth, courtHeight);
    ctx.stroke();
}

// Rect in form [x, y, width, height]
const drawRect = (ctx, rect) => {
    ctx.beginPath();
    ctx.fillRect(rect[0], rect[1], rect[2], rect[3]);
    ctx.stroke();
}