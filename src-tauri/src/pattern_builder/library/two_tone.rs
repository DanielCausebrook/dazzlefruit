use nalgebra_glm::smoothstep;
use noise::{NoiseFn, OpenSimplex};
use palette::Mix;
use rand::random;

use crate::{impl_component, impl_component_config};
use crate::pattern_builder::component::ComponentInfo;
use crate::pattern_builder::component::data::{DisplayPane, FrameSize, PixelFrame};
use crate::pattern_builder::component::property::{Property, PropertyInfo};
use crate::pattern_builder::component::property::cloning::BlendModeProperty;
use crate::pattern_builder::component::property::locked::TextureProperty;
use crate::pattern_builder::component::property::num::{NumProperty, NumSlider};
use crate::pattern_builder::component::texture::Texture;

#[derive(Clone)]
pub struct TwoToneConfig {
    info: ComponentInfo,
    textures: (TextureProperty, TextureProperty),
    noise_flow_speed: NumProperty<f64>,
    gradient_width: NumProperty<f64>,
    gradient_offset: NumProperty<f64>,
    noise_scaling: NumProperty<f64>,
    noise_travel_speed: NumProperty<f64>,
    blend_mode: BlendModeProperty,
}

impl TwoToneConfig {
    pub fn new(colors: (impl Texture, impl Texture), flow_speed: impl Into<NumProperty<f64>>, noise_scaling: impl Into<NumProperty<f64>>) -> Self {
        Self {
            info: ComponentInfo::new("TwoTone"),
            textures: (
                TextureProperty::new(Box::new(colors.0), PropertyInfo::new("Texture 1").display_pane(DisplayPane::Tree)),
                TextureProperty::new(Box::new(colors.1), PropertyInfo::new("Texture 2").display_pane(DisplayPane::Tree))
            ),
            noise_flow_speed: flow_speed.into()
                .set_info(PropertyInfo::new("Noise Flow Speed"))
                .set_slider(Some(NumSlider::new(0.0..20.0, 0.1))),
            noise_scaling: noise_scaling.into()
                .set_info(PropertyInfo::new("Noise Scaling"))
                .set_slider(Some(NumSlider::new(0.0..1.0, 0.02))),
            noise_travel_speed: NumProperty::new(0.0, PropertyInfo::new("Noise Travel Speed"))
                .set_slider(Some(NumSlider::new(-100.0..100.0, 0.25))),
            gradient_width: NumProperty::new(0.2, PropertyInfo::new("Gradient Width"))
                .set_slider(Some(NumSlider::new(0.0..1.0, 0.01))),
            gradient_offset: NumProperty::new(0.0, PropertyInfo::new("Gradient Offset"))
                .set_slider(Some(NumSlider::new(-1.0..1.0, 0.01, ))),
            blend_mode: BlendModeProperty::default(),
        }
    }

    pub fn into_texture(self) -> TwoTone {
        TwoTone::new(random(), self)
    }

    pub fn init_gradient_width(mut self, value: impl Into<NumProperty<f64>>) -> Self {
        self.gradient_width = value.into()
            .set_info(self.gradient_width.get_info().clone())
            .set_slider(self.gradient_width.get_slider().clone());
        self
    }

    pub fn init_gradient_offset(mut self, value: impl Into<NumProperty<f64>>) -> Self {
        self.gradient_offset = value.into()
            .set_info(self.gradient_offset.get_info().clone())
            .set_slider(self.gradient_offset.get_slider().clone());
        self
    }

    pub fn init_noise_velocity(mut self, value: impl Into<NumProperty<f64>>) -> Self {
        self.noise_travel_speed = value.into()
            .set_info(self.noise_travel_speed.get_info().clone())
            .set_slider(self.noise_travel_speed.get_slider().clone());
        self
    }

    pub fn init_blend_mode(mut self, value: impl Into<BlendModeProperty>) -> Self {
        self.blend_mode = value.into()
            .set_info(self.blend_mode.get_info().clone());
        self
    }

    pub fn noise_flow_speed(&self) -> &NumProperty<f64> {
        &self.noise_flow_speed
    }

    pub fn gradient_width(&self) -> &NumProperty<f64> {
        &self.gradient_width
    }

    pub fn gradient_offset(&self) -> &NumProperty<f64> {
        &self.gradient_offset
    }
}

impl_component_config!(self: TwoToneConfig, self.info, [
    self.textures.0,
    self.textures.1,
    self.noise_flow_speed,
    self.noise_scaling,
    self.noise_travel_speed,
    self.gradient_width,
    self.gradient_offset,
]);

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

    pub fn get_config(&self) -> &TwoToneConfig {
        &self.config
    }
}

impl_component!(self: TwoTone, self.config, "pixel");

impl Texture for TwoTone {
    fn get_blend_mode(&self) -> &BlendModeProperty {
        &self.config.blend_mode
    }

    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        let colors0 = self.config.textures.0.write().next_frame(t, num_pixels);
        let colors1 = self.config.textures.1.write().next_frame(t, num_pixels);
        let mut pixels = vec![];
        for x in 0..num_pixels {
            let noise = self.simplex_noise.get([
                t * self.config.noise_flow_speed.get(),
                (x as f64 - t * self.config.noise_travel_speed.get()) * self.config.noise_scaling.get()
            ]) as f32;
            pixels.push(
                colors0[x as usize].mix(
                    colors1[x as usize],
                    smoothstep(
                        (self.config.gradient_offset.get() - self.config.gradient_width.get()) as f32,
                        (self.config.gradient_offset.get() + self.config.gradient_width.get()) as f32,
                        noise
                    )
                )
            )
        }
        pixels
    }
}