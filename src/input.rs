use bevy::window::PrimaryWindow;

use crate::prelude::*;

// Any place in the app that needs to react to input should use these events and resources
// I've never actually done this but this should _hopefully_ make it easy to plug in controller support

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

/// Mouse state. This may be hard to controler-ize. Oh well.
#[derive(Resource)]
pub struct MouseInput {
    world_pos: Vec2,
    pub buttons: ButtonInput<MouseButton>,
    left_drag_start: Option<Vec2>,
    right_drag_start: Option<Vec2>,
}
impl MouseInput {
    pub fn get_world_pos(&self) -> Vec2 {
        self.world_pos
    }

    pub fn get_left_drag_start(&self) -> Option<Vec2> {
        self.left_drag_start
    }

    pub fn get_right_drag_start(&self) -> Option<Vec2> {
        self.right_drag_start
    }
}

/// The bird can do minimal movement using directional input
#[derive(Resource)]
pub struct MovementInput {
    dir: Vec2,
    fast_stop: bool,
}
impl MovementInput {
    pub fn get_dir(&self) -> Vec2 {
        self.dir
    }

    pub fn get_fast_stop(&self) -> bool {
        self.fast_stop
    }
}

/// Input controlling text boxes
#[derive(Resource)]
pub struct ConvoInput {
    next: bool,
}
impl ConvoInput {
    pub fn get_next(&self) -> bool {
        self.next
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
fn update_mouse_input(
    buttons: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<MouseInput>,
    mut launch_writer: EventWriter<Launch>,
    mut fire_writer: EventWriter<Fire>,
) {
    let Some(mouse_pos) = q_windows.single().cursor_position() else {
        // Mouse is not in the window, don't do anything
        return;
    };
    let world_pos = Vec2::new(
        mouse_pos.x - WINDOW_WIDTH_f32 / 2.0,
        -mouse_pos.y + WINDOW_HEIGHT_f32 / 2.0,
    ) / IDEAL_GROWTH_f32;
    let left_drag_start = if buttons.just_pressed(MouseButton::Left) {
        Some(world_pos)
    } else {
        if let Some(drag_start) = state.left_drag_start {
            if !buttons.pressed(MouseButton::Left) {
                launch_writer.send(Launch(drag_start - world_pos));
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
                fire_writer.send(Fire(drag_start - world_pos));
                None
            } else {
                Some(drag_start)
            }
        } else {
            None
        }
    };
    *state = MouseInput {
        world_pos,
        buttons: buttons.clone(),
        left_drag_start,
        right_drag_start,
    };
}

fn update_movement_input(keyboard: Res<ButtonInput<KeyCode>>, mut movement: ResMut<MovementInput>) {
    let mut unnormal = Vec2::ZERO;
    if keyboard.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        unnormal += -Vec2::X;
    }
    if keyboard.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        unnormal += Vec2::X;
    }
    if keyboard.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
        unnormal += Vec2::Y;
    }
    if keyboard.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
        unnormal += -Vec2::Y;
    }
    movement.dir = unnormal;
    movement.fast_stop = keyboard.pressed(KeyCode::Space);
}

fn update_convo_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut convo: ResMut<ConvoInput>,
) {
    convo.next = keyboard.any_just_pressed([
        KeyCode::Space,
        KeyCode::KeyA,
        KeyCode::KeyW,
        KeyCode::KeyS,
        KeyCode::KeyD,
    ]) || mouse.any_just_pressed([MouseButton::Left, MouseButton::Right]);
}

/// Send any and all non-game input. Note the early returns, we only handle at most one
/// such input per frame
fn watch_non_game_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut non_game_writer: EventWriter<NonGameInput>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        non_game_writer.send(NonGameInput::Continue);
        return;
    }
}

pub(super) struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        // Resources
        app.insert_resource(MouseInput {
            world_pos: default(),
            buttons: default(),
            left_drag_start: None,
            right_drag_start: None,
        });
        app.insert_resource(MovementInput {
            dir: default(),
            fast_stop: false,
        });
        app.insert_resource(ConvoInput { next: false });

        // Events
        app.add_event::<Launch>();
        app.add_event::<Fire>();
        app.add_event::<NonGameInput>();

        // Systems
        app.add_systems(
            Update,
            (
                update_mouse_input,
                update_movement_input,
                update_convo_input,
                watch_non_game_input,
            )
                .in_set(InputSet),
        );
    }
}
