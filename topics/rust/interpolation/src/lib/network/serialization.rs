use super::messages::{MessageType,ConnectMessage,ClientUpdate};
use serde::{Serialize,Serializer,Deserialize,Deserializer,de};
use serde_value::Value;

#[derive(Serialize,Deserialize)]
struct MessageWrapper {
    mtype: MessageType,
    version: u8,
    data: Value,
}

#[derive(Debug)]
enum Message {
    Connect(ConnectMessage),
    Position(ClientUpdate),
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
    where
        D: Deserializer<'de> 
    {
        let helper = MessageWrapper::deserialize(deserializer)?;

        match (helper.mtype, helper.version) {
            (MessageType::CONNECT, 1) => ConnectMessage::deserialize(helper.data)
                .map(Message::Connect)
                .map_err(de::Error::custom),
            (MessageType::POSITION, 1) => ClientUpdate::deserialize(helper.data)
                .map(Message::Position)
                .map_err(de::Error::custom),
            (mtype, version) => Err(de::Error::custom(format!("Unrecognized message type + version: {:?}:{}", mtype, version))),
        }
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self { // TODO: Confused
            Message::Connect(cmsg) => MessageWrapper{ mtype: MessageType::CONNECT, data: serde_value::Value::Newtype(cmsg) }.serialize(serializer),
            Message::Position(msg) => {},
        };

        serializer.serialize_bool(false)
    }
}
