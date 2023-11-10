use serde::{Deserialize, Serialize};
use vek::Vec2;

use crate::{chunk::Chunk, uid::Uid};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientPacket {
    Connect,
    Disconnect,
    Ping(PingPacket),
    ChunkRequest(Vec2<i32>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerPacket {
    ClientSync { uid: Uid },
    Ping(PingPacket),
    ChunkUpdate { pos: Vec2<i32>, data: Chunk },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PingPacket {
    Ping,
    Pong,
}
