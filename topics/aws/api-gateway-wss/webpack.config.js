const { join } = require('path')
const { readdirSync, existsSync } = require('fs')

const entries = () => {
    const srcDir = './src'
    const entryFile = 'index.js'
    return readdirSync(srcDir)
        .filter(folderName => existsSync(join(srcDir, folderName, entryFile)))
        .reduce((agg, folderName) => {
            agg[folderName] = './' + join(srcDir, folderName, entryFile)

            return agg
        }, {})
}

module.exports = {
    mode: 'production',
    target: 'node',
    entry: entries(),
    output: {
        filename: '[name]/index.js',
        library: '[name]',
        libraryTarget: 'umd'
    },
}
