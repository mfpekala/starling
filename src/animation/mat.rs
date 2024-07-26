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
    // The below need to be packed for wasm where stuff has to be 16-byte aligned
    #[uniform(3)]
    pub ix_length_flipx_flipy: Vec4, // NOTE: 1.0 = don't flip, -1.0 = flip
    #[uniform(4)]
    pub xoff_yoff_xrep_yrep: Vec4,
    #[uniform(5)]
    pub rgba: Vec4,
}
impl AnimationMaterial {
    pub fn from_handle(
        handle: Handle<Image>,
        length: u32,
        repetitions: Vec2,
        color: Color,
    ) -> Self {
        let srgba_thing = color.to_srgba();
        Self {
            texture: handle,
            ix_length_flipx_flipy: Vec4::new(0.0, length as f32, 1.0, 1.0),
            xoff_yoff_xrep_yrep: Vec4::new(0.0, 0.0, repetitions.x, repetitions.y),
            rgba: Vec4::new(
                srgba_thing.red,
                srgba_thing.green,
                srgba_thing.blue,
                srgba_thing.alpha,
            ),
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
