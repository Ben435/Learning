import * as React from "react";
import './demo.scss';

export default {
    title: 'Common',
    component: Demo,
};

const range = (how_many) => [...Array(how_many+1).keys()].slice(1)
const colorTypes = [
    'primary',
    'accent',
    'grey',
]

export const Demo = () => {
    return (
        <div className="demo">
            <div className="reference">
                <div>
                    <button className="baseline">Baseline</button>
                </div>
                <div>
                    <div className="alert">
                        <h4>Darkest primary reference</h4>
                        <p>Lightest primary for background. More text here too.</p>
                    </div>
                </div>
            </div>

            <div className="pallette">
                {colorTypes.map((colorName) => (
                    <div className={`color-range ${colorName}`}>
                        {range(9).map((i) => (
                        <div className="sample">
                            <div id={`${colorName}-${i}`} className={`color ${colorName}-${i}`}></div>
                            <label for={`${colorName}-${i}`}>{i}</label>
                        </div>
                        ))}
                    </div>
                ))}
            </div>
        </div>
    )
}