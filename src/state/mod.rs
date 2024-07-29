use crate::prelude::*;

pub mod room;
pub mod transition;

pub use room::*;
pub use transition::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum MenuState {
    Studio,
    Title,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum CutsceneState {
    StartAttempt,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum TutorialState {
    LearnToFly,
    LearnToShoot,
    ImpossibleBoss,
    Dead,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Reflect)]
pub enum MetaState {
    Menu(MenuState),
    Cutscene(CutsceneState),
    Tutorial(TutorialState),
    Room(RoomState),
}

/// The state that actually holds data about transitions
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Reflect)]
pub enum MetaTransitionState {
    Stable,
    Volatile {
        transition: transition::InternalTransition,
        next_state: MetaState,
    },
}

/// Kills some verbosity with reading meta states
pub trait MetaUnfucker {
    fn get_menu_state(&self) -> Option<MenuState>;
    fn get_cutscene_state(&self) -> Option<CutsceneState>;
    fn get_tutorial_state(&self) -> Option<TutorialState>;
    fn get_room_state(&self) -> Option<RoomState>;
}
impl MetaUnfucker for MetaState {
    fn get_menu_state(&self) -> Option<MenuState> {
        match self {
            MetaState::Menu(menu_state) => Some(menu_state.clone()),
            _ => None,
        }
    }

    fn get_cutscene_state(&self) -> Option<CutsceneState> {
        match self {
            MetaState::Cutscene(cutscene_state) => Some(cutscene_state.clone()),
            _ => None,
        }
    }

    fn get_tutorial_state(&self) -> Option<TutorialState> {
        match self {
            MetaState::Tutorial(tutorial_state) => Some(tutorial_state.clone()),
            _ => None,
        }
    }

    fn get_room_state(&self) -> Option<RoomState> {
        match self {
            MetaState::Room(room_state) => Some(room_state.clone()),
            _ => None,
        }
    }
}
impl MetaUnfucker for State<MetaState> {
    fn get_menu_state(&self) -> Option<MenuState> {
        MetaState::get_menu_state(self.get())
    }

    fn get_cutscene_state(&self) -> Option<CutsceneState> {
        MetaState::get_cutscene_state(self.get())
    }

    fn get_tutorial_state(&self) -> Option<TutorialState> {
        MetaState::get_tutorial_state(self.get())
    }

    fn get_room_state(&self) -> Option<RoomState> {
        MetaState::get_room_state(self.get())
    }
}

/// Kills some verbosity for writing meta states
pub trait ToMetaState {
    fn to_meta_state(&self) -> MetaState;
}
macro_rules! impl_to_meta_state {
    ($type:ty, $disc:ident) => {
        impl ToMetaState for $type {
            fn to_meta_state(&self) -> MetaState {
                MetaState::$disc(self.clone())
            }
        }
    };
}
impl_to_meta_state!(MenuState, Menu);
impl_to_meta_state!(CutsceneState, Cutscene);
impl_to_meta_state!(TutorialState, Tutorial);
impl_to_meta_state!(RoomState, Room);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Reflect)]
pub enum PauseState {
    Unpaused,
    Paused,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum PhysicsState {
    Inactive,
    Active,
}

impl ComputedStates for PhysicsState {
    type SourceStates = (MetaState, PauseState, ConvoState);

    fn compute(sources: (MetaState, PauseState, ConvoState)) -> Option<Self> {
        // Here we convert from our [`AppState`] to all potential [`IsPaused`] versions.
        match sources {
            (MetaState::Tutorial(_), PauseState::Unpaused, ConvoState::None) => Some(Self::Active),
            (MetaState::Room(_), PauseState::Unpaused, ConvoState::None) => Some(Self::Active),
            _ => Some(Self::Inactive),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect, States)]
pub enum AppMode {
    Dev,
    Prod,
}

pub(super) struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        // Ground truth states

        // app.insert_state(MetaState::Tutorial(TutorialState::LearnToFly)); // INITIAL_STATE (control f this silly)
        app.insert_state(MetaState::Menu(MenuState::Title)); // INITIAL STATE (control f this silly)
                                                             // app.insert_state(RoomState::xth_encounter(EncounterKind::PukebeakOnly, 1).to_meta_state()); // initial

        app.insert_state(MetaTransitionState::Stable);
        app.insert_state(PauseState::Unpaused);
        app.insert_state(AppMode::Dev);
        // Computed states
        app.add_computed_state::<PhysicsState>();
        // Transitions
        transition::register_transition(app);
        // Rooms
        room::register_room_states(app);
    }
}
