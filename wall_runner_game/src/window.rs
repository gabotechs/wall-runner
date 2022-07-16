use bevy::prelude::*;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "BevyTutorial".into(),
            ..WindowDescriptor::default()
        })
        .add_startup_system(init_window);
    }
}

fn init_window(mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    // send to the right monitor
    window.set_position(IVec2::new(2000, 0));
    window.set_maximized(true);
    // window.set_mode(WindowMode::Fullscreen);
}
