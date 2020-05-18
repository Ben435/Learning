const ejs = require('ejs');
const {Base64} = require('js-base64');
const util = require('util');
const fs = require('fs');

const readFile = util.promisify(fs.readFile);
const writeFile = util.promisify(fs.writeFile);

function processUserData(stackName) {
    return ejs.renderFile('./startup-script.sh', {
        stackName
    })
    .then(contents => Base64.encode(contents))
    .then(b64EncodedContents => ejs.renderFile('./template.yml', {
        userDataScript: b64EncodedContents
    }))
    .then(renderedTemplateContents => writeFile("./out/template.yml", renderedTemplateContents))
    .catch(e => {
        console.error("Received Error:", e);
        throw e;
    });
}

function processEcsConfig(stackName) {
    return ejs.renderFile('./ecs.config', {
        stackName
    }).then(renderedConfigContents => writeFile("./out/ecs.config", renderedConfigContents))
    .catch(e => {
        console.error("Received Error:", e);
        throw e;
    });
}

function main(stackName) {
    return Promise.all([
        processUserData(stackName),
        processEcsConfig(stackName)
    ]);
}

let stackName;
if (process.argv.length >= 3) {
    stackName = process.argv[2];
    console.info(`Stack name: '${stackName}'`)
} else {
    stackName = "experiment-ecs";
    console.warn(`Defaulting stack name to '${stackName}'`)
}

main(stackName).then(() => console.log("Done!"));
