use bevy::{prelude::*, core_pipeline::clear_color::ClearColorConfig};
use bevy_postproc::prelude::*;

// This is the component that will get passed to the shader
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
struct PostProcessSettings {
    intensity: f32,
    // WebGL2 structs must be 16 byte aligned.
    #[cfg(feature = "webgl2")]
    _webgl2_padding: Vec3,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(
            PostProcessingPlugin::new()
                .with(PostProcessingEffect {
                    path: "shaders/post_processing.wgsl"
                })
                .build()
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (
            rotate,
            update_settings,
        ))
        .run();
}

/// Set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
                .looking_at(Vec3::default(), Vec3::Y),
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
                ..default()
            },
            ..default()
        },
        // Add the setting to the camera.
        // This component is also used to determine on which camera to run the post processing effect.
        PostProcessSettings {
            intensity: 0.02,
            ..default()
        },
    ));

    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Rotates,
    ));
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });
}

#[derive(Component)]
struct Rotates;

/// Rotates any entity around the x and y axis
fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_seconds());
        transform.rotate_z(0.15 * time.delta_seconds());
    }
}

// Change the intensity over time to show that the effect is controlled from the main world
fn update_settings(mut settings: Query<&mut PostProcessSettings>, time: Res<Time>) {
    for mut setting in &mut settings {
        let mut intensity = time.elapsed_seconds().sin();
        // Make it loop periodically
        intensity = intensity.sin();
        // Remap it to 0..1 because the intensity can't be negative
        intensity = intensity * 0.5 + 0.5;
        // Scale it to a more reasonable level
        intensity *= 0.015;

        // Set the intensity.
        // This will then be extracted to the render world and uploaded to the gpu automatically by the [`UniformComponentPlugin`]
        setting.intensity = intensity;
    }
}