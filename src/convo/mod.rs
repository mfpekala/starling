use crate::prelude::*;

pub mod components;

pub use components::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect, States)]
pub enum ConvoState {
    None,
    EggUnwrap,
}

pub(super) struct ConvoPlugin;
impl Plugin for ConvoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ConvoState::None);
    }
}
