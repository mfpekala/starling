use crate::prelude::*;
use rand::Rng;

/// ChatGpt for the... mediocre code?
pub fn generate_circles(
    num_circles: u32,
    bot_left: Vec2,
    top_right: Vec2,
    rad_range: (f32, f32),
    rot_range: (f32, f32),
    dist_between: f32,
    also_avoid: &[(Shape, Vec2, f32)],
) -> Vec<(Shape, Vec2, f32)> {
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();

    for _ in 0..num_circles {
        let radius = rng.gen_range(rad_range.0..rad_range.1);
        let rot = rng.gen_range(rot_range.0..rot_range.1);

        for _ in 0..100 {
            let x = rng.gen_range(bot_left.x + dist_between..top_right.x - dist_between);
            let y = rng.gen_range(bot_left.y + dist_between..top_right.y - dist_between);
            let center = Vec2 { x, y };

            if is_valid(
                &center,
                radius,
                &result,
                also_avoid,
                bot_left,
                top_right,
                dist_between,
            ) {
                result.push((Shape::Circle { radius }, center, rot));
                break;
            }
        }
    }

    result
}

fn is_valid(
    center: &Vec2,
    radius: f32,
    circles: &[(Shape, Vec2, f32)],
    also_avoid: &[(Shape, Vec2, f32)],
    bot_left: Vec2,
    top_right: Vec2,
    dist_between: f32,
) -> bool {
    // Check distance from edges
    if center.x - radius < bot_left.x + dist_between
        || center.x + radius > top_right.x - dist_between
        || center.y - radius < bot_left.y + dist_between
        || center.y + radius > top_right.y - dist_between
    {
        return false;
    }

    // Check distance from other circles
    for (shape, other_center, _) in circles.iter().chain(also_avoid.iter()) {
        if let Shape::Circle {
            radius: other_radius,
        } = shape
        {
            let distance = center.distance(*other_center);
            let min_dist = radius + other_radius + dist_between;
            if distance < min_dist {
                return false;
            }
        }
    }

    true
}

/// This is the system responsible for randomly generating each room.
/// First it generates the spinning wheels.
/// Then it places the spawners.
fn create_room(
    mut commands: Commands,
    room_root: Res<RoomRoot>,
    mut music_manager: ResMut<MusicManager>,
    encounter_state: Res<State<EncounterState>>,
    mut permanent_skills: ResMut<PermanentSkill>,
    mut ephemeral_skills: ResMut<EphemeralSkill>,
) {
    // Clear the room just to be sure
    commands.entity(room_root.eid()).despawn_descendants();

    let encounter_state = encounter_state.get();

    // Enfource minimums
    // Mind is too messy to keep the full state transitions in the working set
    // If for whatever reason we end up here with stupid defaults, fix them
    // Once again, skill issue
    if permanent_skills.get_num_launches() < 2 {
        permanent_skills.force_set_num_launches(2);
    }
    if permanent_skills.get_num_bullets() < 3 {
        permanent_skills.force_set_num_bullets(3);
    }
    if permanent_skills.get_max_health() < 3 {
        permanent_skills.force_set_max_health(3);
    }

    match encounter_state.kind {
        EncounterKind::SteelbeakOnly => {
            if encounter_state.difficulty == 1 {
                // YO we found it, the place where attempts start
                ephemeral_skills.start_attempt(&permanent_skills);
            }

            music_manager.fade_to_song(MusicKind::NormalBattle); // remember this does nothing if it's already this song

            // Background and room border
            BackgroundKind::Zenith.spawn(default(), room_root.eid(), &mut commands);
            commands
                .spawn(HardPlatformBundle::around_room())
                .set_parent(room_root.eid());

            // Get all the placements
            let bot_left = -(IDEAL_VEC_f32 / 2.0 - Vec2::ONE * 6.0);
            let top_right = -bot_left;
            let num_spawners = encounter_state.difficulty + 2;
            let num_enemies = 6 * encounter_state.difficulty.pow(2) as usize;
            let bird_placements = vec![(Shape::Circle { radius: 7.0 }, Vec2::ZERO, 0.0)];
            let spawner_placements = generate_circles(
                num_spawners,
                bot_left,
                top_right,
                (10.0, 10.1),
                (0.0, 0.1),
                0.0,
                &bird_placements,
            );
            let mut combined_avoid = bird_placements.clone();
            combined_avoid.extend(spawner_placements.clone().into_iter());
            let circle_placements = generate_circles(
                12,
                bot_left,
                top_right,
                (12.0, 32.0),
                (-5.0, 5.0),
                23.0,
                &combined_avoid,
            );

            // Spawn the bird!
            commands
                .spawn(BirdBundle::new(
                    bird_placements[0].1,
                    default(),
                    ephemeral_skills.get_num_launches(),
                    ephemeral_skills.get_num_bullets(),
                    num_enemies as u32,
                ))
                .set_parent(room_root.eid());

            // Calculate the batches and spawn the spawner
            let batch_size_range = 6..(6 + 3 * encounter_state.difficulty.pow(2) as usize);
            let mut batch_sizes = vec![];
            let mut unaccounted_for = num_enemies;
            while unaccounted_for > 0 {
                let batch_size = rand::thread_rng().gen_range(batch_size_range.clone());
                let batch_size = batch_size.min(unaccounted_for);
                batch_sizes.push(batch_size);
                unaccounted_for -= batch_size;
            }
            commands
                .spawn(EnemySpawnerBundle::<SimpBundle>::new(
                    spawner_placements.into_iter().map(|(_, b, _)| b).collect(),
                    batch_sizes,
                ))
                .set_parent(room_root.eid());

            // Spawn the circles
            for (ix, (shape, pos, rot)) in circle_placements.into_iter().enumerate() {
                commands
                    .spawn(StickyPlatformBundle::new(
                        format!("shape_{ix}").as_str(),
                        pos,
                        shape,
                    ))
                    .insert(DynoRot { rot })
                    .set_parent(room_root.eid());
            }
        }
        EncounterKind::PukebeakOnly => {
            // TODO: REMOVE THIS ONLY FOR TESTING
            ephemeral_skills.start_attempt(&permanent_skills);

            music_manager.fade_to_song(MusicKind::NormalBattle); // remember this does nothing if it's already this song

            // Background and room border
            BackgroundKind::Zenith.spawn(default(), room_root.eid(), &mut commands);
            commands
                .spawn(HardPlatformBundle::around_room())
                .set_parent(room_root.eid());

            commands
                .spawn(StickyPlatformBundle::new(
                    "shape",
                    Vec2::new(0.0, -40.0),
                    Shape::Circle { radius: 20.0 },
                ))
                .insert(DynoRot { rot: 0.0 })
                .set_parent(room_root.eid());

            // Spawn the bird!
            commands
                .spawn(BirdBundle::new(
                    Vec2::new(-100.0, 0.0),
                    default(),
                    ephemeral_skills.get_num_launches(),
                    ephemeral_skills.get_num_bullets(),
                    1000,
                ))
                .set_parent(room_root.eid());

            // Spawn the spawner
            commands
                .spawn(EnemySpawnerBundle::<SpewBundle>::new(
                    vec![Vec2::new(100.0, 0.0)],
                    vec![1, 1, 1, 1, 1],
                ))
                .set_parent(room_root.eid());
        }
    }
}

/// At one point I wanted to have a cool transition here but I guess not
/// NOTE: For some reason shit doesn't work if I try to do this state transition from the OnEnter(Entering). Idk why. Skill issue or bevy issue
fn update_encounter_enter(
    encounter_state: Res<State<EncounterState>>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
) {
    next_meta_state.set(
        RoomState::Encounter(EncounterState {
            progress: EncounterProgress::Fighting,
            kind: encounter_state.get().kind,
            difficulty: encounter_state.get().difficulty,
        })
        .to_meta_state(),
    );
}

fn update_encounter_fighting(
    encounter_state: Res<State<EncounterState>>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
    bird: Query<&mut Bird>,
) {
    let Ok(bird) = bird.get_single() else {
        return;
    };
    if bird.get_kills_left() == 0 {
        next_meta_state.set(
            RoomState::Encounter(EncounterState {
                progress: EncounterProgress::Meandering,
                kind: encounter_state.get().kind,
                difficulty: encounter_state.get().difficulty,
            })
            .to_meta_state(),
        );
        return;
    }
}

fn enter_meandering(
    mut commands: Commands,
    mut music_manager: ResMut<MusicManager>,
    steelbeak_spawners: Query<(Entity, &EnemySpawner<SimpBundle>)>,
    room_root: Res<RoomRoot>,
) {
    commands.spawn(SoundEffect::universal("sound_effects/room_clear.ogg", 0.3));
    music_manager.fade_to_song(MusicKind::SandCastles);

    let mut possible_heart_poses = vec![];
    for (eid, spawner) in &steelbeak_spawners {
        possible_heart_poses.extend(spawner.poses.clone().into_iter());
        commands.entity(eid).despawn_recursive();
    }

    for pos in possible_heart_poses {
        if thread_rng().gen::<f32>() < 0.2 {
            commands
                .spawn(HeartBundle::new(pos))
                .set_parent(room_root.eid());
        }
    }

    commands
        .spawn(GoNextBundle::new(Vec2::ZERO))
        .set_parent(room_root.eid());
}

pub(super) fn register_encounters(app: &mut App) {
    app.add_systems(OnEnter(EncounterProgress::Entering), create_room);
    app.add_systems(
        Update,
        update_encounter_enter.run_if(in_state(EncounterProgress::Entering)),
    );
    app.add_systems(
        Update,
        update_encounter_fighting.run_if(in_state(EncounterProgress::Fighting)),
    );
    app.add_systems(OnEnter(EncounterProgress::Meandering), enter_meandering);
}
