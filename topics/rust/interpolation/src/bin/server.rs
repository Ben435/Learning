use std::net::UdpSocket;
use std::io::Result;
use interpolation::network::messages::{MessageType,ClientUpdate,ServerUpdate,CubeState};
use interpolation::network::constants::{SERVER_ADDR,MAX_MESSAGE_SIZE};
use bincode::{serialize,deserialize};
use cgmath::{vec3};
use std::collections::VecDeque;
use std::collections::HashMap;

pub fn main() -> Result<()> {
    let message_queue = VecDeque::<ClientUpdate>::new();

    let sock = UdpSocket::bind(SERVER_ADDR)?;

    let mut buf = [0; MAX_MESSAGE_SIZE];

    let mut world_state: Vec<CubeState> = Vec::new();
    let mut sender_to_cube = HashMap::<String, usize>::new();
    let mut next_cube_index = 0;

    let mut should_stop = false;
    while !should_stop {
        let res = sock.recv_from(&mut buf);
        match res {
            Ok((amount_read, sender)) => {
                if amount_read <= 0 {
                    // Read nothing
                } else {
                    let sender_key = format!("{}", sender);

                    let msg = deserialize::<ClientUpdate>(&buf).expect("Error deserializing");
                    println!("Server received {} bytes: {:?}", amount_read, msg);

                    let cube_index = match sender_to_cube.get(&sender_key) {
                        Some(id) => *id,
                        None => {
                            let ret_index = next_cube_index;
                            next_cube_index += 1;
                            sender_to_cube.insert(sender_key, ret_index);

                            ret_index
                        }
                    };

                    let new_cube_state = CubeState{
                        cube_id: cube_index,
                        position: msg.position,
                    };

                    if world_state.len() < cube_index+1 {
                        for i in world_state.len()..cube_index {
                            world_state.push(CubeState{ cube_id: i, position: vec3(0.0, 0.0, 0.0) });
                        }
                        world_state.push(new_cube_state);
                    } else {
                        world_state.push(new_cube_state);
                        world_state.swap_remove(cube_index);
                    }

                    println!("New server state: {:?}", world_state);

                    let new_msg = ServerUpdate{
                        mtype: MessageType::POSITION,
                        positions: world_state.clone(),
                    };

                    let new_msg = serialize(&new_msg).expect("Error serializing");
                
                    sock.send_to(new_msg.as_slice(), &sender)?;
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
