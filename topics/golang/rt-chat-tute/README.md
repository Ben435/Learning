# rt-chat-tute

Learning a bit of golang + websockets API.
Basic chat app, with frontend + backend.

## Backend

Golang, takes in connections (websockets), and connects to the room. Assigns each connection a UUID to differentiate.
Each message sent to the room is broadcast to everyone in the room. All just kept in memory.

## Frontend

Angular8ish, connects to the room, sends and receives messages from websocket events.
Just prints the messages out in receive order.
