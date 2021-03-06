
import { targetFps, courtWidth, courtHeight, currentKeys, paddleWidth, paddleHeight, ballSize, ballSpeed, ballInitAngle } from './globals';
import { stepFunc } from './mainloop';

export const init = async () => {
    const gameState = import('wasm-pong/wasm_pong')
        .then(wasm_pong => {
            wasm_pong.init();
            return wasm_pong.new_game(
                courtWidth, courtHeight, 
                paddleWidth, paddleHeight, 
                ballSize, ballSpeed, ballInitAngle
            );
        });

    const gameWrapper = document.getElementById("game-wrapper");
    let { width, height } = gameWrapper.getBoundingClientRect();
    // We want a square field (else rectangles get weird)
    width = height = Math.min(width, height)
    const cnvs = document.getElementById("gamespace");
    cnvs.width = width;
    cnvs.height = height;

    const ctx = cnvs.getContext('2d');

    cnvs.addEventListener("keydown", ev => {
        currentKeys.add(ev.keyCode);
    })

    cnvs.addEventListener("keyup", ev => {
        currentKeys.delete(ev.keyCode);
    })

    renderAtFps(targetFps, stepFunc(ctx, await gameState, width, height));
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
