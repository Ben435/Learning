import React, { useState, useEffect, useRef } from 'react';
import { Universe, Cell } from "wasm-game-of-life";
import {memory} from "wasm-game-of-life/wasm_game_of_life_bg";
import "./app.css";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const getIndex = (row, column, width) => {
    return row * width + column;
};

const drawGrid = (ctx, width, height) => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;
    
    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
    
    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }
    
    ctx.stroke();
};
  
const drawCells = (ctx, width, height, universe) => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  
    ctx.beginPath();
  
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col, width);

            ctx.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : ALIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
  
    ctx.stroke();
};

const Excalibur = () => {

    const [universe, setUniverse] = useState(() => Universe.new());

    const requestRef = useRef();
    const prevRequestRef = useRef();
    const canvasRef = useRef();

    const width = universe.width();
    const height = universe.height();

    const animate = ctx => {
        const internalAnimate = time => {
            if (!prevRequestRef.current) {
                prevRequestRef.current = time;
            }

            if ((time - prevRequestRef.current) > (1000/5)) {
                universe.tick();
                drawGrid(ctx, width, height);
                drawCells(ctx, width, height, universe);

                prevRequestRef.current = time;
            }
            requestRef.current = requestAnimationFrame(internalAnimate);
        }

        return internalAnimate;
    }

    useEffect(() => {
        if (canvasRef.current) {
            const ctx = canvasRef.current.getContext('2d');

            requestRef.current = requestAnimationFrame(animate(ctx));
        }

        return () => cancelAnimationFrame(requestRef.current);
    }, [universe, canvasRef])

    return (
        <>
            <canvas 
                height={(CELL_SIZE + 1) * height + 1} 
                width={(CELL_SIZE + 1) * width + 1}
                ref={canvasRef}>
            </canvas>
            <button onClick={() => setUniverse(Universe.new())}>Reset</button>
        </>
    )
}

export default Excalibur;