use crate::prelude::*;

fn setup_dead(mut next_convo_state: ResMut<NextState<ConvoState>>) {
    next_convo_state.set(ConvoState::TutorialHelloDeath);
}

fn drop_egg(mut commands: Commands, tutorial_root: Res<TutorialRoot>) {
    commands
        .spawn(EggBundle::new(Vec2::new(-30.0, 120.0)))
        .set_parent(tutorial_root.eid());
}

fn update(
    mut falling_egg: Query<(Entity, &mut Transform, Option<&Stuck>), With<Egg>>,
    mut commands: Commands,
) {
    if let Ok((egg_id, mut egg_tran, stuck)) = falling_egg.get_single_mut() {
        if egg_tran.translation.y < 10.0 {
            commands
                .entity(egg_id)
                .insert(StaticReceiver::from_kind(StaticReceiverKind::Normal));
        }
        if egg_tran.translation.y < -400.0 {
            // Just in case
            egg_tran.translation.y = 0.0;
        }
        if stuck.is_some() {
            println!("lets goo")
        }
    }
}

pub(super) fn register_dead(app: &mut App) {
    app.add_systems(OnEnter(TutorialState::Dead.to_meta_state()), setup_dead);
    app.add_systems(OnExit(ConvoState::TutorialHelloDeath), drop_egg);
    app.add_systems(
        Update,
        update.run_if(in_state(TutorialState::Dead.to_meta_state())),
    );
}
