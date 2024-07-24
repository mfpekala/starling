use crate::prelude::*;

use super::DebugState;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Reflect)]
struct ShowPhysicsBounds;
impl ComputedStates for ShowPhysicsBounds {
    type SourceStates = (AppMode, DebugState);

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        let (app_mode, debug_state) = sources;
        if matches!(app_mode, AppMode::Prod) {
            return None;
        }
        if debug_state.show_physics_bounds {
            Some(Self)
        } else {
            None
        }
    }
}

fn startup_debug(mut commands: Commands, room_root: Res<RoomRoot>) {
    commands.spawn(BirdBundle::new(default(), default(), 3, 3));
    let outer_width = 4.0;
    commands
        .spawn((
            Name::new("physics_debug_sticky1"),
            StickyPlainPhysicsBundle::new(
                Vec2::new(0.0, IDEAL_HEIGHT_f32 - outer_width) / 2.0,
                Bounds::from_shape(Shape::Polygon {
                    points: vec![
                        Vec2::new(-IDEAL_WIDTH_f32, -outer_width) / 2.0,
                        Vec2::new(-IDEAL_WIDTH_f32, outer_width) / 2.0,
                        Vec2::new(IDEAL_WIDTH_f32, outer_width) / 2.0,
                        Vec2::new(IDEAL_WIDTH_f32, -outer_width) / 2.0,
                    ],
                }),
            ),
        ))
        .set_parent(room_root.eid());
    commands
        .spawn((
            Name::new("physics_debug_sticky2"),
            StickyPlainPhysicsBundle::new(
                Vec2::new(0.0, -IDEAL_HEIGHT_f32 + outer_width) / 2.0,
                Bounds::from_shape(Shape::Polygon {
                    points: vec![
                        Vec2::new(-IDEAL_WIDTH_f32, -outer_width) / 2.0,
                        Vec2::new(-IDEAL_WIDTH_f32, outer_width) / 2.0,
                        Vec2::new(IDEAL_WIDTH_f32, outer_width) / 2.0,
                        Vec2::new(IDEAL_WIDTH_f32, -outer_width) / 2.0,
                    ],
                }),
            ),
        ))
        .set_parent(room_root.eid());
    commands
        .spawn((
            Name::new("physics_debug_sticky3"),
            StickyPlainPhysicsBundle::new(
                Vec2::new(-IDEAL_WIDTH_f32 + outer_width, 0.0) / 2.0,
                Bounds::from_shape(Shape::Polygon {
                    points: vec![
                        Vec2::new(-outer_width, -IDEAL_HEIGHT_f32) / 2.0,
                        Vec2::new(-outer_width, IDEAL_HEIGHT_f32) / 2.0,
                        Vec2::new(outer_width, IDEAL_HEIGHT_f32) / 2.0,
                        Vec2::new(outer_width, -IDEAL_HEIGHT_f32) / 2.0,
                    ],
                }),
            ),
        ))
        .set_parent(room_root.eid());
    commands
        .spawn((
            Name::new("physics_debug_sticky4"),
            StickyPlainPhysicsBundle::new(
                Vec2::new(IDEAL_WIDTH_f32 - outer_width, 0.0) / 2.0,
                Bounds::from_shape(Shape::Polygon {
                    points: vec![
                        Vec2::new(-outer_width, -IDEAL_HEIGHT_f32) / 2.0,
                        Vec2::new(-outer_width, IDEAL_HEIGHT_f32) / 2.0,
                        Vec2::new(outer_width, IDEAL_HEIGHT_f32) / 2.0,
                        Vec2::new(outer_width, -IDEAL_HEIGHT_f32) / 2.0,
                    ],
                }),
            ),
        ))
        .set_parent(room_root.eid());
    commands
        .spawn((
            Name::new("physics_debug_sticky5"),
            StickyPlainPhysicsBundle::new(
                Vec2::new(-300.0 * 4.0, -IDEAL_HEIGHT_f32) / 4.0,
                Bounds::from_shape(Shape::Circle { radius: 10.0 }),
            ),
        ))
        .set_parent(room_root.eid());
    commands
        .spawn((
            Name::new("physics_debug_sticky6"),
            StickyRotPhysicsBundle::new(
                Vec2::new(0.0, -IDEAL_HEIGHT_f32) / 4.0,
                Bounds::from_shape(Shape::Circle { radius: 10.0 }),
                DynoRot { rot: 6.0 },
            ),
        ))
        .set_parent(room_root.eid());

    commands
        .spawn((
            Name::new("physics_debug_uninteresting_tran_only"),
            DynoTran { vel: Vec2::ONE },
            SpatialBundle::default(),
            Bounds::from_shape(Shape::Circle { radius: 10.0 }),
        ))
        .set_parent(room_root.eid());
    commands
        .spawn((
            Name::new("physics_debug_uninteresting_rot_only"),
            DynoRot { rot: 0.1 },
            SpatialBundle::default(),
            Bounds::from_shape(Shape::Circle { radius: 10.0 }),
        ))
        .set_parent(room_root.eid());
    commands
        .spawn((
            Name::new("physics_debug_uninteresting_both"),
            DynoTran { vel: -Vec2::ONE },
            DynoRot { rot: 0.1 },
            SpatialBundle::default(),
            Bounds::from_shape(Shape::Circle { radius: 10.0 }),
        ))
        .set_parent(room_root.eid());

    commands
        .spawn((
            Name::new("physics_debug_sticky_both"),
            StickyBothPhysicsBundle::new(
                Vec2::new(-100.0, 0.0),
                Bounds::from_shape(Shape::Circle { radius: 15.0 }),
                DynoTran {
                    vel: Vec2::ONE * 4.0,
                },
                DynoRot { rot: 2.0 },
            ),
        ))
        .set_parent(room_root.eid());
    commands
        .spawn((
            Name::new("physics_debug_sticky_rot"),
            StickyRotPhysicsBundle::new(
                Vec2::new(-150.0, 0.0),
                Bounds::from_shape(Shape::Circle { radius: 15.0 }),
                DynoRot { rot: 0.2 },
            ),
        ))
        .set_parent(room_root.eid());
}

impl Bounds {
    fn draw(&self, pos: Vec2, rot: f32, gz: &mut Gizmos, color: Color) {
        // First draw the shape
        match self.get_shape() {
            Shape::Circle { radius } => {
                gz.circle_2d(pos, *radius, color);
            }
            Shape::Polygon { points } => {
                for [p1, p2] in points.to_lines() {
                    gz.line_2d(pos + p1.my_rotate(rot), pos + p2.my_rotate(rot), color);
                }
            }
        }
        // Then draw a line to show rotation (useful for circles)
        let diff = Vec2::X.my_rotate(rot) * 4.0;
        gz.line_2d(pos, pos + diff, color);
    }
}

fn draw_bounds(
    bounds_q: Query<(
        &Bounds,
        &GlobalTransform,
        Option<&StaticProvider>,
        Option<&TriggerKind>,
    )>,
    mouse_state: Res<MouseState>,
    mut gz: Gizmos,
) {
    for (bound, gtran, stat, trig) in &bounds_q {
        let (tran, angle) = gtran.tran_n_angle();
        let color = match (stat.map(|provider| provider.kind), trig) {
            (Some(StaticProviderKind::Normal), _) => tailwind::STONE_700,
            (Some(StaticProviderKind::Sticky), _) => tailwind::PINK_600,
            (None, Some(TriggerKind::Bird)) => tailwind::GREEN_600,
            (None, Some(TriggerKind::BulletGood)) => tailwind::GREEN_400,
            (None, Some(TriggerKind::Enemy)) => tailwind::RED_600,
            (None, Some(TriggerKind::BulletBad)) => tailwind::RED_500,
            (None, None) => tailwind::ZINC_950,
        };
        bound.draw(tran, angle, &mut gz, color.into());
    }
    if let Some(start_left) = mouse_state.get_left_drag_start() {
        gz.line_2d(start_left, mouse_state.get_world_pos(), tailwind::AMBER_700);
    }
    if let Some(start_right) = mouse_state.get_right_drag_start() {
        gz.line_2d(
            start_right,
            mouse_state.get_world_pos(),
            tailwind::ORANGE_700,
        );
    }
}

pub(super) fn register_physics_debug(app: &mut App) {
    app.add_computed_state::<ShowPhysicsBounds>();
    app.add_systems(OnEnter(ShowPhysicsBounds), startup_debug);
    app.add_systems(Update, draw_bounds.run_if(in_state(ShowPhysicsBounds)));
}
