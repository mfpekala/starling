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
    pub kind: EncounterKind,
    pub difficulty: u32,
    pub progress: EncounterProgress,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum RoomState {
    Encounter(EncounterState),
    Dead,
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
                _ => None,
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
                _ => None,
            },
            None => None,
        }
    }
}

impl ComputedStates for EncounterState {
    type SourceStates = MetaState;

    fn compute(sources: MetaState) -> Option<Self> {
        match sources.get_room_state() {
            Some(room_state) => match room_state {
                RoomState::Encounter(encounter_state) => Some(encounter_state),
                _ => None,
            },
            None => None,
        }
    }
}

pub(super) fn register_room_states(app: &mut App) {
    app.add_computed_state::<EncounterKind>();
    app.add_computed_state::<EncounterProgress>();
    app.add_computed_state::<EncounterState>();
}
