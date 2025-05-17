use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_voxel_world::prelude::VoxelWorldCamera;

use crate::level;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FlyCameraPlugin)
            .add_systems(Startup, (
                setup,
                cursor_grab,
            ))
            .add_systems(Update, handle_cursor_grab);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Transform::from_xyz(0.0, 60.0, 0.0),
        Camera3d::default(),
        FlyCamera::default(),
        VoxelWorldCamera::<level::MainLevel>::default(),
        Player
    ));

    let cascade_shadow_config = CascadeShadowConfigBuilder { ..default() }.build();
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.1, 0.15), Vec3::Y),
        cascade_shadow_config,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.98, 0.95, 0.82),
        brightness: 100.0,
        affects_lightmapped_meshes: true,
    });
}

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut().unwrap();

    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;
}

fn handle_cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut primary_window = q_windows.single_mut().unwrap();

    if keys.just_pressed(KeyCode::Escape) {
        primary_window.cursor_options.grab_mode = CursorGrabMode::None;
        primary_window.cursor_options.visible = true;
    }

    if keys.just_pressed(KeyCode::Tab) {
        primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor_options.visible = false;
    }
}