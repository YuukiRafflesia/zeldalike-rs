mod player;
mod setup;
mod data;

use bevy::prelude::*;
use player::PlayerPlugin;
use setup::SetupPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1180.0,
            height: 820.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
