use std::{marker::PhantomData, time::Duration};

use crate::prelude::*;

#[derive(Component, Clone, Debug)]
enum SpawnerState {
    MidBatch { num_left: u32, timer: Timer },
    BetweenBatches(Timer),
    Done,
}
impl SpawnerState {
    fn initial() -> Self {
        let mut rng = thread_rng();
        Self::BetweenBatches(Timer::from_seconds(
            rng.gen_range(0.5..2.0),
            TimerMode::Once,
        ))
    }

    fn new_mid<B: EnemyBundle>(data: &mut EnemySpawner<B>) -> Self {
        let mut rng = thread_rng();
        Self::MidBatch {
            num_left: data.batch_sizes.pop().unwrap_or(0) as u32,
            timer: Timer::from_seconds(
                rng.gen_range(data.batch_rate_range.clone()),
                TimerMode::Once,
            ),
        }
    }

    fn new_between<B: EnemyBundle>(data: &mut EnemySpawner<B>) -> Self {
        let mut rng = thread_rng();
        Self::BetweenBatches(Timer::from_seconds(
            rng.gen_range(data.between_range.clone()),
            TimerMode::Once,
        ))
    }
}

#[derive(Component, Clone, Debug)]
pub struct EnemySpawner<B: EnemyBundle> {
    pd: PhantomData<B>,
    batch_sizes: Vec<usize>,
    /// Range of time to wait between spawning enemies while actively in a batch
    batch_rate_range: Range<f32>,
    /// Range of time to wait between batches
    between_range: Range<f32>,
    pub poses: Vec<Vec2>,
}
impl<B: EnemyBundle> Default for EnemySpawner<B> {
    fn default() -> Self {
        Self {
            pd: default(),
            batch_sizes: vec![],
            batch_rate_range: 0.2..1.0,
            between_range: 3.0..10.0,
            poses: default(),
        }
    }
}

#[derive(Bundle)]
pub struct EnemySpawnerBundle<B: EnemyBundle> {
    name: Name,
    spawner: EnemySpawner<B>,
    state: SpawnerState,
}
impl<B: EnemyBundle> EnemySpawnerBundle<B> {
    pub fn new(poses: Vec<Vec2>, batch_sizes: Vec<usize>) -> Self {
        Self {
            name: Name::new("spawner"),
            spawner: EnemySpawner {
                poses,
                batch_sizes,
                ..default()
            },
            state: SpawnerState::initial(),
        }
    }
}

fn update_spawners<B: EnemyBundle>(
    mut spawners: Query<(Entity, &mut EnemySpawner<B>, &mut SpawnerState)>,
    mut commands: Commands,
    meta_state: Res<State<MetaState>>,
    tutorial_root: Res<TutorialRoot>,
    room_root: Res<RoomRoot>,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
) {
    let relevant_root = if meta_state.get_tutorial_state().is_some() {
        tutorial_root.eid()
    } else {
        room_root.eid()
    };
    let time_factor = time.delta_seconds() * bullet_time.factor();
    let mut rng = thread_rng();
    for (_eid, mut spawner, mut state) in &mut spawners {
        let state_transition = match state.as_mut() {
            SpawnerState::MidBatch { num_left, timer } => {
                if *num_left == 0 {
                    if spawner.batch_sizes.is_empty() {
                        // funny hack
                        // Some(None) means there is some state transition, going to no state
                        Some(SpawnerState::Done)
                    } else {
                        Some(SpawnerState::new_between(&mut spawner))
                    }
                } else {
                    timer.tick(Duration::from_secs_f32(time_factor));
                    if timer.finished() {
                        // Spawn a new bad boi, but no state transition
                        let pos = spawner.poses[rng.gen_range(0..spawner.poses.len())];
                        B::spawn(pos, &mut commands, relevant_root);
                        *num_left -= 1;
                        *timer = Timer::from_seconds(
                            rng.gen_range(spawner.batch_rate_range.clone()),
                            TimerMode::Once,
                        );
                        None
                    } else {
                        // No state transition, keep on waiting
                        None
                    }
                }
            }
            SpawnerState::BetweenBatches(timer) => {
                timer.tick(Duration::from_secs_f32(time_factor));
                if timer.finished() {
                    Some(SpawnerState::new_mid(&mut spawner))
                } else {
                    None
                }
            }
            SpawnerState::Done => continue,
        };
        if let Some(new_state) = state_transition {
            // No rust
            *state = new_state;
        }
    }
}

pub(super) fn register_spawners(app: &mut App) {
    app.add_systems(
        Update,
        update_spawners::<SimpBundle>.run_if(in_state(PhysicsState::Active)),
    );
}
