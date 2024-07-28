use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum EncounterKind {
    SteelbeakOnly,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum EncounterProgress {
    Entering,
    Fighting,
    Meandering,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub struct EncounterState {
    kind: EncounterKind,
    difficulty: u32,
    progress: EncounterProgress,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum RoomState {
    Encounter(EncounterState),
}
impl RoomState {
    pub fn xth_encounter(kind: EncounterKind, difficulty: u32) -> Self {
        Self::Encounter(EncounterState {
            kind,
            difficulty,
            progress: EncounterProgress::Entering,
        })
    }
}

impl ComputedStates for EncounterKind {
    type SourceStates = MetaState;

    fn compute(sources: MetaState) -> Option<Self> {
        match sources.get_room_state() {
            Some(room_state) => match room_state {
                RoomState::Encounter(encounter_state) => Some(encounter_state.kind),
            },
            None => None,
        }
    }
}

impl ComputedStates for EncounterProgress {
    type SourceStates = MetaState;

    fn compute(sources: MetaState) -> Option<Self> {
        match sources.get_room_state() {
            Some(room_state) => match room_state {
                RoomState::Encounter(encounter_state) => Some(encounter_state.progress),
            },
            None => None,
        }
    }
}

pub(super) fn register_room_states(app: &mut App) {
    app.add_computed_state::<EncounterKind>();
    app.add_computed_state::<EncounterProgress>();
}
