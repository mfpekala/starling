use crate::prelude::*;

mod dead;
mod encounter;

pub(super) struct RoomPlugin;
impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        dead::register_dead(app);
        encounter::register_encounters(app);
    }
}
