use bevy::{prelude::*, render::{RenderApp, render_resource::{BindGroupLayout, Sampler, CachedRenderPipelineId, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ShaderStages, BindingType, TextureSampleType, TextureViewDimension, SamplerBindingType, SamplerDescriptor, PipelineCache, RenderPipelineDescriptor, FragmentState, ColorTargetState, TextureFormat, ColorWrites, PrimitiveState, MultisampleState}, renderer::RenderDevice, texture::BevyDefault}, core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state};

pub mod prelude {
    pub use bevy::render::extract_component::ExtractComponent;
    pub use bevy::render::render_resource::ShaderType;

    pub use crate::PostProcessingPlugin;
    pub use crate::PostProcessingEffect;
}

#[derive(Clone, Copy, Debug)]
pub struct PostProcessingEffect {
    pub path: &'static str,
}

/// Postprocessing plugin for bevy.
pub struct PostProcessingPlugin {
    effects: Vec<PostProcessingEffect>,
}

impl PostProcessingPlugin {
    /// Creates a new postprocessing pipeline.
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    pub fn with(&mut self, effect: PostProcessingEffect) -> &mut Self {
        self.effects.push(effect);
        self
    }

    pub fn build(&self) -> Self {
        Self {
            effects: self.effects.clone()
        }
    }
}

impl From<&mut Self> for PostProcessingPlugin {
    fn from(value: &mut Self) -> Self {
        value.into()
    }
}

impl Plugin for PostProcessingPlugin {
    fn build(&self, app: &mut App) {
        info!("Building plugin...");
        for effect in self.effects.clone() {
        }
    }
}