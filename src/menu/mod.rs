use crate::prelude::*;

mod studio;
mod title;

pub(super) struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        studio::register_studio(app);
        title::register_title(app);
    }
}
