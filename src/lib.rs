#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod chunk;
pub(crate) mod test;

use bevy::{app::PluginGroupBuilder, prelude::*};

/// The default Bevy plugins that should be added to any ModularMC server instance.
pub struct DefaultModularMCPlugins;

impl PluginGroup for DefaultModularMCPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(chunk::BlockAndChunkPlugin);
    }
}
