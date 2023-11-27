use palette::{Mix, WithAlpha};

use crate::{impl_component, impl_component_config};
use crate::pattern_builder::component::ComponentInfo;
use crate::pattern_builder::component::data::{BlendMode, DisplayPane, FrameSize, Pixel, PixelFrame};
use crate::pattern_builder::component::property::cloning::BlendModeProperty;
use crate::pattern_builder::component::property::num::{NumProperty, NumSlider};
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::texture::{Texture, TextureProperty};
use crate::pattern_builder::math_functions::skew_sin;

#[derive(Clone)]
pub struct Wave {
    info: ComponentInfo,
    fg_texture: TextureProperty,
    bg_texture: TextureProperty,
    wave1_speed: NumProperty<f32>,
    wave1_scale: NumProperty<f32>,
    wave1_skew: NumProperty<f32>,
    wave2_speed: NumProperty<f32>,
    wave2_scale: NumProperty<f32>,
    wave2_skew: NumProperty<f32>,
    brightness: NumProperty<f32>,
}

impl Wave {
    pub fn new(fg_texture: impl Texture, bg_texture: impl Texture) -> Self {
        Self {
            info: ComponentInfo::new("Wave"),
            fg_texture: TextureProperty::new(Box::new(fg_texture), PropertyInfo::new("FG").display_pane(DisplayPane::Tree)),
            bg_texture: TextureProperty::new(Box::new(bg_texture), PropertyInfo::new("BG").display_pane(DisplayPane::Tree)),
            wave1_speed: NumProperty::new(9.0, PropertyInfo::new("Wave 1 Speed")).set_slider(Some(NumSlider::new(-30.0..30.0, 0.1))),
            wave1_scale: NumProperty::new(28.5, PropertyInfo::new("Wave 1 Scale")).set_slider(Some(NumSlider::new(0.0..50.0, 0.5))),
            wave1_skew: NumProperty::new(0.6, PropertyInfo::new("Wave 1 Skew")).set_slider(Some(NumSlider::new(-1.0..1.0, 0.01))),
            wave2_speed: NumProperty::new(-11.5, PropertyInfo::new("Wave 2 Speed")).set_slider(Some(NumSlider::new(-30.0..30.0, 0.1))),
            wave2_scale: NumProperty::new(34.0, PropertyInfo::new("Wave 2 Scale")).set_slider(Some(NumSlider::new(0.0..50.0, 0.5))),
            wave2_skew: NumProperty::new(-0.5, PropertyInfo::new("Wave 2 Skew")).set_slider(Some(NumSlider::new(-1.0..1.0, 0.01))),
            brightness: NumProperty::new(1.0, PropertyInfo::new("Brightness")).set_slider(Some(NumSlider::new(0.0..1.0, 0.05)))
        }
    }
}

impl_component_config!(self: Wave, self.info, [
    self.fg_texture,
    self.bg_texture,
    self.wave1_speed,
    self.wave1_scale,
    self.wave1_skew,
    self.wave2_speed,
    self.wave2_scale,
    self.wave2_skew,
    self.brightness,
]);

impl_component!(self: Wave, *self, "pixel");

impl Texture for Wave {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        let fg = self.fg_texture.write().next_frame(t, num_pixels);
        let bg = self.bg_texture.write().next_frame(t, num_pixels);
        let t = t as f32;
        (0..num_pixels).map(|x_int| {
            let x = x_int as f32;
            // let t = t + ((x/10.0 + t).sin() / 2.0);
            let mut wave1_val = skew_sin(self.wave1_skew.get(), 1.0, (x + self.wave1_speed.get() * t) / self.wave1_scale.get());
            let mut wave2_val = skew_sin(self.wave2_skew.get(), 1.0, (x + self.wave2_speed.get() * t) / self.wave2_scale.get());
            wave1_val = wave1_val / 2.0 + 0.5;
            wave2_val = wave2_val / 2.0 + 0.5;
            let wave = (wave1_val + wave2_val) / 2.0;
            let c1: Pixel = *bg.get(x_int as usize).unwrap();
            c1.mix(*fg.get(x_int as usize).unwrap(), wave)
                .with_alpha(self.brightness.get().powf(2.0))
        }).collect()
    }
}

