const commonWebpack = require('./webpack.common');
const merge = require('webpack-merge');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

module.exports = merge({
    mode: "production",
    plugins: [
        new CleanWebpackPlugin(),
    ]
}, commonWebpack);