use crate::chunk::*;
use bevy::prelude::*;

/// A plugin for integration/unit testing.
pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(increment_blocks_system);
    }
}

const DEFAULT_CHUNK_POS: [i32; 2] = [0, 0];

pub fn increment_blocks_system(
    mut commands: Commands,
    chunks: Res<Chunks>,
    mut query: Query<(&mut BlockID, &BlockPosInChunk)>,
) {
    if let Some(chunk) = chunks.0.get(&DEFAULT_CHUNK_POS) {
        todo!();
    } else {
        // Create the 0,0 chunk store since it doesn't exist.
        todo!();
    }
}
