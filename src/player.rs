use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_voxel_world::prelude::*;

use crate::{level::{self, MainLevel}, voxels};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct PlayerCursor {
    pub voxel_pos: IVec3,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(FlyCameraPlugin)
            .add_systems(Startup, (
                setup,
                cursor_grab,
            ))
            .add_systems(Update, (
                handle_cursor_grab,
                update_cursor_cube,
                mouse_button_input,
            ));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Transform::from_xyz(0.0, 60.0, 0.0),
        Camera3d::default(),
        FlyCamera::default(),
        VoxelWorldCamera::<level::MainLevel>::default(),
        Player
    ));

    commands.spawn((
        Transform::from_xyz(0.0, -10.0, 0.0),
        MeshMaterial3d(materials.add(Color::srgba_u8(124, 144, 255, 128))),
        Mesh3d(meshes.add(Mesh::from(Cuboid {
            half_size: Vec3::splat(0.5),
        }))),
        PlayerCursor {
            voxel_pos: IVec3::new(0, -10, 0),
        },
    ));

    let cascade_shadow_config = CascadeShadowConfigBuilder { ..default() }.build();
    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadows_enabled: false,
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
    let window_size = primary_window.physical_size();
    let center_x = window_size.x / 2;
    let center_y = window_size.y / 2;

    primary_window.set_cursor_position(Some(Vec2::new(center_x as f32, center_y as f32)));
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;
}

fn handle_cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut primary_window = q_windows.single_mut().unwrap();
    let window_size = primary_window.physical_size();
    let center_x = window_size.x / 2;
    let center_y = window_size.y / 2;

    if keys.just_pressed(KeyCode::Escape) {
        primary_window.cursor_options.grab_mode = CursorGrabMode::None;
        primary_window.cursor_options.visible = true;
    }

    if keys.just_pressed(KeyCode::Tab) {
        primary_window.set_cursor_position(Some(Vec2::new(center_x as f32, center_y as f32)));
        primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor_options.visible = false;
    }
}

fn update_cursor_cube(
    voxel_world_raycast: VoxelWorld<MainLevel>,
    camera_info: Query<(&Camera, &GlobalTransform), With<VoxelWorldCamera<MainLevel>>>,
    mut cursor_evr: EventReader<CursorMoved>,
    mut cursor_cube: Query<(&mut Transform, &mut PlayerCursor)>,
) {
    for ev in cursor_evr.read() {
        // Get a ray from the cursor position into the world
        let (camera, cam_gtf) = camera_info.single().unwrap();
        let Ok(ray) = camera.viewport_to_world(cam_gtf, ev.position) else {
            return;
        };

        if let Some(result) = voxel_world_raycast.raycast(ray, &|(_pos, _vox)| true) {
            let (mut transform, mut cursor_cube) = cursor_cube.single_mut().unwrap();
            // Move the cursor cube to the position of the voxel we hit
            // Camera is by construction not in a solid voxel, so result.normal must be Some(...)
            let voxel_pos = result.position + result.normal.unwrap();
            transform.translation = voxel_pos + Vec3::new(0.5, 0.5, 0.5);
            cursor_cube.voxel_pos = voxel_pos.as_ivec3();
        }
    }
}

fn mouse_button_input(
    buttons: Res<ButtonInput<MouseButton>>,
    mut voxel_world: VoxelWorld<MainLevel>,
    cursor_cube: Query<&PlayerCursor>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let vox = cursor_cube.single().unwrap();
        voxel_world.set_voxel(vox.voxel_pos, WorldVoxel::Solid(voxels::STONE));
    }
}