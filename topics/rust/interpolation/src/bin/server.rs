use std::net::UdpSocket;
use std::io::Result;
use interpolation::network::messages::{MessageType,Message};
use interpolation::network::constants::{SERVER_ADDR,MAX_MESSAGE_SIZE};
use bincode::{serialize,deserialize};

pub fn main() -> Result<()> {
    let sock = UdpSocket::bind(SERVER_ADDR)?;

    sock.set_nonblocking(true)?;

    let mut buf = [0; MAX_MESSAGE_SIZE];

    let mut should_stop = false;
    while !should_stop {
        let res = sock.recv_from(&mut buf);
        match res {
            Ok((amount_read, sender)) => {
                if amount_read <= 0 {
                    // Read nothing
                } else {                
                    let mut msg = deserialize::<Message>(&buf).expect("Error deserializing");

                    println!("Server received {} bytes: {:?}", amount_read, msg);

                    msg.content.reverse();

                    let new_msg = serialize(&msg).expect("Error serializing");
                
                    sock.send_to(new_msg.as_slice(), &sender)?;
                    should_stop = true;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // We could probably handle this, if we were clever.
                // We are not.
                println!("WouldBlock error: {}", e);
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
            Err(e) => {
                println!("Unknown error: {}", e);
                should_stop = true;
            }
        }
    }

    return Ok(());
}
