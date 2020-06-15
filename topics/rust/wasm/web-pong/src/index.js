
const main = async () => {
    const cnvs = document.getElementById("gamespace")

    cnvs.getContext('2d');

    const wasm_pong = await import('wasm-pong/wasm_pong')

    const gameState = wasm_pong.GameState.new(10, 10, 1, 2)
    const ballPos = gameState.get_ball_position()
    const aiPaddlePos = gameState.get_ai_paddle_position()
    const playerPaddlePos = gameState.get_player_paddle_position()

    console.log(ballPos.get_x(), ballPos.get_y());
    console.log(aiPaddlePos.get_x(), aiPaddlePos.get_y());
    console.log(playerPaddlePos.get_x(), playerPaddlePos.get_y());

    window.requestAnimationFrame(stepFunc(gameState))
};

const stepFunc = gameState => stepTime => {
    console.log('New frame!');

    gameState.tick(stepTime);

    const newBallPos = gameState.get_ball_position()
    console.log(newBallPos.get_x(), newBallPos.get_y());

    window.requestAnimationFrame(stepFunc(gameState))
}

main();
