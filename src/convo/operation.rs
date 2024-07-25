use std::time::Duration;

use super::components::*;
use crate::prelude::*;

/// Plays the text, watches for input to update the progress
fn update_box(
    time: Res<Time>,
    mut progress: Query<&mut Progress>,
    mouse_state: Res<MouseState>,
    mut content: Query<(&mut Text, &mut FullContent)>,
    stale: Query<Entity, With<ProgressStale>>,
    mut commands: Commands,
) {
    let (Ok(mut progress), Ok((mut text, fc))) =
        (progress.get_single_mut(), content.get_single_mut())
    else {
        return;
    };
    progress.timer.tick(time.delta());
    if mouse_state.buttons.just_pressed(MouseButton::Right) {
        if progress.timer.finished() {
            progress.absolutely_finished = true;
        } else {
            let amount_left = progress.timer.remaining_secs();
            progress
                .timer
                .tick(Duration::from_secs_f32(amount_left + 1.0));
        }
    }
    let frac_complete = progress.timer.fraction();
    let num_chars_showing = (fc.content.len() as f32 * frac_complete).ceil() as usize;
    let substring_showing = fc.content[0..num_chars_showing].to_string();
    text.sections[0].value = substring_showing;
    if progress.timer.finished() {
        for eid in &stale {
            commands.entity(eid).despawn_recursive();
        }
    }
}

/// Basically just checks if the current box is done. If so, spawns the next one and despawns the current one.
/// If there are no more boxes to spawn, despawns the convo and sends the finished event.
fn update_convo(
    mut commands: Commands,
    convo_root: Res<ConvoRoot>,
    mut convo: Query<&mut Convo>,
    box_q: Query<(Entity, &Progress, &Children)>,
    mut next_convo_state: ResMut<NextState<ConvoState>>,
) {
    let mut convo = convo.single_mut();

    let bx = convo.active_box_eid.map(|eid| box_q.get(eid).unwrap());

    // Figure out if we can spawn. Also has side effect of despawining the current
    // one if it's absolutely finished.
    let can_spawn = match bx {
        Some((bid, bx, children)) => {
            if bx.absolutely_finished {
                commands.entity(bid).remove::<Progress>();
                for child in children {
                    commands.entity(*child).remove::<FullContent>();
                    commands.entity(*child).remove::<Text>();
                }
                commands.entity(bid).insert(ProgressStale);
            }
            bx.absolutely_finished
        }
        None => true,
    };

    if can_spawn {
        match convo.bundles.pop() {
            Some(the_box) => {
                // Time to have babies
                convo.active_box_eid = Some(the_box.do_spawn(
                    -(convo.bundles.len() as f32),
                    &mut commands,
                    convo_root.eid(),
                ));
            }
            None => {
                // Time to die (this will kill us next frame)
                convo.active_box_eid = None;
                next_convo_state.set(ConvoState::None);
            }
        }
    }
}

pub(super) fn register_operation(app: &mut App) {
    app.add_systems(
        Update,
        (update_box, update_convo).run_if(not(in_state(ConvoState::None))),
    );
}
