FROM golang:latest

ADD app /app/
WORKDIR /app

RUN ["make"]

EXPOSE 8080
CMD ["out/gomovie"]
