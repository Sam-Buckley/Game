use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Event)]
pub struct LossEvent(pub bool);

pub struct LossPlugin;

impl Plugin for LossPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LossEvent>();
        app.add_systems(Update, loss_system);
    }
}

fn loss_system(
    mut commands: Commands,
    mut loss_events: EventReader<LossEvent>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // read into a variable
    for event in loss_events.read() {
        if event.0 {
            let window = window_query.get_single().unwrap();
            let mut transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 1.0);
            // Scale the loss sprite (660x371) to the window size
            transform.scale = Vec3::splat(window.width() / 660.0);
            // Streth the loss sprite to the window size if needed
            transform.scale = Vec3::splat((window.height() / 371.0) + 0.15);
            commands.spawn(SpriteBundle {
                texture: asset_server.load("backdrops/loss_screen.png"),
                transform,
                ..Default::default()
            });
        }
    }
}