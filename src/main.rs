use bevy::prelude::*;

pub mod args;
pub mod game;
pub mod level;
pub mod player;
pub mod voxels;

fn main() {
    let game_args = args::handle_args();
    
    App::new()
        .insert_resource(game_args)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        title: "Voxel Sandbox".to_string(),
                        resizable: false, 
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            
            game::GamePlugin,
        ))
        .run();
}