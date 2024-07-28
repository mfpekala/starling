use crate::prelude::*;
use rand::Rng;

/// This is the system responsible for randomly generating each room.
/// First it generates the spinning wheels.
/// Then it places the spawners.
fn create_room(
    mut commands: Commands,
    room_root: Res<RoomRoot>,
    mut music_manager: ResMut<MusicManager>,
) {
    // Clear the room just to be sure
    commands.entity(room_root.eid()).despawn_descendants();
    music_manager.fade_to_song(MusicKind::NormalBattle); // remember this does nothing if it's already this song

    // Background and room border
    BackgroundKind::Zenith.spawn(default(), room_root.eid(), &mut commands);
    commands
        .spawn(HardPlatformBundle::around_room())
        .set_parent(room_root.eid());

    // Circles!!
    // let bot_left = -(IDEAL_VEC_f32 / 2.0 - Vec2::ONE * 6.0);
    // let top_right = -bot_left;
    // let rad_range = (12.0, 32.0);
    // let rot_range = (-5.0, 5.0);
    // let dist_between = 24.0;
    // let circles = generate_circles(9, bot_left, top_right, rad_range, rot_range, dist_between);
    // for (ix, (shape, pos, rot)) in circles.into_iter().enumerate() {
    //     commands
    //         .spawn(StickyPlatformBundle::new(
    //             format!("shape_{ix}").as_str(),
    //             pos,
    //             shape,
    //         ))
    //         .insert(DynoRot { rot })
    //         .set_parent(room_root.eid());
    // }

    commands
        .spawn(EnemySpawnerBundle::<SimpBundle>::new(
            default(),
            vec![3, 3, 3],
        ))
        .set_parent(room_root.eid());
}

pub(super) fn register_encounters(app: &mut App) {
    app.add_systems(OnEnter(EncounterProgress::Entering), create_room);
}

/// ChatGpt for the... mediocre code?
pub fn generate_circles(
    num_circles: u32,
    bot_left: Vec2,
    top_right: Vec2,
    rad_range: (f32, f32),
    rot_range: (f32, f32),
    dist_between: f32,
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

            if is_valid(&center, radius, &result, bot_left, top_right, dist_between) {
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
    for (shape, other_center, _) in circles.iter() {
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
