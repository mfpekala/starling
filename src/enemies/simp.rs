use crate::prelude::*;

#[derive(Component)]
pub struct SimpBody {
    prefer_left: bool,
}

#[derive(Component)]
pub struct SimpVision {
    prefer_future: f32,
}

#[derive(Bundle)]
pub struct SimpBundle {
    name: Name,
    simp: SimpBody,
    face_dyno: FaceDyno,
    physics: SimpBodyPhysicsBundle,
    multi: MultiAnimationManager,
}
impl SimpBundle {
    pub fn spawn(pos: Vec2, commands: &mut Commands, parent: Entity) {
        let static_radius = 16.0;
        let trigger_radius = 8.0;
        commands
            .spawn(Self {
                name: Name::new("simp"),
                simp: SimpBody { prefer_left: true },
                face_dyno: FaceDyno,
                physics: SimpBodyPhysicsBundle::new(pos, static_radius),
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
                    SimpVision { prefer_future: 0.0 },
                    SimpVisionPhysicsBundle::new(trigger_radius),
                ));
            })
            .set_parent(parent);
    }
}

/// EPIC function name
fn simp_vision(
    bird: Query<(&GlobalTransform, &DynoTran), With<Bird>>,
    mut simp_bodys: Query<(&SimpBody, &mut DynoTran, &GlobalTransform, &Children), Without<Bird>>,
    mut simp_visions: Query<(&SimpVision, &mut Transform, &TriggerReceiver)>,
) {
    let Ok((bird_gtran, bird_dyno_tran)) = bird.get_single() else {
        return;
    };
    for (simp_body, mut simp_dyno_tran, simp_body_gtran, simp_children) in &mut simp_bodys {
        let (simp_vision, mut simp_vision_tran, simp_vision_rx) = simp_visions
            .get_mut(*simp_children.iter().next().unwrap())
            .unwrap();
        println!("collisions: {:?}", simp_vision_rx.collisions);
        let diff = bird_gtran.translation().truncate() - simp_body_gtran.translation().truncate();
        simp_vision_tran.set_angle(diff.to_angle());
        simp_dyno_tran.vel = diff.normalize_or_zero() * 20.0;
    }
}

pub(super) fn register_simps(app: &mut App) {
    app.add_systems(
        Update,
        simp_vision
            .run_if(in_state(PhysicsState::Active))
            .after(PhysicsSet),
    );
}
