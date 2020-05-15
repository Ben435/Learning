const commonWebpack = require('./webpack.common');
const merge = require('webpack-merge');

module.exports = merge({
    mode: "development",
    devServer: {
        contentBase: './out',
        index: 'index.html'
    },
}, commonWebpack);