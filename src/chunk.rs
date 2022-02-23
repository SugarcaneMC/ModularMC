//! A module for storing block and chunk data.

use bevy::prelude::*;
use dashmap::DashMap;

/// The position of a block within it's chunk
#[derive(Debug, Component)]
pub struct BlockPosInChunk(pub [u8; 3]);

/// The Minecraft Block ID of a block.
#[derive(Debug, Component)]
pub struct BlockID(pub u32);

/// A chunk representation, containing references to each block as a Bevy Entity ID.
#[derive(Debug, Clone, Hash)]
pub struct Chunk {
    /// The X and Z coordinates of the chunk.
    pub xz: [i32; 2],
    /// A Vector of Bevy Entity handles representing the all the blocks in the chunk.
    pub blocks: Vec<Entity>,
}

/// A resource mapping a chunk X and Z coordinate to it's chunk struct.
#[derive(Debug, Default)]
pub struct Chunks(pub DashMap<[i32; 2], Chunk>);

/// A plugin for representing blocks and chunks.
pub struct BlockAndChunkPlugin;

impl Plugin for BlockAndChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Chunks>();
    }
}
