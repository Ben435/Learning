import React, { useEffect, useState } from 'react'
import rand, { RandomSeed } from 'random-seed'
import './GamePage.css'

const boardWidth = 10
const boardHeight = 15

interface CellState {
    type: string
    marked: boolean
}

const initBoard = (rng: RandomSeed): CellState[][] => {
    const arr: number[][] = new Array(boardHeight).fill(new Array(boardWidth).fill(0))

    const jsxArr = arr.map((row) => row.map((_) => {
        const types = ['red', 'green', 'blue', 'purple', 'white']
        const num = rng.intBetween(0, types.length-1)
        return {
            type: types[num],
            marked: false,
        }
    }))

    return jsxArr
}

const seed = 'abc123'

export const GamePage: React.FC = () => {
    const rng = rand.create(seed)
    const [ gameState, setGameState ] = useState(initBoard(rng))

    const identifyMatches = () => {
        const toMark: {[rowNum: number]: number[]} = {}
        const markCell = (rowNum: number, ...colNums: number[]): void => {
            let existing = toMark[rowNum]
            if (!existing) {
                existing = []
            }
            toMark[rowNum] = existing.concat(colNums)
        }

        gameState.forEach((row, rowNum) => rowNum === 0 || rowNum === boardHeight-1 ? row : row.forEach((cell, colNum) => {
            if (colNum === 0 || colNum === boardWidth-1) {
                return // skip first and last col
            }
            // vertical
            if (gameState[rowNum-1][colNum].type === cell.type && gameState[rowNum+1][colNum].type === cell.type) {
                // vertical mark
                markCell(rowNum-1, colNum)
                markCell(rowNum, colNum)
                markCell(rowNum+1, colNum)
            } else if (gameState[rowNum][colNum-1].type === cell.type && gameState[rowNum][colNum+1].type === cell.type) {
                // horizontal mark
                markCell(rowNum, colNum-1, colNum, colNum+1)
            }
        }))
        const newGameState = gameState.map((row, rowNum) => {
            if (!toMark[rowNum]?.length) {
                return row
            }

            const marks = toMark[rowNum]
            return row.map((cell, colNum) => {
                if (marks.includes(colNum)) {
                    console.log(`Marking: (${colNum}, ${rowNum})`)
                    return { ...cell, marked: true }
                }
                return cell
            })
        })

        setGameState(newGameState)
    }

    return (
        <div>
            <section className='board'>
                {gameState.map((row, rowNum) => <div className='row' key={rowNum}>{row.map((cell, colNum) => <Cell key={rowNum * 100 + colNum} cellState={cell} rowNum={rowNum} colNum={colNum}/>)}</div>)}
            </section>
            <button onClick={() => identifyMatches()}>Identify Matches</button>
        </div>
    )
}

interface CellProps {
    cellState: CellState
    rowNum: number
    colNum: number
}

const Cell: React.FC<CellProps> = ({ cellState, rowNum, colNum }) => {
    return (
        <div className={`cell cell--${cellState.type} ${cellState.marked && 'cell--marked'}`}>{colNum},{rowNum}</div>
    )
}
