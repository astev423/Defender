use bevy::{
    app::{App, Update},
    ecs::{
        query::With,
        system::{Query, Res},
    },
    input::{ButtonInput, mouse::MouseButton},
    window::{PrimaryWindow, Window},
};

pub fn debug_plugin(app: &mut App) {
    app.add_systems(Update, check_for_click);
}

pub fn check_for_click(
    windows: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = windows.single().expect("More than one window");
        handle_click(window);
    }
}

fn handle_click(window: &Window) {
    print_pos_on_click(window);
}

fn print_pos_on_click(window: &Window) {
    let mouse_pos = window
        .cursor_position()
        .expect("Error getting cursor position");
    println!("{:?}", mouse_pos);
}
