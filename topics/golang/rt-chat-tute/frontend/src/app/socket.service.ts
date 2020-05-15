import {EventEmitter, Injectable} from '@angular/core';

export enum EventType {
  OPEN = "open",
  CLOSE = "close",
  MESSAGE = "message"
}

@Injectable({
  providedIn: 'root'
})
export class SocketService {

  private socket: WebSocket;
  private listener: EventEmitter<any> = new EventEmitter<{type: EventType, data: Event | CloseEvent | any}>();

  constructor() {}

  public connect() {
    this.socket = new WebSocket("ws://localhost:8080/api/ws");
    this.socket.onopen = event => this.listener.emit({"type": EventType.OPEN, "data": event});
    this.socket.onclose = event => this.listener.emit({"type": EventType.CLOSE, "data": event});
    this.socket.onmessage = event => this.listener.emit({"type": EventType.MESSAGE, "data": JSON.parse(event.data)});
  }

  public send(data: string): void {
    if (!this.socket) {
      this.connect();
    }
    this.socket.send(data);
  }

  public close(): void {
    if (this.socket) {
      this.socket.close();
    }
  }

  public getEventListener(): EventEmitter<{type: EventType, data: Event | CloseEvent | any}> {
    return this.listener;
  }
}
