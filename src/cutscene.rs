use crate::prelude::*;

// fuck it just making a resource
#[derive(Resource, Default)]
struct FallData {
    time_since_start: Stopwatch,
    time_since_fall: Stopwatch,
}

#[derive(Component)]
struct ShrinkParent;

#[derive(Component)]
struct PlacedEgg;

#[derive(Component)]
struct PlacedForest;

fn enter_cutscene(mut commands: Commands, croot: Res<CutsceneRoot>) {
    let shrink_id = commands
        .spawn((
            Name::new("shrink_parent"),
            ShrinkParent,
            SpatialBundle {
                transform: Transform {
                    scale: (Vec2::ONE * 4.0).extend(1.0),
                    translation: Vec3::new(0.0, -96.0, 0.0),
                    ..default()
                },
                ..default()
            },
        ))
        .set_parent(croot.eid())
        .id();
    let forest_id = BackgroundKind::Forest.spawn(default(), shrink_id, &mut commands);
    commands.entity(forest_id).insert(PlacedForest);
    commands
        .spawn((
            Name::new("egg"),
            PlacedEgg,
            multi!([
                (
                    "core1",
                    anim_man!({
                        egg: {
                            path: "lenny/egg.png",
                            size: (24, 24),
                        },
                    })
                ),
                (
                    "light1",
                    anim_man!({
                        path: "lenny/spotlight.png",
                        size: (48, 48),
                    })
                    .with_render_layers(LightCamera::render_layers())
                    .with_scale(Vec2::new(2.0, 2.0))
                ),
            ]),
        ))
        .set_parent(shrink_id);
}

fn exit_cutscene(mut commands: Commands, croot: Res<CutsceneRoot>) {
    commands.entity(croot.eid()).despawn_descendants();
}

// fn update_cutscene(
//     mut data: ResMut<FallData>,
//     time: Res<Time>,
//     mut shrink: Query<&mut Transform, With<ShrinkParent>>,
// ) {
//     data.time_since_start.tick(time.delta());
//     if data.time_since_start.elapsed_secs() > 3.0 {
//         data.time_since_fall.tick(time.delta());
//     }
//     let mut shrink = shrink.single_mut();
//     shrink.scale = (Vec2::ONE
//         * Spleen::EaseInOutCubic.bound_interp(
//             data.time_since_start.elapsed_secs().min(3.0),
//             4.0,
//             max,
//         ))
//     .extend(1.0);
// }

pub(super) struct CutscenePlugin;
impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FallData::default());
        app.add_systems(OnEnter(CutsceneState::Fall.to_meta_state()), enter_cutscene);
        app.add_systems(OnExit(CutsceneState::Fall.to_meta_state()), exit_cutscene);
        // app.add_systems(
        //     Update,
        //     update_cutscene.run_if(in_state(CutsceneState::Fall.to_meta_state())),
        // );
    }
}
