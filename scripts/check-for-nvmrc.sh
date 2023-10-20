#!/bin/bash

nodeVersion="v18"
allPackageJsons=$(find . -name "package.json" -not -path "*/node_modules/*")

for packageJson in $allPackageJsons
do
    directory=$(dirname "$packageJson")
    nvmrcPath="$directory/.nvmrc"
    if [ -f $nvmrcPath ]; then
        echo "$directory .nvmrc exists with version: $(cat $nvmrcPath)"
    else
        echo "$directory no nvmrc! Creating one with $nodeVersion..."
        echo $nodeVersion > $nvmrcPath
    fi
done
