use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(modular_mc::DefaultModularMCPlugins)
        .run();
}
