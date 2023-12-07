use nalgebra_glm::smoothstep;
use noise::{NoiseFn, OpenSimplex};
use palette::Mix;
use rand::random;

use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::{DisplayPane, FrameSize, PixelFrame};
use crate::pattern_builder::component::property::{PropertyInfo};
use crate::pattern_builder::component::property::component::{TexturePropCore};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::texture::{Texture, TextureLayer};

#[derive(Clone)]
pub struct TwoToneConfig {
    textures: (Prop<TextureLayer>, Prop<TextureLayer>),
    noise_flow_speed: Prop<f64>,
    gradient_width: Prop<f64>,
    gradient_offset: Prop<f64>,
    noise_scaling: Prop<f64>,
    noise_travel_speed: Prop<f64>,
}

impl TwoToneConfig {
    pub fn new(colors: (TextureLayer, TextureLayer), flow_speed: f64, noise_scaling: f64) -> Self {
        Self {
            textures: (
                TexturePropCore::new(colors.0).into_prop(PropertyInfo::new("Texture 1").set_display_pane(DisplayPane::Tree)),
                TexturePropCore::new(colors.1).into_prop(PropertyInfo::new("Texture 2").set_display_pane(DisplayPane::Tree))
            ),
            noise_flow_speed: NumPropCore::new_slider(flow_speed, 0.0..20.0, 0.1).into_prop(PropertyInfo::new("Noise Flow Speed")),
            noise_scaling: NumPropCore::new_slider(noise_scaling, 0.0..1.0, 0.02).into_prop(PropertyInfo::new("Noise Scaling")),
            noise_travel_speed: NumPropCore::new_slider(0.0, -100.0..100.0, 0.25).into_prop(PropertyInfo::new("Noise Travel Speed")),
            gradient_width: NumPropCore::new_slider(0.2, 0.0..1.0, 0.01).into_prop(PropertyInfo::new("Gradient Width")),
            gradient_offset: NumPropCore::new_slider(0.0, -1.0..1.0, 0.01).into_prop(PropertyInfo::new("Gradient Offset")),
        }
    }

    pub fn into_texture(self) -> TwoTone {
        TwoTone::new(random(), self)
    }

    pub fn noise_flow_speed(&self) -> &Prop<f64> {
        &self.noise_flow_speed
    }
    
    pub fn noise_scaling(&self) -> &Prop<f64> {
        &self.noise_scaling
    }
    
    pub fn noise_travel_speed(&self) -> &Prop<f64> {
        &self.noise_travel_speed
    }
    
    pub fn gradient_width(&self) -> &Prop<f64> {
        &self.gradient_width
    }
    
    pub fn gradient_offset(&self) -> &Prop<f64> {
        &self.gradient_offset
    }
}

#[derive(Clone)]
pub struct TwoTone {
    simplex_noise: OpenSimplex,
    config: TwoToneConfig,
}

impl TwoTone {
    fn new(seed: u32, layer: TwoToneConfig) -> Self {
        Self {
            simplex_noise: OpenSimplex::new(seed),
            config: layer,
        }
    }

    pub fn config(&self) -> &TwoToneConfig {
        &self.config
    }
}

impl Component for TwoTone {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.config.noise_flow_speed,
            self.config.noise_scaling,
            self.config.noise_travel_speed,
            self.config.gradient_width,
            self.config.gradient_offset,
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.config.noise_flow_speed,
            self.config.noise_scaling,
            self.config.noise_travel_speed,
            self.config.gradient_width,
            self.config.gradient_offset,
        );
    }
}

impl Texture for TwoTone {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        let colors0 = self.config.textures.0.write().next_frame(t, num_pixels);
        let colors1 = self.config.textures.1.write().next_frame(t, num_pixels);
        let mut pixels = vec![];
        for x in 0..num_pixels {
            let noise = self.simplex_noise.get([
                t * *self.config.noise_flow_speed.read(),
                (x as f64 - t * *self.config.noise_travel_speed.read()) * *self.config.noise_scaling.read()
            ]) as f32;
            pixels.push(
                colors0[x as usize].mix(
                    colors1[x as usize],
                    smoothstep(
                        (*self.config.gradient_offset.read() - *self.config.gradient_width.read()) as f32,
                        (*self.config.gradient_offset.read() + *self.config.gradient_width.read()) as f32,
                        noise
                    )
                )
            )
        }
        pixels
    }
}