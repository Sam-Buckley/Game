use bevy::prelude::*;
use bevy::window::PrimaryWindow;

// Make a movable camera
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera);
    }
}

fn camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let (width, height) = (window.width(), window.height());
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(width / 2.0, height / 2.0, 10.0)),
        ..Default::default()
    });
}
