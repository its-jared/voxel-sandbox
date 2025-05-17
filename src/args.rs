use std::env;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameArgs {
    pub render_distance: u32,
}

impl Default for GameArgs {
    fn default() -> Self {
        Self { 
            render_distance: 10, 
        }
    }
}

pub fn handle_args() -> GameArgs {
    let args: Vec<String> = env::args().collect();
    let mut game_args = GameArgs::default();

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "render_distance" => {
                if index + 1 < args.len() {
                    if let Ok(new_render_dist) = args[index + 1].parse::<u32>() {
                        game_args.render_distance = new_render_dist;
                    }
                    index += 1;
                }
            },
            _ => { index += 1; }
        }
    }

    game_args
}