FROM golang:latest

WORKDIR backend

# Go Mod dependencies
COPY backend/go.mod backend/go.sum ./
RUN go mod download rt-chat-tute/backend
COPY backend .

# Build
RUN go build -o app src/main/main.go

# Get static stuff
COPY out/static static

EXPOSE 8080
ENTRYPOINT ["./app"]
