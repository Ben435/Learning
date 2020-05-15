import React from 'react';
import ReactDOM from 'react-dom';

import("./app.jsx").then(mod => {
    const Excalibur = mod.default;
    ReactDOM.render(
        <Excalibur />,
        document.getElementById('root')
    );
}).catch(e => console.error("Failed to load react:", e));
