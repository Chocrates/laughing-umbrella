use bevy::color::palettes::css::*;
use bevy::input::keyboard::KeyCode;
use bevy::pbr::experimental::meshlet::MeshletPlugin;
use bevy::pbr::wireframe::{Wireframe, WireframeColor, WireframePlugin};
use bevy::prelude::*;

#[derive(Component)]
struct CameraController {
    speed: f32,
    sensitivity: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshletPlugin) // New feature for handling high-poly meshes efficiently
        .add_plugins(WireframePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_movement_system)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Camera with Depth of Field and Auto Exposure
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                hdr: true, // Enable HDR for advanced lighting
                ..default()
            },
            ..default()
        })
        .insert(CameraController {
            speed: 5.0,
            sensitivity: 0.2,
        });

    // Light source with volumetric fog
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            range: 20.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0),
        reflectance: 1.0,
        unlit: false,
        ..default()
    });

    // Spawn a simple textured mesh with PBR material
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere::default())),
            material: cube_material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Wireframe,
        WireframeColor { color: LIME.into() },
    ));

    // Add some UI with rounded corners
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(100.00),
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.8)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Welcome to Bevy v0.14!",
                TextStyle {
                    font: asset_server.load("fonts/ShadeBlue-2OozX.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
        });
}

fn camera_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &CameraController)>,
) {
    for (mut transform, controller) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            let forward = transform.forward();
            let right = transform.right();
            let movement = (forward * direction.z + right * direction.x)
                * controller.speed
                * time.delta_seconds();
            transform.translation += movement;
        }
    }
}
