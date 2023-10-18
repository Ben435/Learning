FROM golang:alpine

ADD . /work

WORKDIR /work

RUN go build -o out/api cmd/api/main.go

ENTRYPOINT ./out/api
