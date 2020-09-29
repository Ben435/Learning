use super::messages::{ConnectMessage,ClientUpdate,ServerUpdate};
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    Connect(ConnectMessage),
    ClientUpdate(ClientUpdate),
    ServerUpdate(ServerUpdate),
}

impl Message {
    pub fn connect(msg: ConnectMessage) -> Message {
        Message::Connect(msg)
    }

    pub fn client_update(msg: ClientUpdate) -> Message {
        Message::ClientUpdate(msg)
    }

    pub fn server_update(msg: ServerUpdate) -> Message {
        Message::ServerUpdate(msg)
    }
}
