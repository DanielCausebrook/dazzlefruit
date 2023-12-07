use std::sync::Arc;
use nalgebra_glm::smoothstep;
use palette::WithAlpha;
use parking_lot::Mutex;

use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::{Component, LayerInfo};
use crate::pattern_builder::component::data::{BlendMode, DisplayPane, Frame, FrameSize, PixelFrame};
use crate::pattern_builder::component::property::component::TextureGeneratorPropCore;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::computed::ComputedPropCore;
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::texture::{Texture, TextureLayer};
use crate::pattern_builder::component::texture_generator::{TextureGenerator, TextureGeneratorLayer};
use crate::pattern_builder::library::core::SolidColor;
use crate::pattern_builder::library::two_tone::{TwoTone, TwoToneConfig};

#[derive(Clone)]
pub struct PulsingBlocksConfig {
    textures: Prop<TextureGeneratorLayer>,
    textures_per_second: Prop<f64>,
    texture_duration: Prop<f64>,
    fade_in_out_duration: Prop<f64>,
    flow_speed: Prop<f64>,
    scaling: Prop<f64>,
    block_density: Prop<f64>,
    block_softness: Prop<f64>,
}

impl PulsingBlocksConfig {
    pub fn new(textures: TextureGeneratorLayer) -> Self {
        Self {
            textures: TextureGeneratorPropCore::new(textures).into_prop(PropertyInfo::new("Textures").set_display_pane(DisplayPane::Tree)),
            textures_per_second: NumPropCore::new_slider(1.0, 0.0..5.0, 0.05).into_prop(PropertyInfo::new("Textures per Second")),
            texture_duration: NumPropCore::new_slider(20.0, 0.0..30.0, 0.1).into_prop(PropertyInfo::new("Texture Duration")),
            fade_in_out_duration: NumPropCore::new_slider(8.0, 0.0..10.0, 0.05).into_prop(PropertyInfo::new("Fade In/Out Duration")),
            flow_speed: NumPropCore::new_slider(0.7, 0.0..20.0, 0.1).into_prop(PropertyInfo::new("Flow Speed")),
            scaling: NumPropCore::new_slider(0.07, 0.0..1.0, 0.01).into_prop(PropertyInfo::new("Scaling")),
            block_density: NumPropCore::new_slider(0.7, 0.0..2.0, 0.05).into_prop(PropertyInfo::new("Block Density")),
            block_softness: NumPropCore::new_slider(0.03, 0.0..1.0, 0.01).into_prop(PropertyInfo::new("Block Softness")),
        }
    }
    pub fn into_texture(self) -> PulsingBlocks {
        PulsingBlocks::new(self)
    }
}

#[derive(Clone)]
struct PulsingBlockLayer {
    two_tone: TwoTone,
    start_t: f64,
    duration: f64,
    fade_in_out_duration: f64,
    transition_progress: Arc<Mutex<f64>>,
}
impl PulsingBlockLayer {
    fn new(config: &PulsingBlocksConfig, texture: TextureLayer, start_t: f64) -> Self {
        let two_tone_config = TwoToneConfig::new(
            (SolidColor::new(palette::named::BLACK.transparent().into()).into_layer(LayerInfo::new("BG")), texture),
            0.0,
            1.0,
        );
        two_tone_config.noise_scaling().replace_core(config.scaling.map_core(|x| *x));
        two_tone_config.noise_flow_speed().replace_core(config.flow_speed.map_core(|x| *x));
        two_tone_config.gradient_width().replace_core(config.block_softness.map_core(|x| *x));

        let transition_progress = Arc::new(Mutex::new(0.0));

        let captured_progress = transition_progress.clone();
        let block_density = config.block_density.clone();
        two_tone_config.gradient_offset().replace_core(ComputedPropCore::new(move || {
            1.0 - (*block_density.read() * *captured_progress.lock())
        }));

        Self {
            two_tone: two_tone_config.into_texture(),
            start_t,
            duration: *config.texture_duration.read(),
            fade_in_out_duration: *config.fade_in_out_duration.read(),
            transition_progress: transition_progress.clone(),
        }
    }
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> Option<PixelFrame> {
        let t_since_start = t - self.start_t;
        if t_since_start > self.duration {
            None
        } else {
            *self.transition_progress.lock() = smoothstep(0.0, self.fade_in_out_duration, t_since_start)
                - smoothstep(self.duration - self.fade_in_out_duration, self.duration, t_since_start);
            Some(self.two_tone.next_frame(t, num_pixels))
        }
    }
}

#[derive(Clone)]
pub struct PulsingBlocks {
    config: PulsingBlocksConfig,
    layers: Vec<PulsingBlockLayer>,
    last_texture_layer_t: f64,
}

impl PulsingBlocks {
    pub fn new(config: PulsingBlocksConfig) -> Self {
        Self {
            config,
            layers: vec![],
            last_texture_layer_t: 0.0,
        }
    }
}

impl Component for PulsingBlocks {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.config.textures,
            self.config.textures_per_second,
            self.config.texture_duration,
            self.config.fade_in_out_duration,
            self.config.flow_speed,
            self.config.scaling,
            self.config.block_density,
            self.config.block_softness,
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.config.textures,
            self.config.textures_per_second,
            self.config.texture_duration,
            self.config.fade_in_out_duration,
            self.config.flow_speed,
            self.config.scaling,
            self.config.block_density,
            self.config.block_softness,
        );
    }
}

impl Texture for PulsingBlocks {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        let texture_delay = 1.0 / *self.config.textures_per_second.read();
        if t - self.last_texture_layer_t > texture_delay {
            self.layers.push(PulsingBlockLayer::new(
                &self.config,
                self.config.textures.write().next_texture(),
                self.last_texture_layer_t + texture_delay
            ));
            let lag = t - self.last_texture_layer_t - texture_delay;
            self.last_texture_layer_t = t - lag.min(texture_delay);
        }
        let mut frames = vec![];
        self.layers = self.layers.drain(0..)
            .filter_map(|mut layer| {
                match layer.next_frame(t, num_pixels) {
                    None => None,
                    Some(frame) => {
                        frames.push(frame);
                        Some(layer)
                    }
                }
            })
            .collect();
        frames.iter()
            .rev()
            .fold(
                vec![palette::named::BLACK.transparent().into(); num_pixels as usize],
                |active, frame| frame.blend(active, BlendMode::Normal)
            )
    }
}