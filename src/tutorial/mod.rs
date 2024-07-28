use crate::prelude::*;

mod dead;
pub(self) mod fly_spots;
mod impossible_boss;
mod learn_to_fly;
mod learn_to_shoot;
pub(self) mod targets;

#[derive(Component)]
struct HelpText;

/// Sets up the tutorial.
/// NOTE: For simplicity, you can't save game mid tutorial, i.e. we are assuming that
/// the state transitions will always go:
///   (non-tutorial) -> TutorialState::LearnFlight -> TutorialState::LearnShoot -> (non-tutorial)
/// This means setup_tutorial only happens OnEnter(MetaState::Tutorial(TutorialState::LearnFlight))
/// TODO: Can probably fix this with another computedstate (AnyTutorial) but not bothering rn
fn setup_tutorial(
    mut commands: Commands,
    tutorial_root: Res<TutorialRoot>,
    mut next_convo_state: ResMut<NextState<ConvoState>>,
    mut permanent_skills: ResMut<PermanentSkill>,
    mut ephemeral_skills: ResMut<EphemeralSkill>,
) {
    permanent_skills.force_set_num_launches(0);
    permanent_skills.force_set_num_bullets(0);
    permanent_skills.force_set_max_health(3);
    ephemeral_skills.start_attempt(&permanent_skills);
    next_convo_state.set(ConvoState::TutorialEggUnwrap);
    commands
        .spawn(HardPlatformBundle::around_room())
        .set_parent(tutorial_root.eid());
    commands
        .spawn(StickyPlatformBundle::new(
            "chain1",
            Vec2::new(-90.0, -30.0),
            Shape::Circle { radius: 15.0 },
        ))
        .insert(DynoRot { rot: 2.0 })
        .set_parent(tutorial_root.eid());
    commands
        .spawn(StickyPlatformBundle::new(
            "chain2",
            Vec2::new(-10.0, 10.0),
            Shape::Circle { radius: 25.0 },
        ))
        .insert(DynoRot { rot: -4.0 })
        .set_parent(tutorial_root.eid());
    commands
        .spawn(StickyPlatformBundle::new(
            "chain3",
            Vec2::new(70.0, 35.0),
            Shape::Circle { radius: 15.0 },
        ))
        .insert(DynoRot { rot: 2.0 })
        .set_parent(tutorial_root.eid());
    commands
        .spawn(StickyPlatformBundle::new(
            "perch",
            Vec2::new(140.0, 45.0),
            Shape::Polygon {
                points: simple_rect(40.0, 10.0),
            },
        ))
        .set_parent(tutorial_root.eid());
    commands
        .spawn(BirdBundle::new(
            Vec2::new(125.0, -78.0),
            Vec2::ZERO,
            ephemeral_skills.get_num_launches(),
            ephemeral_skills.get_num_bullets(),
            0,
        ))
        .set_parent(tutorial_root.eid());
    commands
        .spawn(GhostBundle::new(Vec2::new(140.0, 70.0), true, true))
        .set_parent(tutorial_root.eid());
    commands
        .spawn((
            Name::new("help_text"),
            HelpText,
            Text2dBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        font_size: 12.0,
                        ..default()
                    },
                )
                .with_justify(JustifyText::Center),
                transform: Transform::from_translation(Vec3::new(0.0, 60.0, ZIX_BIRD - 0.5)),
                ..default()
            },
            SpriteCamera::render_layers(),
        ))
        .set_parent(tutorial_root.eid());
    BackgroundKind::Zenith.spawn(Vec2::ZERO, tutorial_root.eid(), &mut commands);
}

/// Sets up the tutorial.
fn destroy_tutorial(mut commands: Commands, tutorial_root: Res<TutorialRoot>) {
    commands.entity(tutorial_root.eid()).despawn_descendants();
}

pub(super) struct TutorialPlugin;
impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(TutorialState::LearnToFly.to_meta_state()),
            setup_tutorial,
        );
        app.add_systems(
            OnExit(TutorialState::Dead.to_meta_state()),
            destroy_tutorial,
        );

        learn_to_fly::register_learn_to_fly(app);
        learn_to_shoot::register_learn_to_shoot(app);
        impossible_boss::register_impossible_boss(app);
        dead::register_dead(app);
    }
}
