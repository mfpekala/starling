use crate::prelude::*;

mod encounter;

pub(super) struct RoomPlugin;
impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        encounter::register_encounters(app);
    }
}
