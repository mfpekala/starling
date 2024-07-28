use upgrade_button::{color_upgrade_buttons, update_upgrade_buttons};

use crate::prelude::*;

pub mod upgrade_button;

pub use upgrade_button::*;

/// The skills that persist through attempts
#[derive(Resource)]
pub struct PermanentSkill {
    num_launches: u32,
    num_bullets: u32,
    max_health: u32,
}
impl Default for PermanentSkill {
    fn default() -> Self {
        Self {
            num_launches: 0,
            num_bullets: 0,
            max_health: 3,
        }
    }
}
impl PermanentSkill {
    pub fn get_num_launches(&self) -> u32 {
        self.num_launches
    }

    pub fn get_num_bullets(&self) -> u32 {
        self.num_bullets
    }

    pub fn get_max_health(&self) -> u32 {
        self.max_health
    }

    pub fn increase_num_launches(&mut self, amt: u32) {
        self.num_launches += amt;
    }

    pub fn increase_num_bullets(&mut self, amt: u32) {
        self.num_bullets += amt;
    }

    pub fn increase_max_health(&mut self, amt: u32) {
        self.max_health += amt;
    }

    pub fn force_set_num_launches(&mut self, val: u32) {
        self.num_launches = val;
    }

    pub fn force_set_num_bullets(&mut self, val: u32) {
        self.num_bullets = val;
    }

    pub fn force_set_max_health(&mut self, val: u32) {
        self.max_health = val;
    }
}

/// The skills that get reset at the beginning of each attempt
#[derive(Resource)]
pub struct EphemeralSkill {
    num_launches: u32,
    num_bullets: u32,
    /// Skill issue on my part. I shouldn't have put health on the bird bundle.
    /// Oh well, this is fine I think.
    current_health: u32,
    max_health: u32,
}
impl Default for EphemeralSkill {
    fn default() -> Self {
        Self {
            num_launches: 0,
            num_bullets: 0,
            max_health: 3,
            current_health: 3,
        }
    }
}
impl EphemeralSkill {
    /// Called at the beginning of an attempt to reset ephemeral skill values
    pub fn start_attempt(&mut self, permanent: &PermanentSkill) {
        self.num_launches = permanent.get_num_launches();
        self.num_bullets = permanent.get_num_bullets();
        self.current_health = permanent.get_max_health();
        self.max_health = permanent.get_max_health();
    }

    pub fn get_num_launches(&self) -> u32 {
        self.num_launches
    }

    pub fn get_num_bullets(&self) -> u32 {
        self.num_bullets
    }

    pub fn get_max_health(&self) -> u32 {
        self.max_health
    }

    pub fn increase_num_launches(&mut self, amt: u32) {
        self.num_launches += amt;
    }

    pub fn increase_num_bullets(&mut self, amt: u32) {
        self.num_bullets += amt;
    }

    pub fn increase_max_health(&mut self, amt: u32) {
        self.current_health += amt;
        self.max_health += amt;
    }

    pub fn get_current_health(&self) -> u32 {
        self.current_health
    }

    pub fn set_current_health(&mut self, val: u32) {
        self.current_health = val;
    }

    pub fn inc_current_health(&mut self, amt: u32) {
        self.current_health = self.max_health.min(self.current_health + amt);
    }

    pub fn dec_current_health(&mut self, amt: u32) {
        self.current_health = self.current_health.saturating_sub(amt);
    }
}

pub(super) struct SkillTreePlugin;
impl Plugin for SkillTreePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PermanentSkill::default());
        app.insert_resource(EphemeralSkill::default());

        app.add_systems(Update, (update_upgrade_buttons, color_upgrade_buttons));
    }
}
