use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::setup_camera);
    }
}

impl CameraPlugin {
    fn setup_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
        let window = window_query.get_single().unwrap();
        //spawn a camera centered at the window's center
        commands.spawn(Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0),
            ..Default::default()
        });
    }
}
