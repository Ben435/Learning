use std::net::UdpSocket;
use std::io::Result;
use interpolation::messages::{Message,MessageType};
use interpolation::constants::{SERVER_ADDR,MAX_MESSAGE_SIZE};
use bincode::{serialize,deserialize};

pub fn main() -> Result<()> {
    let sock = UdpSocket::bind("127.0.0.1:0")?;

    sock.connect(SERVER_ADDR)?;

    // Send msg
    let mut buf: [u8; 32] = [0; 32];
    for i in 0..10 {
        buf[i] = i as u8;
    }
    let msg = Message{
        typ: MessageType::METADATA,
        content: buf,
    };

    let serial_msg = serialize(&msg).expect("Failed to serialize");

    sock.send(serial_msg.as_slice())?;

    // Receive response
    let mut recv_buf: [u8; MAX_MESSAGE_SIZE] = [0; MAX_MESSAGE_SIZE];
    let (amount_read, _sender) = sock.recv_from(&mut recv_buf)?;

    let recv_msg = deserialize::<Message>(&recv_buf);
    
    println!("Received {} bytes: {:?}", amount_read, recv_msg);

    Ok(())
}
