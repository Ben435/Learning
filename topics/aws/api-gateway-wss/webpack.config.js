module.exports = {
    mode: 'production',
    target: 'node',
    entry: {
        default: './src/default/index.js',
        connect: './src/connect/index.js',
        disconnect: './src/disconnect/index.js'
    },
    output: {
        filename: '[name]/index.js',
        library: '[name]',
        libraryTarget: 'umd'
    },
}
