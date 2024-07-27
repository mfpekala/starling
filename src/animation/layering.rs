use bevy::render::camera::RenderTarget;
use bevy::render::{
    render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    texture::BevyDefault,
    view::RenderLayers,
};
use bevy::sprite::MaterialMesh2dBundle;

use crate::prelude::*;

pub trait CameraLayer {
    const RENDER_LAYER: usize;

    fn layer() -> usize {
        Self::RENDER_LAYER
    }

    fn render_layers() -> RenderLayers {
        RenderLayers::from_layers(&[Self::RENDER_LAYER])
    }
}

#[derive(Component, Debug, Reflect, Default)]
pub struct BgSpriteCamera;
#[derive(Component, Debug, Reflect, Default)]
pub struct BgLightCamera;
#[derive(Component, Debug, Reflect, Default)]
pub struct SpriteCamera;
#[derive(Component, Debug, Reflect, Default)]
pub struct LightCamera;
#[derive(Component, Debug, Reflect, Default)]
pub struct MenuCamera;

impl CameraLayer for BgSpriteCamera {
    const RENDER_LAYER: usize = 1;
}
impl CameraLayer for BgLightCamera {
    const RENDER_LAYER: usize = 2;
}
impl CameraLayer for SpriteCamera {
    const RENDER_LAYER: usize = 3;
}
impl CameraLayer for LightCamera {
    const RENDER_LAYER: usize = 4;
}
impl CameraLayer for MenuCamera {
    const RENDER_LAYER: usize = 5;
}

#[derive(Resource)]
pub struct LayeringSettings {
    bg_clear_color: ClearColorConfig,
    bg_ambient_light: ClearColorConfig,
    clear_color: ClearColorConfig,
    ambient_light: ClearColorConfig,
    menu_clear_color: ClearColorConfig,
}
impl Default for LayeringSettings {
    fn default() -> Self {
        Self {
            bg_clear_color: ClearColorConfig::Default,
            bg_ambient_light: ClearColorConfig::Custom(Color::srgb(0.5, 0.5, 0.5)),
            clear_color: ClearColorConfig::Custom(Color::srgba(0.1, 0.1, 0.1, 0.05)),
            ambient_light: ClearColorConfig::Custom(Color::srgb(0.6, 0.6, 0.6)),
            menu_clear_color: ClearColorConfig::Custom(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        }
    }
}

#[derive(Resource, Default)]
pub(super) struct CameraTargets {
    pub bg_sprite_target: Handle<Image>,
    pub bg_light_target: Handle<Image>,
    pub sprite_target: Handle<Image>,
    pub light_target: Handle<Image>,
    pub menu_target: Handle<Image>,
}
impl CameraTargets {
    pub fn create(images: &mut Assets<Image>) -> Self {
        macro_rules! make_layer_image {
            ($label:expr, $unique_u128:expr, $size:expr) => {{
                let target_extent = Extent3d {
                    width: $size.x,
                    height: $size.y,
                    ..default()
                };

                // Makes the image
                let mut image = Image {
                    texture_descriptor: TextureDescriptor {
                        label: Some($label),
                        size: target_extent,
                        dimension: TextureDimension::D2,
                        format: TextureFormat::bevy_default(),
                        mip_level_count: 1,
                        sample_count: 1,
                        usage: TextureUsages::TEXTURE_BINDING
                            | TextureUsages::COPY_DST
                            | TextureUsages::RENDER_ATTACHMENT,
                        view_formats: &[],
                    },
                    ..default()
                };
                // Fills it with zeros
                image.resize(target_extent);
                let handle: Handle<Image> = Handle::weak_from_u128($unique_u128);
                images.insert(handle.id(), image);
                handle
            }};
        }

        let bg_light_handle =
            make_layer_image!("target_bg_light", 84562364042238462870, WINDOW_VEC);
        let bg_sprite_handle =
            make_layer_image!("target_bg_sprite", 81297563682952991276, WINDOW_VEC);
        let light_handle = make_layer_image!("target_light", 84562364042238462871, WINDOW_VEC);
        let sprite_handle = make_layer_image!("target_sprite", 81297563682952991277, WINDOW_VEC);
        let menu_handle = make_layer_image!("target_menu", 51267563632952991278, WINDOW_VEC);

        Self {
            bg_light_target: bg_light_handle,
            bg_sprite_target: bg_sprite_handle,
            light_target: light_handle,
            sprite_target: sprite_handle,
            menu_target: menu_handle,
        }
    }
}

macro_rules! impl_layer_quad_n_mat {
    ($prefix:ident, $mat_type:ty, $unique_u128s:expr) => {
        paste::paste! {
            const [<$prefix _QUAD>]: Handle<Mesh> = Handle::weak_from_u128($unique_u128s);
            const [<$prefix _MATERIAL>]: Handle<$mat_type> = Handle::weak_from_u128($unique_u128s + 1);
        }
    };
}

impl_layer_quad_n_mat!(BG_PP, BlendTexturesMaterial, 23467206864860343677);
impl_layer_quad_n_mat!(PP, BlendTexturesMaterial, 53466206864860343678);
impl_layer_quad_n_mat!(MENU, AnimationMaterial, 36467206864860383170);

fn remake_layering_materials(
    camera_targets: &CameraTargets,
    blend_materials: &mut ResMut<Assets<BlendTexturesMaterial>>,
    anim_materials: &mut ResMut<Assets<AnimationMaterial>>,
) {
    let bg_material = BlendTexturesMaterial {
        texture1: camera_targets.bg_sprite_target.clone(),
        texture2: camera_targets.bg_light_target.clone(),
    };
    let material = BlendTexturesMaterial {
        texture1: camera_targets.sprite_target.clone(),
        texture2: camera_targets.light_target.clone(),
    };
    let menu_material = AnimationMaterial::from_handle(
        camera_targets.menu_target.clone(),
        1,
        Vec2::ONE,
        Color::WHITE,
    );
    blend_materials.insert(BG_PP_MATERIAL.id(), bg_material);
    blend_materials.insert(PP_MATERIAL.id(), material);
    anim_materials.insert(MENU_MATERIAL.id(), menu_material);
}

fn setup_post_processing(
    mut commands: Commands,
    mut camera_targets: ResMut<CameraTargets>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut blend_materials: ResMut<Assets<BlendTexturesMaterial>>,
    mut anim_materials: ResMut<Assets<AnimationMaterial>>,
    layering_root: Res<LayeringRoot>,
) {
    let ideal_quad = Mesh::from(Rectangle::new(IDEAL_WIDTH_f32, IDEAL_HEIGHT_f32));
    meshes.insert(BG_PP_QUAD.id(), ideal_quad.clone());
    meshes.insert(PP_QUAD.id(), ideal_quad.clone());
    meshes.insert(MENU_QUAD.id(), ideal_quad.clone());

    *camera_targets = CameraTargets::create(&mut images);

    remake_layering_materials(&camera_targets, &mut blend_materials, &mut anim_materials);

    let combined_layer = RenderLayers::from_layers(&[30]);

    macro_rules! spawn_layer_mat_mesh {
        ($name:expr, $quad:expr, $mat:expr, $z:expr) => {{
            commands
                .spawn((
                    Name::new($name),
                    MaterialMesh2dBundle {
                        mesh: $quad.clone().into(),
                        material: $mat,
                        transform: Transform {
                            translation: Vec3::Z * $z,
                            scale: Vec3::new(IDEAL_GROWTH_f32, IDEAL_GROWTH_f32, 1.0),
                            ..default()
                        },
                        ..default()
                    },
                    combined_layer.clone(),
                ))
                .set_parent(layering_root.eid());
        }};
    }
    spawn_layer_mat_mesh!("bg_pp_layer", BG_PP_QUAD, BG_PP_MATERIAL, 1.0);
    spawn_layer_mat_mesh!("pp_layer", PP_QUAD, PP_MATERIAL, 2.0);
    spawn_layer_mat_mesh!("menu_layer", MENU_QUAD, MENU_MATERIAL, 3.0);

    commands
        .spawn((
            Name::new("post_processing_camera"),
            Camera2dBundle {
                camera: Camera {
                    order: 6,
                    ..default()
                },
                ..default()
            },
            InheritedVisibility::VISIBLE,
            combined_layer,
        ))
        .set_parent(layering_root.eid());
}

fn setup_layers(
    mut commands: Commands,
    camera_targets: Res<CameraTargets>,
    layering_settings: Res<LayeringSettings>,
    layering_root: Res<LayeringRoot>,
) {
    macro_rules! spawn_layer_camera {
        ($comp:ty, $name:expr, $order:expr, $image:expr, $clear_color:expr) => {{
            commands
                .spawn((
                    Name::new($name),
                    Camera2dBundle {
                        camera: Camera {
                            order: $order,
                            target: RenderTarget::Image($image),
                            clear_color: $clear_color,
                            ..default()
                        },
                        projection: OrthographicProjection {
                            scale: 1.0 / IDEAL_GROWTH_f32,
                            near: ZIX_MIN,
                            far: ZIX_MAX,
                            ..default()
                        },
                        ..Default::default()
                    },
                    <$comp>::default(),
                    <$comp>::render_layers(),
                ))
                .set_parent(layering_root.eid());
        }};
    }
    spawn_layer_camera!(
        BgLightCamera,
        "bg_light_camera",
        0,
        camera_targets.bg_light_target.clone(),
        layering_settings.bg_ambient_light
    );
    spawn_layer_camera!(
        BgSpriteCamera,
        "bg_sprite_camera",
        1,
        camera_targets.bg_sprite_target.clone(),
        layering_settings.bg_clear_color
    );
    spawn_layer_camera!(
        LightCamera,
        "light_camera",
        2,
        camera_targets.light_target.clone(),
        layering_settings.ambient_light
    );
    spawn_layer_camera!(
        SpriteCamera,
        "sprite_camera",
        3,
        camera_targets.sprite_target.clone(),
        layering_settings.clear_color
    );
    spawn_layer_camera!(
        MenuCamera,
        "menu_camera",
        4,
        camera_targets.menu_target.clone(),
        layering_settings.menu_clear_color
    );
}

pub(super) struct LayeringPlugin;
impl Plugin for LayeringPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LayeringSettings::default());
        app.insert_resource(CameraTargets::default());

        app.add_systems(
            Startup,
            (setup_post_processing, setup_layers)
                .chain()
                .after(RootInit),
        );
    }
}
