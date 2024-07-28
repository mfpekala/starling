use rand::thread_rng;
use rand::Rng;

use crate::prelude::*;

#[derive(Component, Reflect, Clone, Copy)]
pub enum UpgradeKind {
    NumLaunches(u32),
    NumBullets(u32),
    MaxHealth(u32),
}
impl UpgradeKind {
    fn rarity_text(amt: u32) -> String {
        match amt {
            1 => "Common".into(),
            2 => "Rare".into(),
            3 => "Epic".into(),
            _ => "Unknown".into(),
        }
    }

    pub fn new(prob_rare: f32, prob_epic: f32) -> Self {
        let mut rng = thread_rng();
        let smpl = rng.gen::<f32>();
        let amt = {
            if smpl < prob_epic {
                3_u32
            } else if smpl < prob_epic + prob_rare {
                2_u32
            } else {
                1_u32
            }
        };
        let smpl = rng.gen::<f32>();
        if smpl < 0.33 {
            Self::NumLaunches(amt)
        } else if smpl < 0.67 {
            Self::NumBullets(amt)
        } else {
            Self::MaxHealth(amt)
        }
    }

    pub fn to_button_text(&self) -> String {
        match self {
            Self::NumLaunches(amt) => format!(
                "STRONGER\n# of Launches +{amt}\n({})",
                Self::rarity_text(*amt)
            ),
            Self::NumBullets(amt) => format!(
                "DEADLIER\n# of Bullets +{amt}\n({})",
                Self::rarity_text(*amt)
            ),
            Self::MaxHealth(amt) => {
                format!("TOUGHER\nMax Health +{amt}\n({})", Self::rarity_text(*amt))
            }
        }
    }

    pub fn apply(&self, permanent_skill: &mut PermanentSkill) {
        match self {
            Self::NumLaunches(amt) => permanent_skill.increase_num_launches(*amt),
            Self::NumBullets(amt) => permanent_skill.increase_num_bullets(*amt),
            Self::MaxHealth(amt) => permanent_skill.increase_max_health(*amt),
        }
    }
}

#[derive(Component, Reflect)]
pub struct UpgradeButton {
    pub ix: u32,
    kind: UpgradeKind,
}

#[derive(Component)]
pub(super) struct Hovered;

#[derive(Component)]
pub struct UpgradeButtonApplied;

#[derive(Bundle)]
pub struct UpgradeButtonBundle {
    name: Name,
    button: UpgradeButton,
    spatial: SpatialBundle,
    multi: MultiAnimationManager,
}
impl UpgradeButtonBundle {
    const SIZE: Vec2 = Vec2::new(100.0, 50.0);

    pub fn spawn(ix: u32, pos: Vec2, kind: UpgradeKind, commands: &mut Commands, parent: Entity) {
        commands
            .spawn(UpgradeButtonBundle {
                name: Name::new("upgrade_button"),
                button: UpgradeButton { ix, kind },
                spatial: spat_tran(pos.x, pos.y, ZIX_UPGRADE_BUTTON),
                multi: multi!([
                    (
                        "outline",
                        anim_man!({
                            path: "lenny/skill_tree_button_outline.png",
                            size: (Self::SIZE.x as u32, Self::SIZE.y as u32),
                        })
                        .with_offset(Vec3::Z * 0.2)
                        .with_render_layers(MenuCamera::render_layers())
                    ),
                    (
                        "fill",
                        anim_man!({
                            path: "lenny/skill_tree_button_fill.png",
                            size: (Self::SIZE.x as u32, Self::SIZE.y as u32),
                            color: Color::BLACK,
                        })
                        .with_render_layers(MenuCamera::render_layers())
                    ),
                ]),
            })
            .set_parent(parent)
            .with_children(|dad| {
                dad.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            kind.to_button_text(),
                            TextStyle {
                                font_size: 10.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_justify(JustifyText::Center),
                        // text_2d_bounds: Text2dBounds {
                        //     size: Vec2::new(Self::SIZE.x, Self::SIZE.y),
                        // },
                        transform: Transform::from_translation(Vec3::Z),
                        ..default()
                    },
                    MenuCamera::render_layers(),
                ));
            });
    }
}

pub(super) fn update_upgrade_buttons(
    mouse_input: Res<MouseInput>,
    buttons: Query<(Entity, &GlobalTransform, &UpgradeButton)>,
    mut commands: Commands,
    mut permanent_skills: ResMut<PermanentSkill>,
    already_applied: Query<&UpgradeButtonApplied>,
) {
    let world_pos = mouse_input.get_world_pos();
    for (eid, gt, data) in &buttons {
        let gt = gt.translation();
        let hovered_x = (world_pos.x - gt.x).abs() < UpgradeButtonBundle::SIZE.x / 2.0;
        let hovered_y = (world_pos.y - gt.y).abs() < UpgradeButtonBundle::SIZE.y / 2.0;
        let hovered = hovered_x && hovered_y;
        if hovered {
            commands.entity(eid).insert(Hovered);
            if already_applied.is_empty() && mouse_input.buttons.just_released(MouseButton::Left) {
                commands.entity(eid).insert(UpgradeButtonApplied);
                data.kind.apply(&mut permanent_skills);
                commands.spawn(SoundEffect::universal(
                    "sound_effects/choose_upgrade.ogg",
                    0.2,
                ));
            }
        } else {
            commands.entity(eid).remove::<Hovered>();
        }
    }
}

pub(super) fn color_upgrade_buttons(
    mut buttons: Query<
        (
            &mut MultiAnimationManager,
            &Children,
            Option<&Hovered>,
            Option<&UpgradeButtonApplied>,
        ),
        With<UpgradeButton>,
    >,
    mut text: Query<&mut Text>,
    mut commands: Commands,
) {
    for (mut multi, children, hovered, applied) in &mut buttons {
        let is_white = hovered.is_some() || applied.is_some();
        let (fill_color, text_color) = if is_white {
            (Color::WHITE, Color::BLACK)
        } else {
            (Color::BLACK, Color::WHITE)
        };
        multi
            .manager_mut("fill")
            .map
            .values_mut()
            .for_each(|node| node.sprite.color = fill_color);
        multi.manager_mut("fill").force_reset(&mut commands);
        let text_eid = children.iter().next().unwrap();
        let mut text = text.get_mut(*text_eid).unwrap();
        text.sections[0].style.color = text_color;
    }
}
