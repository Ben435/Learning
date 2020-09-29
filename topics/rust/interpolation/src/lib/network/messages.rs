use serde::{Serialize,Deserialize};
use cgmath::Vector3;

#[derive(Debug,Serialize,Deserialize)]
pub struct ConnectMessage {}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct CubeState {
    pub cube_id: usize,
    pub position: Vector3<f32>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ClientUpdate {
    pub position: Vector3<f32>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ServerUpdate {
    pub positions: Vec<CubeState>,
}
