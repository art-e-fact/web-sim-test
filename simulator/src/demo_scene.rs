use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use kesko::core::{interaction::groups::GroupDynamic, event::SimulatorRequestEvent};

use kesko::diagnostic::DiagnosticsPlugins;
use kesko::plugins::CorePlugins;

use kesko::models::{car::CarPlugin, wheely::WheelyPlugin};
use kesko_object_interaction::InteractiveBundle;
use kesko::physics::{
    collider::ColliderShape, event::collision::GenerateCollisionEvents, force::Force,
    gravity::GravityScale, rigid_body::RigidBody, joint::{ JointMotorEvent ,MotorCommand },
};
use kesko::models;

pub fn run_models_demo_app() {
    App::new()
        .add_plugins(CorePlugins::default())
        .add_plugins(DiagnosticsPlugins)
        .add_plugin(CarPlugin)
        .add_plugin(WheelyPlugin)
        .add_startup_system(test_scene)
        .run();
}

fn test_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // asset_server: Res<AssetServer>
) {
    models::arena::spawn(
        &mut commands,
        materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        }),
        &mut meshes,
        10.0,
        10.0,
        0.75,
    );

    models::car::Car::spawn(
        &mut commands,
        materials.add(Color::SEA_GREEN.into()),
        materials.add(Color::DARK_GRAY.into()),
        Transform::from_xyz(0.0, 1.0, -2.0),
        &mut meshes,
    );

    models::spider::spawn(
        &mut commands,
        materials.add(Color::CRIMSON.into()),
        Transform::from_xyz(2.0, 1.0, 0.0),
        &mut meshes,
    );

    models::wheely::Wheely::spawn(
        &mut commands,
        materials.add(Color::FUCHSIA.into()),
        materials.add(Color::DARK_GRAY.into()),
        Transform::from_xyz(-2.0, 1.0, 0.0),
        &mut meshes,
    );

    models::humanoid::Humanoid::spawn(
        &mut commands,
        materials.add(StandardMaterial {
            base_color: Color::ORANGE,
            perceptual_roughness: 1.0,
            ..default()
        }),
        Transform::from_xyz(2.0, 2.0, 2.0),
        &mut meshes,
    );

    models::snake::Snake::spawn(
        &mut commands,
        materials.add(StandardMaterial {
            base_color: Color::ORANGE_RED,  
            perceptual_roughness: 1.0,
            ..default()
        }),
        Transform::from_xyz(0.0, 2.0, 2.0).with_rotation(Quat::from_rotation_x(1.0)),
        &mut meshes,
    );

    // models::gltf_model::GltfModel::spawn(
    //     &mut commands,
    //     "/home/azazdeaz/repos/temp/bevy/assets/models/FlightHelmet/FlightHelmet.gltf#Scene0",
    //     Transform::from_xyz(-2.0, 2.0, 3.0),
    //     &asset_server,
    // );

    // spawn sphere that will generate collision events
    commands.spawn((
        PbrBundle {
            material: materials.add(Color::PURPLE.into()),
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 0.2,
                    subdivisions: 5,
                }
                .try_into()
                .unwrap(),
            ),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        RigidBody::Dynamic,
        ColliderShape::Sphere { radius: 0.2 },
        InteractiveBundle::<GroupDynamic>::default(),
        Force::default(),
        GravityScale::default(),
        GenerateCollisionEvents,
    ));

    // Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.0,
            // Configure the projection to better fit the scene
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });
}