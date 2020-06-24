// Warning: reality will only ever be the closest 60/2*n. eg: 60fps, 30fps, 15fps, 7.5fps, etc.
// So eg: targetFps = 20, will actually be ~15fps.
export const targetFps = 70;

export const paddleWidth = 10;
export const paddleHeight = 50;
export const ballSpeed = 250;
export const ballSize = 7;
export const ballInitAngle = Math.PI / 3;

export const courtWidth = 500;
export const courtHeight = 500;

// Used to track keydown + keyup events, so the engine can check 
// the current key state, instead of reacting to events.
export var currentKeys = new Set();
