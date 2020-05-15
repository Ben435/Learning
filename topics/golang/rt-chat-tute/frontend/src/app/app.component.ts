import {Component, OnDestroy, OnInit} from '@angular/core';
import {EventType, SocketService} from "./socket.service";

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit, OnDestroy{
  public messages: Array<any>;
  public chatBox: string;

  public constructor(private readonly socket: SocketService) {
    this.messages = [];
    this.chatBox = "";
  }

  ngOnInit(): void {
    this.socket.connect();
    this.socket.getEventListener().subscribe(ev => {
      if (ev.type == EventType.OPEN) {
        this.messages.push("/Socket opened")
      } else if (ev.type == EventType.CLOSE) {
        this.messages.push("/Socket closed")
      } else if (ev.type == EventType.MESSAGE) {
        let data = ev.data.content;
        if (ev.data.sender) {
          data = ev.data.sender + ": " + data;
        }

        this.messages.push(data)
      } else {
        console.log("Received something weird", ev)
      }
    })
  }

  ngOnDestroy(): void {
    this.socket.close();
  }

  public send() {
    if(this.chatBox) {
      this.socket.send(this.chatBox);
      this.chatBox = "";
    }
  }

  public isSystemMessage(message: string) {
    return message.startsWith("/") ? `<strong>${message.substr(1)}</strong>` : message;
  }
}
