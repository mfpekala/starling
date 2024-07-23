use bevy::window::PrimaryWindow;

use crate::prelude::*;

// Any place in the app that needs to react to input should use these events and resources
// I've never actually done this but this should _hopefully_ make it easy to plug in controller support

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

/// Mouse state. This may be hard to controler-ize. Oh well.
#[derive(Resource)]
pub struct MouseState {
    world_pos: Vec2,
    pub buttons: ButtonInput<MouseButton>,
    left_drag_start: Option<Vec2>,
    right_drag_start: Option<Vec2>,
}
impl MouseState {
    pub fn get_world_pos(&self) -> Vec2 {
        self.world_pos
    }
}

/// Event that corresponds to input that _should_ send a bird flying.
/// Usually means click and drag left mouse button.
#[derive(Event)]
pub struct Launch(pub Vec2);

/// Event that corresponds to input that _should_ shoot a bullet.
/// Usually means click and drag right mouse button.
#[derive(Event)]
pub struct Fire(pub Vec2);

/// Pretty wide net, but should cover all input in menus, cutscenes, pause screens, convos...
#[derive(Event)]
pub enum NonGameInput {
    Continue,
}

// INTERNAL INPUT SYSTEM (ONLY USED IN THIS FILE)

/// Updates the `MouseState` resource.
fn update_mouse_state(
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<MouseState>,
    mut launch_writer: EventWriter<Launch>,
    mut fire_writer: EventWriter<Fire>,
) {
    let Some(mouse_pos) = q_windows.single().cursor_position() else {
        // Mouse is not in the window, don't do anything
        return;
    };
    let world_pos = Vec2::new(mouse_pos.x, -mouse_pos.y);
    let left_drag_start = if buttons.just_pressed(MouseButton::Left) {
        Some(world_pos)
    } else {
        if let Some(drag_start) = state.left_drag_start {
            if !buttons.pressed(MouseButton::Left) {
                launch_writer.send(Launch(world_pos - drag_start));
                None
            } else {
                Some(drag_start)
            }
        } else {
            None
        }
    };
    let right_drag_start = if buttons.just_pressed(MouseButton::Right) {
        Some(world_pos)
    } else {
        if let Some(drag_start) = state.right_drag_start {
            if !buttons.pressed(MouseButton::Right) {
                fire_writer.send(Fire(world_pos - drag_start));
                None
            } else {
                Some(drag_start)
            }
        } else {
            None
        }
    };
    *state = MouseState {
        world_pos,
        buttons: buttons.clone(),
        left_drag_start,
        right_drag_start,
    };
}

/// Send any and all non-game input. Note the early returns, we only handle at most one
/// such input per frame
fn watch_non_game_input(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Enter) {
        commands.trigger(NonGameInput::Continue);
        return;
    }
}

pub(super) struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        app.insert_resource(MouseState {
            world_pos: default(),
            buttons: default(),
            left_drag_start: None,
            right_drag_start: None,
        });

        // Events
        app.add_event::<Launch>();
        app.add_event::<Fire>();
        app.add_event::<NonGameInput>();

        // Systems
        app.add_systems(
            Update,
            (update_mouse_state, watch_non_game_input).in_set(InputSet),
        );
    }
}
