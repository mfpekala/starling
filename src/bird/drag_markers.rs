use crate::prelude::*;

#[derive(Component, Default)]
struct LaunchDragMarker;

#[derive(Component, Default)]
struct FireDragMarker;

#[derive(Bundle)]
struct DragMarkerBundle<M: Component + Default> {
    name: Name,
    marker: M,
    multi: MultiAnimationManager,
    spatial: SpatialBundle,
}
impl<M: Component + Default> DragMarkerBundle<M> {
    fn new(name: &str) -> Self {
        Self {
            name: Name::new(format!("drag_marker_{name}")),
            marker: M::default(),
            multi: multi!([
                (
                    "tail",
                    anim_man!({
                        path: format!("drag_markers/{name}_tail.png").as_str(),
                        size: (5, 5),
                        length: 5,
                        fps: 600.0,
                    })
                    .with_render_layers(MenuCamera::render_layers())
                    .with_hidden(true),
                ),
                (
                    "head",
                    anim_man!({
                        path: format!("drag_markers/{name}_head.png").as_str(),
                        size: (7, 9),
                    })
                    .with_offset(Vec3::new(0.0, 0.0, 0.1))
                    .with_render_layers(MenuCamera::render_layers())
                    .with_hidden(true),
                )
            ]),
            spatial: spat_tran(0.0, 0.0, ZIX_DRAG_MARKERS),
        }
    }
}

fn setup_drag_markers(mut commands: Commands, menu_root: Res<MenuRoot>) {
    commands
        .spawn(DragMarkerBundle::<LaunchDragMarker>::new("launch"))
        .set_parent(menu_root.eid());
    commands
        .spawn(DragMarkerBundle::<FireDragMarker>::new("fire"))
        .set_parent(menu_root.eid());
}

fn destroy_drag_markers(
    mut commands: Commands,
    eids: Query<Entity, Or<(With<LaunchDragMarker>, With<FireDragMarker>)>>,
) {
    for eid in &eids {
        commands.entity(eid).despawn_recursive();
    }
}

fn update_drag_markers(
    bird: Query<&Bird>,
    mouse_input: Res<MouseInput>,
    mut launch_multi: Query<
        (&mut MultiAnimationManager, &mut Transform),
        (With<LaunchDragMarker>, Without<FireDragMarker>),
    >,
    mut fire_multi: Query<
        (&mut MultiAnimationManager, &mut Transform),
        (Without<LaunchDragMarker>, With<FireDragMarker>),
    >,
    mut commands: Commands,
) {
    let Ok(bird) = bird.get_single() else {
        return;
    };

    macro_rules! manage_multi {
        ($multi:ident, $tran:ident, $length:expr, $start:expr, $commands:expr) => {
            $tran.translation = $start.extend($tran.translation.z);
            $tran.set_angle(($start - mouse_input.get_world_pos()).to_angle());
            $multi
                .manager_mut("tail")
                .set_points(simple_rect($length, 5.0), $commands);
            $multi
                .manager_mut("tail")
                .set_offset(Vec3::new(-$length / 2.0, 0.0, 0.0), $commands);
        };
    }

    let (mut launch_multi, mut launch_tran) = launch_multi.single_mut();
    let launch_hidden = {
        if bird.launches_left > 0 {
            if let Some(start) = mouse_input.get_left_drag_start() {
                let length = (start - mouse_input.get_world_pos()).length();
                manage_multi!(launch_multi, launch_tran, length, start, &mut commands);
                false
            } else {
                true
            }
        } else {
            true
        }
    };
    for anim in launch_multi.map.values_mut() {
        anim.set_hidden(launch_hidden, &mut commands);
    }

    let (mut fire_multi, mut fire_tran) = fire_multi.single_mut();
    let fire_hidden = {
        if bird.bullets_left > 0 {
            if let Some(start) = mouse_input.get_right_drag_start() {
                let length = (start - mouse_input.get_world_pos()).length();
                manage_multi!(fire_multi, fire_tran, length, start, &mut commands);
                false
            } else {
                true
            }
        } else {
            true
        }
    };
    for anim in fire_multi.map.values_mut() {
        anim.set_hidden(fire_hidden, &mut commands);
    }
}

pub(super) struct DragMarkerPlugin;
impl Plugin for DragMarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PhysicsState::Active), setup_drag_markers);
        app.add_systems(OnExit(PhysicsState::Active), destroy_drag_markers);
        app.add_systems(
            Update,
            update_drag_markers.run_if(in_state(PhysicsState::Active)),
        );
    }
}
