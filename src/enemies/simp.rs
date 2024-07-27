use rand::{thread_rng, Rng};

use crate::prelude::*;

#[derive(Component)]
pub struct SimpGuide {
    speed: f32,
    prefer_future: f32,
}

#[derive(Component)]
pub struct SimpHurtbox;

#[derive(Bundle)]
pub struct SimpBundle {
    name: Name,
    simp: SimpGuide,
    face_dyno: FaceDyno,
    physics: SimpGuidePhysicsBundle,
    multi: MultiAnimationManager,
}
impl SimpBundle {
    const STATIC_RADIUS: f32 = 10.0;
    const TRIGGER_RADIUS: f32 = 8.0;
    const SPEED_RANGE: (f32, f32) = (30.0, 50.0);
    const MULT_RANGE: (i32, i32) = (-40, 40);
    const FUTURE_RANGE: (f32, f32) = (-0.5, 2.0);

    pub fn spawn(pos: Vec2, commands: &mut Commands, parent: Entity) {
        let mut rng = thread_rng();
        let speed = rng.gen_range(Self::SPEED_RANGE.0..Self::SPEED_RANGE.1);
        let mult = rng.gen_range(Self::MULT_RANGE.0..Self::MULT_RANGE.1);
        let fut = rng.gen_range(Self::FUTURE_RANGE.0..Self::FUTURE_RANGE.1);
        commands
            .spawn(Self {
                name: Name::new("simp"),
                simp: SimpGuide {
                    speed,
                    prefer_future: fut,
                },
                face_dyno: FaceDyno,
                physics: SimpGuidePhysicsBundle::new(pos, Self::STATIC_RADIUS, mult),
                multi: multi!([
                    (
                        "core",
                        anim_man!({
                            path: "enemies/simp.png",
                            size: (20, 20),
                        })
                    ),
                    (
                        "light",
                        anim_man!({
                            path: "enemies/simp_light.png",
                            size: (30, 30),
                        })
                        .with_render_layers(LightCamera::render_layers())
                    )
                ]),
            })
            .with_children(|dad| {
                dad.spawn((
                    Name::new("vision"),
                    SimpHurtbox,
                    SimpHurtboxPhysicsBundle::new(Self::TRIGGER_RADIUS),
                ));
            })
            .set_parent(parent);
    }
}

fn guide_simps(
    bird: Query<(&GlobalTransform, &DynoTran), With<Bird>>,
    mut simp_guides: Query<(&SimpGuide, &mut DynoTran, &GlobalTransform), Without<Bird>>,
    time: Res<Time>,
    bullet_time: Res<BulletTime>,
) {
    let Ok((bird_gtran, bird_dyno_tran)) = bird.get_single() else {
        return;
    };
    let time_factor = time.delta_seconds() * bullet_time.factor();
    for (simp_guide, mut simp_dyno_tran, simp_body_gtran) in &mut simp_guides {
        let goal_bird_pos =
            bird_gtran.translation().truncate() + bird_dyno_tran.vel * simp_guide.prefer_future;
        let diff = goal_bird_pos - simp_body_gtran.translation().truncate();
        simp_dyno_tran.vel += diff.normalize_or_zero() * 100.0 * time_factor;
        simp_dyno_tran.vel = simp_dyno_tran.vel.clamp_length(0.0, simp_guide.speed);
    }
}

pub(super) fn register_simps(app: &mut App) {
    app.add_systems(
        Update,
        guide_simps
            .run_if(in_state(PhysicsState::Active))
            .after(PhysicsSet),
    );
}
