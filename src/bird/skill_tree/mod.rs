use crate::prelude::*;

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
    max_health: u32,
}
impl Default for EphemeralSkill {
    fn default() -> Self {
        Self {
            num_launches: 0,
            num_bullets: 0,
            max_health: 3,
        }
    }
}
impl EphemeralSkill {
    /// Called at the beginning of an attempt to reset ephemeral skill values
    pub fn start_attempt(&mut self, permanent: &PermanentSkill) {
        self.num_launches = permanent.get_num_launches();
        self.num_bullets = permanent.get_num_bullets();
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
        self.max_health += amt;
    }
}

pub(super) struct SkillTreePlugin;
impl Plugin for SkillTreePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PermanentSkill::default());
        app.insert_resource(EphemeralSkill::default());
    }
}
