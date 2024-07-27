use crate::prelude::*;

#[derive(Resource, Reflect)]
pub struct BirdFlightConsts {
    drag: f32,
    fast_stop_drag: f32,
    hor_mul: f32,
    down_mul: f32,
    up_mul: f32,
    slow_down_mul: f32,
    max_hor_speed: f32,
    max_up_speed: f32,
    max_down_speed: f32,
}
impl Default for BirdFlightConsts {
    fn default() -> Self {
        Self {
            // Vel multiplied by this every frame
            drag: 0.99,
            // When pressing space (fast stop) what is the drag?
            fast_stop_drag: 0.9,
            // How quickly we speed up horizontally
            hor_mul: 125.0,
            // How quickly we speed up down
            down_mul: 100.0,
            // How quickly we speed up down
            up_mul: 800.0,
            // Extra boost to speed when slowing down Above max speed, helps us come to a stop after launching
            slow_down_mul: 6.0,
            // Max speeds
            max_down_speed: 240.0,
            max_hor_speed: 80.0,
            max_up_speed: 80.0,
        }
    }
}
impl BirdFlightConsts {
    fn apply(&self, dir: Vec2) -> Vec2 {
        let x = dir.x * self.hor_mul;
        let y = if dir.y > 0.0 {
            dir.y * self.up_mul
        } else {
            dir.y * self.down_mul
        };
        Vec2::new(x, y)
    }
}

pub(super) fn flying(
    mut bird_q: Query<(Entity, &mut DynoTran, &mut Transform), With<Bird>>,
    movement: Res<MovementInput>,
    mut commands: Commands,
    flight_consts: Res<BirdFlightConsts>,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
) {
    let Ok((eid, mut dyno_tran, mut tran)) = bird_q.get_single_mut() else {
        return;
    };
    let time_factor = time.delta_seconds() * bullet_time.factor();
    let vel_nudge = flight_consts.apply(movement.get_dir()) * time_factor;
    if movement.get_dir().length_squared() > 0.0 {
        tran.set_angle(0.0);
        commands.entity(eid).remove::<Stuck>();
        // let new_vel = dyno_tran.vel + vel_nudge;
        // Easiest to comprehend if I just bash cases even though it's verbose
        if vel_nudge.x > 0.0 {
            if dyno_tran.vel.x < -flight_consts.max_hor_speed {
                // We are slowing down from above max speed
                dyno_tran.vel.x += vel_nudge.x * flight_consts.slow_down_mul;
            } else if dyno_tran.vel.x < flight_consts.max_hor_speed {
                // We are speeding up to the right, below max speed in both directions
                dyno_tran.vel.x += vel_nudge.x;
            }
        }
        if vel_nudge.x < 0.0 {
            if dyno_tran.vel.x > flight_consts.max_hor_speed {
                // We are slowing down from above max speed
                dyno_tran.vel.x += vel_nudge.x * flight_consts.slow_down_mul;
            } else if dyno_tran.vel.x > -flight_consts.max_hor_speed {
                // We are speeding up to the left, below max speed in both directions
                dyno_tran.vel.x += vel_nudge.x;
            }
        }

        if vel_nudge.y > 0.0 {
            if dyno_tran.vel.y < -flight_consts.max_down_speed {
                // We are slowing down from above max speed
                dyno_tran.vel.y += vel_nudge.y * flight_consts.slow_down_mul;
            } else if dyno_tran.vel.y < flight_consts.max_up_speed {
                // We are speeding up to the up, below max speed in both directions
                dyno_tran.vel.y += vel_nudge.y;
            }
        }
        if vel_nudge.y < 0.0 {
            if dyno_tran.vel.y > flight_consts.max_up_speed {
                // We are slowing down from above max speed
                dyno_tran.vel.y += vel_nudge.y * flight_consts.slow_down_mul;
            } else if dyno_tran.vel.y > -flight_consts.max_down_speed {
                // We are speeding up to the down, below max speed in both directions
                dyno_tran.vel.y += vel_nudge.y;
            }
        }
    }
    dyno_tran.vel *= if movement.get_fast_stop() {
        flight_consts.fast_stop_drag
    } else {
        if matches!(*bullet_time, BulletTime::Inactive) {
            // Drag during bullet time is weird
            flight_consts.drag
        } else {
            1.0
        }
    };
}
