use nalgebra_glm::smoothstep;
use palette::WithAlpha;
use crate::pattern_builder::component::{ComponentInfo, ComponentConfig, Component};
use crate::pattern_builder::component::data::{BlendMode, DisplayPane, Frame, FrameSize, PixelFrame};
use crate::pattern_builder::component::texture::Texture;
use crate::pattern_builder::component::property::{Property, PropertyInfo};
use crate::pattern_builder::component::property::cloning::BlendModeProperty;
use crate::pattern_builder::component::property::locked::TextureProducerProperty;
use crate::pattern_builder::component::property::num::{NumProperty, NumSlider};
use crate::pattern_builder::library::core::SolidColor;
use crate::pattern_builder::library::two_tone::{TwoTone, TwoToneConfig};

#[derive(Clone)]
pub struct PulsingBlocksConfig {
    info: ComponentInfo,
    blend_mode: BlendModeProperty,
    textures: TextureProducerProperty,
    textures_per_second: NumProperty<f64>,
    texture_duration: NumProperty<f64>,
    fade_in_out_duration: NumProperty<f64>,
    flow_speed: NumProperty<f64>,
    scaling: NumProperty<f64>,
    block_density: NumProperty<f64>,
    block_softness: NumProperty<f64>,
}
impl PulsingBlocksConfig {
    pub fn new(textures: TextureProducerProperty) -> Self {
        Self {
            info: ComponentInfo::new("Pulsing Blocks"),
            blend_mode: BlendModeProperty::default(),
            textures: textures.set_info(PropertyInfo::new("Textures").display_pane(DisplayPane::Tree)),
            textures_per_second: NumProperty::new(1.0, PropertyInfo::new("Textures per Second"))
                .set_slider(Some(NumSlider::new(0.0..5.0, 0.05))),
            texture_duration: NumProperty::new(20.0, PropertyInfo::new("Texture Duration"))
                .set_slider(Some(NumSlider::new(0.0..30.0, 0.1))),
            fade_in_out_duration: NumProperty::new(8.0, PropertyInfo::new("Fade In/Out Duration"))
                .set_slider(Some(NumSlider::new(0.0..10.0, 0.05))),
            flow_speed: NumProperty::new(0.7, PropertyInfo::new("Flow Speed"))
                .set_slider(Some(NumSlider::new(0.0..20.0, 0.1))),
            scaling: NumProperty::new(0.07, PropertyInfo::new("Scaling"))
                .set_slider(Some(NumSlider::new(0.0..1.0, 0.01))),
            block_density: NumProperty::new(0.7, PropertyInfo::new("Block Density"))
                .set_slider(Some(NumSlider::new(0.0..2.0, 0.05))),
            block_softness: NumProperty::new(0.03, PropertyInfo::new("Block Softness"))
                .set_slider(Some(NumSlider::new(0.0..1.0, 0.01))),
        }
    }
    pub fn into_texture(self) -> PulsingBlocks {
        PulsingBlocks::new(self)
    }
}
impl ComponentConfig for PulsingBlocksConfig {
    fn info(&self) -> &ComponentInfo {
        &self.info
    }

    fn info_mut(&mut self) -> &mut ComponentInfo {
        &mut self.info
    }

    fn properties(&self) -> Vec<&dyn Property> {
        vec![
            &self.textures,
            &self.textures_per_second,
            &self.texture_duration,
            &self.fade_in_out_duration,
            &self.flow_speed,
            &self.scaling,
            &self.block_density,
            &self.block_softness,
        ]
    }

    fn properties_mut(&mut self) -> Vec<&mut dyn Property> {
        vec![
            &mut self.textures,
            &mut self.textures_per_second,
            &mut self.texture_duration,
            &mut self.fade_in_out_duration,
            &mut self.flow_speed,
            &mut self.scaling,
            &mut self.block_density,
            &mut self.block_softness,
        ]
    }
}

#[derive(Clone)]
struct PulsingBlockLayer {
    two_tone: TwoTone,
    start_t: f64,
    duration: f64,
    fade_in_out_duration: f64,
    block_density: f64,
}
impl PulsingBlockLayer {
    fn new(layer: &PulsingBlocksConfig, texture: impl Texture, start_t: f64) -> Self {
        let two_tone_config = TwoToneConfig::new(
            (SolidColor::new(palette::named::BLACK.transparent().into()), texture),
            layer.flow_speed.clone(),
            layer.scaling.clone(),
        )
            .init_gradient_offset(0.15)
            .init_gradient_width(layer.block_softness.clone());
        Self {
            two_tone: two_tone_config.into_texture(),
            start_t,
            duration: layer.texture_duration.get(),
            fade_in_out_duration: layer.fade_in_out_duration.get(),
            block_density: layer.block_density.get(),
        }
    }
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> Option<PixelFrame> {
        let t_since_start = t - self.start_t;
        if t_since_start > self.duration {
            None
        } else {
            let transition_amount = smoothstep(0.0, self.fade_in_out_duration, t_since_start)
                - smoothstep(self.duration - self.fade_in_out_duration, self.duration, t_since_start);
            self.two_tone.get_config().gradient_offset().replace(1.0 - (self.block_density * transition_amount));
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
    fn config(&self) -> &dyn ComponentConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut dyn ComponentConfig {
        &mut self.config
    }

    fn component_type(&self) -> &'static str {
        "pixel"
    }
}

impl Texture for PulsingBlocks {
    fn get_blend_mode(&self) -> &BlendModeProperty {
        &self.config.blend_mode
    }

    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        let texture_delay = 1.0 / self.config.textures_per_second.get();
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