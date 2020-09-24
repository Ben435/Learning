FROM node:12

RUN mkdir app
ADD "./package.json" "./package.json"
ADD "./package-lock.json" "./package-lock.json"
ADD "./static" "./static"

VOLUME [ "/__sapper__/build" ]

RUN npm ci

RUN ls
RUN ls

ENTRYPOINT ["node", "/__sapper__/build"]
