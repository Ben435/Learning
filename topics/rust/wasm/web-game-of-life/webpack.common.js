const path = require('path');
const webpack = require('webpack');
const FaviconsWebpackPlugin = require('favicons-webpack-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin');

const handler = (percentage, message, ...args) => {
    // e.g. Output each progress message directly to the console:
    console.info(`${(percentage * 100).toFixed(2)}%: ${message}`, ...args);
  };

module.exports = {
    entry: path.resolve(__dirname, "src", "index.js"),
    output: {
        filename: "main.[hash].js",
        path: path.resolve(__dirname, 'out')
    },
    plugins: [
        new webpack.ProgressPlugin(handler),
        new HtmlWebpackPlugin({
            title: 'WasmGameOfLife',
            inject: true,
            template: path.resolve(__dirname, 'static', 'index.html'),
        }),
        new FaviconsWebpackPlugin({
            logo: path.resolve(__dirname, 'static', 'logo.png'),
            inject: true
        })
    ],
    module: {
      rules: [
        {
          test: /\.(js|jsx)$/,
          exclude: /node_modules/,
          use: {
            loader: "babel-loader"
          }
        },
        {
          test: /\.(css)$/,
          use: ['style-loader', 'css-loader'],
        }
      ]
    }
}
