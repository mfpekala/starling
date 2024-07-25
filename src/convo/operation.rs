use super::components::*;
use crate::prelude::*;

/// Basically just checks if the current box is done. If so, spawns the next one and despawns the current one.
/// If there are no more boxes to spawn, despawns the convo and sends the finished event.
fn update_convo(
    mut commands: Commands,
    convo_root: Res<ConvoRoot>,
    mut convo: Query<(Entity, &mut Convo)>,
    box_q: Query<(Entity, &Progress)>,
    mut next_convo_state: ResMut<NextState<ConvoState>>,
) {
    let (cid, mut convo) = convo.single_mut();

    let bx = convo.active_box_eid.map(|eid| box_q.get(eid).unwrap());

    // Figure out if we can spawn. Also has side effect of despawining the current
    // one if it's absolutely finished.
    let can_spawn = match bx {
        Some((bid, bx)) => {
            if bx.absolutely_finished {
                commands.entity(bid).despawn_recursive();
            }
            bx.absolutely_finished
        }
        None => true,
    };

    if can_spawn {
        match convo.bundles.pop() {
            Some(the_box) => {
                // Time to have babies
                convo.active_box_eid = Some(the_box.do_spawn(&mut commands, convo_root.eid()));
            }
            None => {
                // Time to die (this will kill us next frame)
                next_convo_state.set(ConvoState::None);
            }
        }
    }
}

pub(super) fn register_operation(app: &mut App) {
    app.add_systems(Update, update_convo.run_if(not(in_state(ConvoState::None))));
}
