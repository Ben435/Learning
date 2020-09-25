FROM node:12

RUN mkdir app
ADD "./package.json" "./package.json"
ADD "./package-lock.json" "./package-lock.json"
ADD "./static" "./static"

RUN npm ci

VOLUME [ "/__sapper__/build" ]

ENTRYPOINT ["node", "/__sapper__/build"]
