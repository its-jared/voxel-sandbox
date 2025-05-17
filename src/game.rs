use bevy::prelude::*;
use bevy_voxel_world::prelude::*;

use crate::{level::MainLevel, player::PlayerPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                PlayerPlugin,
                VoxelWorldPlugin::with_config(MainLevel)
            ));
    }
}