use crate::prelude::*;

#[derive(Component)]
pub struct GoNext;

#[derive(Bundle)]
pub struct GoNextBundle {
    name: Name,
    go_next: GoNext,
    trigger: GoNextTriggerPhysicsBundle,
    multi: MultiAnimationManager,
}
impl GoNextBundle {
    pub fn new(pos: Vec2) -> Self {
        let trigger = GoNextTriggerPhysicsBundle::new(pos, 6.0);
        Self {
            name: Name::new(format!("go_next")),
            go_next: GoNext,
            trigger,
            multi: multi!([
                (
                    "core",
                    anim_man!({
                        stable: {
                            path: "lenny/continue.png",
                            size: (16, 16),
                            length: 3,
                            fps: 8.0,
                        },
                        none: {
                            path: "sprites/none.png",
                            size: (1, 1),
                            length: 2,
                            next: "despawn",
                        }
                    })
                ),
                (
                    "light",
                    anim_man!({
                        path: "lenny/continue_light.png",
                        size: (36, 36),
                    })
                    .with_render_layers(LightCamera::render_layers()),
                ),
            ]),
        }
    }
}

fn update_go_next(
    mut hearts: Query<(&mut MultiAnimationManager, &TriggerReceiver), With<GoNext>>,
    collisions: Query<&TriggerCollisionRecord>,
    mut commands: Commands,
    meta_state: Res<State<MetaState>>,
    mut next_meta_transition: ResMut<NextState<MetaTransitionState>>,
    birds: Query<Entity, With<Bird>>,
) {
    // Too tired to do this with fancy iterator stuff
    let mut any_hit = false;
    for (mut multi, trigger) in &mut hearts {
        if multi.manager("core").get_key().as_str() != "stable" {
            continue;
        }
        let mut kinds = HashSet::new();
        for tid in trigger.collisions.iter() {
            kinds.insert(collisions.get(*tid).unwrap().other_kind.clone());
        }
        if kinds.contains(&TriggerKind::BulletGood) {
            multi.manager_mut("core").reset_key("none", &mut commands);
            multi.manager_mut("light").set_hidden(false, &mut commands);
            any_hit = true;
            break;
        }
    }
    if any_hit {
        if let Some(room_state) = meta_state.get_room_state() {
            next_meta_transition.set(
                TransitionKind::FadeToBlack
                    .to_meta_transition_state(1.0, room_state.next_room().to_meta_state()),
            );
            for eid in &birds {
                // We really shouldn't need to do this, skill issue on my part
                // Basically I fucked up when the health bar gets spawned/despawned...
                // and I also fucked up who should own the current health data...
                // So yeah, without a refactor that's not worth the time in a jam this is necessary
                // to get everything (namely healthbar) in the next room working well
                commands.entity(eid).despawn_recursive();
            }
            commands.spawn(SoundEffect::universal(
                "sound_effects/lenny_go_next.ogg",
                0.1,
            ));
        }
    }
}

pub(super) fn register_go_next(app: &mut App) {
    app.add_systems(
        Update,
        update_go_next
            .run_if(in_state(PhysicsState::Active))
            .after(PhysicsSet),
    );
}
