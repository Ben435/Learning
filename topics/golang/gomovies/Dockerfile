FROM golang:latest

ADD app /app/
WORKDIR /app

RUN ["make"]

CMD ["out/gomovie"]
