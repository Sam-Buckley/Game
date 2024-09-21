use bevy::prelude::*;

pub struct BackgroundPlugin;

#[allow(deprecated)]
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::rgb(0.576, 0.749, 0.902)));
    }
}
