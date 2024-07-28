use crate::prelude::*;

#[derive(Component)]
pub struct Heart;

#[derive(Bundle)]
pub struct HeartBundle {
    name: Name,
    heart: Heart,
    trigger: HeartTriggerPhysicsBundle,
    multi: MultiAnimationManager,
}
impl HeartBundle {
    pub fn new(pos: Vec2) -> Self {
        let trigger = HeartTriggerPhysicsBundle::new(pos, 6.0);
        Self {
            name: Name::new(format!("heart")),
            heart: Heart,
            trigger,
            multi: multi!([
                (
                    "core",
                    anim_man!({
                        stable: {
                            path: "lenny/heart.png",
                            size: (16, 16),
                            length: 7,
                            fps: 12.0,
                        },
                        explode: {
                            path: "lenny/heart_explode.png",
                            size: (16, 16),
                            length: 6,
                            fps: 16.0,
                            next: "despawn",
                        }
                    })
                ),
                (
                    "light",
                    anim_man!({
                        path: "lenny/heart_light.png",
                        size: (36, 36),
                    })
                    .with_render_layers(LightCamera::render_layers()),
                ),
            ]),
        }
    }
}

fn update_hearts(
    mut hearts: Query<(&mut MultiAnimationManager, &TriggerReceiver), With<Heart>>,
    collisions: Query<&TriggerCollisionRecord>,
    mut commands: Commands,
    mut skills: ResMut<EphemeralSkill>,
) {
    let mut total_inc = 0;
    for (mut multi, trigger) in &mut hearts {
        if multi.manager("core").get_key().as_str() != "stable" {
            continue;
        }
        let mut kinds = HashSet::new();
        for tid in trigger.collisions.iter() {
            kinds.insert(collisions.get(*tid).unwrap().other_kind.clone());
        }
        if kinds.contains(&TriggerKind::Bird) || kinds.contains(&TriggerKind::BulletGood) {
            multi
                .manager_mut("core")
                .reset_key("explode", &mut commands);
            multi.manager_mut("core").force_reset(&mut commands);
            multi.manager_mut("light").set_hidden(false, &mut commands);
            total_inc += 1;
            commands.spawn(SoundEffect::universal("sound_effects/lenny_heal.ogg", 0.1));
        }
    }
    skills.inc_current_health(total_inc);
}

pub(super) fn register_hearts(app: &mut App) {
    app.add_systems(
        Update,
        update_hearts
            .run_if(in_state(PhysicsState::Active))
            .after(PhysicsSet),
    );
}
