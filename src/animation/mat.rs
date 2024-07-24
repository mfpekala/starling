use crate::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayoutRef;
use bevy::render::render_resource::{
    AsBindGroup, BlendComponent, BlendFactor, BlendOperation, BlendState, RenderPipelineDescriptor,
    ShaderRef, SpecializedMeshPipelineError,
};
use bevy::sprite::{Material2d, Material2dKey};

pub const SPRITE_MATERIAL_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(8267429772218888889);

#[derive(AsBindGroup, Debug, Clone, Asset, Reflect, PartialEq)]
pub struct AnimationMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
    #[uniform(3)]
    pub index: f32,
    #[uniform(4)]
    pub length: f32,
    #[uniform(5)]
    pub x_offset: f32,
    #[uniform(6)]
    pub y_offset: f32,
    #[uniform(7)]
    pub x_repetitions: f32,
    #[uniform(8)]
    pub y_repetitions: f32,
    #[uniform(9)]
    pub r: f32,
    #[uniform(10)]
    pub g: f32,
    #[uniform(11)]
    pub b: f32,
    #[uniform(12)]
    pub a: f32,
}
impl AnimationMaterial {
    pub fn from_handle(
        handle: Handle<Image>,
        length: u32,
        repetitions: Vec2,
        color: Color,
    ) -> Self {
        Self {
            texture: handle,
            index: 0.0,
            length: length as f32,
            x_offset: 0.0,
            y_offset: 0.0,
            x_repetitions: repetitions.x,
            y_repetitions: repetitions.y,
            r: color.to_srgba().red,
            g: color.to_srgba().blue,
            b: color.to_srgba().green,
            a: color.to_srgba().alpha,
        }
    }
}

impl Material2d for AnimationMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animation_mat.wgsl".into()
    }
}

pub const BLEND_ADD: BlendState = BlendState {
    color: BlendComponent {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::OneMinusSrcAlpha,
        operation: BlendOperation::Add,
    },
    alpha: BlendComponent {
        src_factor: BlendFactor::One,
        dst_factor: BlendFactor::OneMinusSrcAlpha,
        operation: BlendOperation::Add,
    },
};

#[derive(AsBindGroup, TypePath, Asset, Debug, Clone)]
pub struct BlendTexturesMaterial {
    #[texture(1)]
    #[sampler(2)]
    pub texture1: Handle<Image>,
    #[texture(3)]
    #[sampler(4)]
    pub texture2: Handle<Image>,
}

impl Material2d for BlendTexturesMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/blend_light.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(fragment) = &mut descriptor.fragment {
            if let Some(target_state) = &mut fragment.targets[0] {
                target_state.blend = Some(BLEND_ADD);
            }
        }

        Ok(())
    }
}
