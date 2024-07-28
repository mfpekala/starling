use crate::prelude::*;

#[derive(Component)]
struct ProgressBar;
impl ProgressBar {
    const DIMS: Vec2 = Vec2::new(60.0, 4.0);
    const BORDER: f32 = 1.0;
}

fn spawn_progress_bar(mut commands: Commands, room_root: Res<RoomRoot>) {
    commands
        .spawn((
            Name::new("progress_bar"),
            ProgressBar,
            multi!([
                (
                    "bg",
                    anim_man!({
                        path: "sprites/default.png",
                        size: (1, 1),
                        color: Color::BLACK,
                    })
                    .with_points(simple_rect(
                        ProgressBar::DIMS.x + ProgressBar::BORDER * 2.0,
                        ProgressBar::DIMS.y + ProgressBar::BORDER * 2.0
                    ))
                    .with_offset(-Vec3::Z),
                ),
                (
                    "fg",
                    anim_man!({
                        path: "sprites/default.png",
                        size: (1, 1),
                        color: Color::srgb(0.7, 0.8, 0.0),
                    })
                    .with_points(simple_rect(ProgressBar::DIMS.x, ProgressBar::DIMS.y)),
                )(
                    "light",
                    anim_man!({
                        path: "sprites/default.png",
                        size: (1, 1),
                    })
                    .with_render_layers(LightCamera::render_layers())
                    .with_points(simple_rect(
                        ProgressBar::DIMS.x + ProgressBar::BORDER * 2.0,
                        ProgressBar::DIMS.y + ProgressBar::BORDER * 2.0
                    )),
                )
            ]),
            spat_tran(119.0, -77.0, ZIX_MAX - 0.1),
        ))
        .set_parent(room_root.eid());
}

fn destroy_progress_bar(eids: Query<Entity, With<ProgressBar>>, mut commands: Commands) {
    for eid in &eids {
        commands.entity(eid).despawn_recursive();
    }
}

fn update_progress_bar(
    bird: Query<&Bird>,
    mut multi: Query<&mut MultiAnimationManager, With<ProgressBar>>,
    mut commands: Commands,
    skills: Res<EphemeralSkill>,
) {
    let Ok(bird) = bird.get_single() else {
        return;
    };
    let Ok(mut multi) = multi.get_single_mut() else {
        return;
    };
    let frac_complete =
        (bird.total_kills_this_room - bird.kills_left) as f32 / (bird.total_kills_this_room as f32);
    // It looks too sad if it's totally empty, always show at least 5% complete
    let frac_complete = frac_complete.max(0.05);
    let new_points = simple_rect(ProgressBar::DIMS.x * frac_complete, ProgressBar::DIMS.y)
        .into_iter()
        .map(|mut p| {
            // First shift so it's centered on my left edge
            p.x += ProgressBar::DIMS.x * frac_complete / 2.0;
            // Then shift left to align left edge where expected
            p.x -= ProgressBar::DIMS.x / 2.0;
            // Could I combine these? Yes. My brain hurts tho.
            p
        })
        .collect::<Vec<_>>();
    multi
        .manager_mut("fg")
        .set_points(new_points, &mut commands);
    // NASTY HACK INCOMING
    for anim in multi.map.values_mut() {
        anim.set_hidden(skills.get_current_health() == 0, &mut commands);
    }
}

pub(super) fn register_progress_bar(app: &mut App) {
    app.add_systems(OnEnter(EncounterProgress::Fighting), spawn_progress_bar);
    app.add_systems(OnExit(EncounterProgress::Fighting), destroy_progress_bar);
    app.add_systems(
        Update,
        update_progress_bar.run_if(in_state(EncounterProgress::Fighting)),
    );
}
