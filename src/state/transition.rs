use std::time::Duration;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum TransitionKind {
    /// Will fade to black, update the state, then unfade from black
    FadeToBlack,
}
impl TransitionKind {
    pub fn to_meta_transition_state(
        &self,
        duration_f32: f32,
        next_state: MetaState,
    ) -> MetaTransitionState {
        MetaTransitionState::Volatile {
            transition: Transition {
                kind: *self,
                duration: Duration::from_secs_f32(duration_f32),
            },
            next_state,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub struct Transition {
    kind: TransitionKind,
    duration: Duration,
}

/// The state to care about for entering and exiting transitions
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
struct InMetaTransition;
impl ComputedStates for InMetaTransition {
    type SourceStates = MetaTransitionState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            MetaTransitionState::Stable => None,
            MetaTransitionState::Volatile { .. } => Some(Self),
        }
    }
}

#[derive(Component)]
struct TransitionRoot {
    kind: TransitionKind,
    switch_timer: Option<Timer>,
    full_timer: Timer,
    next_state: MetaState,
}
#[derive(Bundle)]
struct TransitionRootBundle {
    name: Name,
    root: TransitionRoot,
    spatial: SpatialBundle,
}
impl TransitionRootBundle {
    fn new(kind: TransitionKind, next_state: MetaState, duration: Duration) -> Self {
        Self {
            name: Name::new("transition_root"),
            root: TransitionRoot {
                kind,
                switch_timer: Some(Timer::new(duration.div_f32(2.0), TimerMode::Once)),
                full_timer: Timer::new(duration, TimerMode::Once),
                next_state,
            },
            spatial: SpatialBundle::default(),
        }
    }
}

#[derive(Component)]
struct FadeToBlackSprite;
impl TransitionKind {
    fn spawn(&self, parent: &mut ChildBuilder) {
        match self {
            Self::FadeToBlack => {
                parent.spawn((
                    Name::new("black_box"),
                    FadeToBlackSprite,
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgba(0.0, 0.0, 0.0, 0.0),
                            custom_size: Some(WINDOW_VEC_f32),
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
        }
    }
}

fn setup_transition(
    meta_transition_state: Res<State<MetaTransitionState>>,
    mut commands: Commands,
) {
    let MetaTransitionState::Volatile {
        transition,
        next_state,
    } = meta_transition_state.get()
    else {
        return;
    };
    let root = TransitionRootBundle::new(transition.kind, next_state.clone(), transition.duration);
    commands.spawn(root).with_children(|parent| {
        transition.kind.spawn(parent);
    });
}

fn update_transition(
    mut root_q: Query<&mut TransitionRoot>,
    time: Res<Time>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
    mut next_transition_state: ResMut<NextState<MetaTransitionState>>,
    mut fade_to_black_sprite: Query<&mut Sprite, With<FadeToBlackSprite>>,
) {
    // Handle the timers
    let mut root = root_q.single_mut();
    let should_switch = match root.switch_timer.as_mut() {
        Some(timer) => {
            timer.tick(time.delta());
            timer.finished()
        }
        None => false,
    };
    if should_switch {
        next_meta_state.set(root.next_state);
        root.switch_timer = None;
    }
    root.full_timer.tick(time.delta());
    if root.full_timer.finished() {
        next_transition_state.set(MetaTransitionState::Stable);
    }
    // Update the transition
    match root.kind {
        TransitionKind::FadeToBlack => {
            let mut sprite = fade_to_black_sprite.single_mut();
            let alpha = 1.0 - (root.full_timer.fraction() * 2.0 - 1.0).abs();
            sprite.color = Color::srgba(0.0, 0.0, 0.0, alpha);
        }
    }
}

fn destroy_transition(roots: Query<Entity, With<TransitionRoot>>, mut commands: Commands) {
    for root in &roots {
        commands.entity(root).despawn_recursive();
    }
}

pub(super) fn register_transition(app: &mut App) {
    app.add_computed_state::<InMetaTransition>();
    app.add_systems(OnEnter(InMetaTransition), setup_transition);
    app.add_systems(Update, update_transition.run_if(in_state(InMetaTransition)));
    app.add_systems(OnExit(InMetaTransition), destroy_transition);
}
