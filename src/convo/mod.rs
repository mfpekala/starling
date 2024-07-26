use crate::prelude::*;

pub(self) mod components;
mod data;
mod operation;

use components::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect, States)]
pub enum ConvoState {
    None,
    TutorialEggUnwrap,
    TutorialLaunchChallengeStart,
    TutorialLaunchSlowMotionRemark,
    TutorialLaunchExhaustedWarning,
    TutorialLaunchFlightWarning,
    TutorialLaunchChallengeCompleted,
}

fn setup_convo(
    mut commands: Commands,
    convo_state: Res<State<ConvoState>>,
    convo_root: Res<ConvoRoot>,
) {
    let state = convo_state.get().clone();
    let box_data = data::get_full_convo(state)
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
    commands
        .spawn(ConvoBundle {
            name: Name::new(format!("convo_{:?}", state)),
            convo: Convo {
                state,
                active_box_eid: None,
                bundles: box_data,
            },
            spatial: default(),
        })
        .set_parent(convo_root.eid());
}

fn destroy_convo(mut commands: Commands, convo_root: Res<ConvoRoot>) {
    commands.entity(convo_root.eid()).despawn_descendants();
}

pub(super) struct ConvoPlugin;
impl Plugin for ConvoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ConvoState::None);
        app.add_systems(OnExit(ConvoState::None), setup_convo);
        app.add_systems(OnEnter(ConvoState::None), destroy_convo);

        operation::register_operation(app);

        // Typesssss
        app.register_type::<Convo>();
        app.register_type::<BoxContent>();
        app.register_type::<BoxPos>();
        app.register_type::<Progress>();
        app.register_type::<BoxSpeaker>();
    }
}
