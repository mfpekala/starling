use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    text::Text2dBounds,
};

use crate::prelude::*;

use super::DebugState;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
struct ShowCommands;
impl ComputedStates for ShowCommands {
    type SourceStates = (AppMode, DebugState);

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        let (app_mode, debug_state) = sources;
        if matches!(app_mode, AppMode::Prod) {
            return None;
        }
        if debug_state.show_commands {
            Some(Self)
        } else {
            None
        }
    }
}

const BG_SIZE: UVec2 = UVec2::new(160, 20);

#[derive(Component, Default)]
struct CommandsBg {
    focused: bool,
    input: String,
    outputs: Vec<String>,
}
#[derive(Bundle)]
struct CommandsBgBundle {
    name: Name,
    marker: CommandsBg,
    multi: MultiAnimationManager,
    spatial: SpatialBundle,
}
impl CommandsBgBundle {
    fn new() -> Self {
        Self {
            name: Name::new("commands_bg"),
            marker: CommandsBg::default(),
            multi: MultiAnimationManager::from_single(
                AnimationManager::from_nodes(vec![
                    (
                        "unfocused",
                        AnimationNode::new_static(
                            SpriteInfo::new("debug/commands_unfocused.png", BG_SIZE.x, BG_SIZE.y)
                                .with_color(Color::srgba(1.0, 1.0, 1.0, 0.4)),
                        ),
                    ),
                    (
                        "focused",
                        AnimationNode::new_static(
                            SpriteInfo::new("debug/commands_focused.png", BG_SIZE.x, BG_SIZE.y)
                                .with_color(Color::srgba(1.0, 1.0, 1.0, 0.8)),
                        ),
                    ),
                ])
                .with_render_layers(MenuCamera::render_layers()),
            ),
            spatial: spat_tran(
                (IDEAL_WIDTH_f32 - BG_SIZE.x as f32) / 2.0,
                (-IDEAL_HEIGHT_f32 + BG_SIZE.y as f32) / 2.0,
                0.0,
            ),
        }
    }
}

#[derive(Component, Default)]
struct CommandsInputText;
#[derive(Component, Default)]
struct CommandsOutputText;
#[derive(Bundle)]
struct CommandsTextBundle<C: Component + Default> {
    name: Name,
    marker: C,
    text: Text2dBundle,
    render_layers: RenderLayers,
}
impl<C: Component + Default> CommandsTextBundle<C> {
    fn new(name: &str, offset: Vec2) -> Self {
        Self {
            name: Name::new(name.to_string()),
            marker: C::default(),
            text: Text2dBundle {
                transform: Transform::from_translation(offset.extend(1.0)),
                text: Text::from_section(
                    "HELLO",
                    TextStyle {
                        font_size: 10.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                text_2d_bounds: Text2dBounds {
                    size: Vec2::new(BG_SIZE.x as f32, BG_SIZE.y as f32 / 2.0),
                },
                ..default()
            },
            render_layers: MenuCamera::render_layers(),
        }
    }
}

fn setup_debug_commands(mut commands: Commands, debug_root: Res<DebugRoot>) {
    commands
        .spawn(CommandsBgBundle::new())
        .with_children(|bg| {
            bg.spawn(CommandsTextBundle::<CommandsInputText>::new(
                "input_text",
                Vec2::new(0.0, -5.0),
            ));
            bg.spawn(CommandsTextBundle::<CommandsOutputText>::new(
                "output_text",
                Vec2::new(0.0, 5.0),
            ));
        })
        .set_parent(debug_root.eid());
}

fn update_debug_commands(
    mut bg: Query<(&mut CommandsBg, &mut MultiAnimationManager)>,
    mut keyboard: ResMut<ButtonInput<KeyCode>>,
    mut evr_char: EventReader<KeyboardInput>,
    mut commands: Commands,
    mut input_text: Query<&mut Text, (With<CommandsInputText>, Without<CommandsOutputText>)>,
    mut output_text: Query<&mut Text, (Without<CommandsInputText>, With<CommandsOutputText>)>,
) {
    let (mut bg, mut multi) = bg.single_mut();
    let mut input_text = input_text.single_mut();
    let mut output_text = output_text.single_mut();
    // First just do styling
    multi.single_mut().set_key(
        if bg.focused { "focused" } else { "unfocused" },
        &mut commands,
    );
    let text_color = if bg.focused {
        Color::WHITE
    } else {
        Color::srgba(1.0, 1.0, 1.0, 0.4)
    };
    input_text.sections[0].value = bg.input.clone();
    input_text.sections[0].style.color = text_color;
    output_text.sections[0].value = bg
        .outputs
        .iter()
        .last()
        .unwrap_or(&"No output yet".to_string())
        .to_string();
    output_text.sections[0].style.color = text_color;
    // Then actually do logic
    if !bg.focused {
        bg.focused = keyboard.just_pressed(KeyCode::Slash);
    } else {
        if keyboard.just_pressed(KeyCode::Escape) {
            bg.focused = false;
            bg.input = default();
        } else if keyboard.just_pressed(KeyCode::Enter) {
            // TODO: Actual commands
            bg.focused = false;
            bg.input = default();
        } else {
            let chars: Vec<&KeyboardInput> = evr_char.read().collect();
            for input in chars {
                if input.state != ButtonState::Pressed {
                    continue;
                }
                if matches!(input.logical_key.clone(), Key::Backspace) {
                    bg.input.pop();
                }
                if matches!(input.logical_key.clone(), Key::Space) {
                    bg.input += " ";
                }
                let Key::Character(c) = input.logical_key.clone() else {
                    continue;
                };
                let c = c.as_str();
                if !["/", "\n", "\u{8}"].contains(&c) {
                    bg.input += c;
                }
            }
        }
        keyboard.reset_all();
    }
}

fn destroy_debug_commands(mut commands: Commands, bg: Query<Entity, With<CommandsBg>>) {
    commands.entity(bg.single()).despawn_recursive();
}

pub(super) fn register_commands_debug(app: &mut App) {
    app.add_computed_state::<ShowCommands>();
    app.add_systems(OnEnter(ShowCommands), setup_debug_commands);
    app.add_systems(
        Update,
        update_debug_commands
            .run_if(in_state(ShowCommands))
            .before(InputSet),
    );
    app.add_systems(OnExit(ShowCommands), destroy_debug_commands);
}
