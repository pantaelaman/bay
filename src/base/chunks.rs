use super::commands::Command;
use log::info;

#[derive(Debug)]
pub enum ChunkType {
    TARGET
}

#[derive(Debug)]
pub struct Chunk {
    chunktype: ChunkType,
    name: String,
    commands: Vec<Command>
}

impl Chunk {
    pub fn new(chunktype: ChunkType, name: String, commands: Vec<Command>) -> Self {
        info!(target: "parser::chunks", "Creating new chunk `{}` of type {:?}", name, chunktype);
        Chunk {
            chunktype,
            name,
            commands,
        }
    }
}

