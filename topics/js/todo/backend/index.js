const express = require('express');

const app = express();
const port = 3000;

app.use('/static', express.static('static'));

app.get('/api/hello', (_req, res) => res.send('world'));

app.listen(port, () => {
    console.log(`Listening on http://localhost:${port}`);
})
