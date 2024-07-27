use crate::prelude::*;

#[derive(Component, Debug, Reflect)]
pub struct FlySpot {
    pub key: String,
}
#[derive(Bundle)]
pub struct FlySpotBundle {
    name: Name,
    fly_spot: FlySpot,
    trigger: TutorialTriggerPhysicsBundle,
    multi: MultiAnimationManager,
}
impl FlySpotBundle {
    pub fn new(pos: Vec2, radius: f32, key: &str) -> Self {
        let trigger = TutorialTriggerPhysicsBundle::new(pos, radius, key.to_string());
        let anim_points = trigger.bounds.get_shape().to_anim_points();
        Self {
            name: Name::new(format!("spot_{key}")),
            fly_spot: FlySpot {
                key: key.to_string(),
            },
            trigger,
            multi: multi!([
                (
                    "core",
                    anim_man!({
                        path: "tutorial/shiny_spot_core.png",
                        size: (6, 6),
                        length: 5,
                        fps: 12.0,
                    })
                    .with_points(anim_points.clone()),
                ),
                (
                    "light",
                    anim_man!({
                        path: "tutorial/shiny_spot_light.png",
                        size: (6, 6),
                    })
                    .with_points(anim_points)
                    .with_render_layers(LightCamera::render_layers()),
                ),
            ]),
        }
    }
}
