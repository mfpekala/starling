use crate::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub mod transition;
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
    LearnFlight,
    LearnShooting,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum EncounterState {
    Entering,
    Fighting,
    Meandering,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
pub enum RoomState {
    Encounter(EncounterState),
    SkillUp,
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
        transition: transition::Transition,
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
impl MetaUnfucker for State<MetaState> {
    fn get_menu_state(&self) -> Option<MenuState> {
        match self.get() {
            MetaState::Menu(menu_state) => Some(menu_state.clone()),
            _ => None,
        }
    }

    fn get_cutscene_state(&self) -> Option<CutsceneState> {
        match self.get() {
            MetaState::Cutscene(cutscene_state) => Some(cutscene_state.clone()),
            _ => None,
        }
    }

    fn get_tutorial_state(&self) -> Option<TutorialState> {
        match self.get() {
            MetaState::Tutorial(tutorial_state) => Some(tutorial_state.clone()),
            _ => None,
        }
    }

    fn get_room_state(&self) -> Option<RoomState> {
        match self.get() {
            MetaState::Room(room_state) => Some(room_state.clone()),
            _ => None,
        }
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
    type SourceStates = (MetaState, PauseState);

    fn compute(sources: (MetaState, PauseState)) -> Option<Self> {
        // Here we convert from our [`AppState`] to all potential [`IsPaused`] versions.
        match sources {
            (MetaState::Menu(_), _) | (MetaState::Cutscene(_), _) => Some(Self::Inactive),
            (_, PauseState::Paused) => Some(Self::Inactive),
            (MetaState::Tutorial(_), PauseState::Unpaused) => Some(Self::Active),
            (MetaState::Room(_), PauseState::Unpaused) => Some(Self::Active),
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
        // app.insert_state(MetaState::Menu(MenuState::Studio));
        app.insert_state(MetaState::Tutorial(TutorialState::LearnFlight));
        app.insert_state(MetaTransitionState::Stable);
        app.insert_state(PauseState::Unpaused);
        app.insert_state(AppMode::Dev);
        // Computed states
        app.add_computed_state::<PhysicsState>();
        // Transitions
        transition::register_transition(app);
        // Debug
        app.add_plugins(ResourceInspectorPlugin::<State<MetaState>>::new());
        app.add_plugins(ResourceInspectorPlugin::<State<PauseState>>::new());
    }
}