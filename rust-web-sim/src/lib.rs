use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;
use bevy::prelude::*;

pub mod demo_scene;
pub use demo_scene::run_models_demo_app;

#[wasm_bindgen]
pub fn run_demo_scene2() {
    demo_scene::run_models_demo_app();
}

#[wasm_bindgen]
pub fn run_bevy_app() {
    // panic::set_hook(Box::new(console_error_panic_hook::hook));

    demo_scene::run_models_demo_app();
    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_startup_system(setup)
    //     .add_system(rotate_mesh_system)
    //     .run();
}

#[derive(Component)]
struct RotateMe;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    }, RotateMe));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn rotate_mesh_system(time: Res<Time>, mut query: Query<(&mut Transform), With<RotateMe>>) {
    for mut transform in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(time.delta_seconds() * 0.5));
    }
}