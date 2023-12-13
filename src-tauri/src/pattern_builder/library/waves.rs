use palette::{Mix};

use crate::pattern_builder::component::data::{DisplayPane, Pixel, PixelFrame};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::component::{TexturePropCore};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::layer::texture::{Texture, TextureLayer};
use crate::pattern_builder::math_functions::skew_sin;
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Wave {
    fg_texture: Prop<TextureLayer>,
    bg_texture: Prop<TextureLayer>,
    wave1_speed: Prop<f32>,
    wave1_scale: Prop<f32>,
    wave1_skew: Prop<f32>,
    wave2_speed: Prop<f32>,
    wave2_scale: Prop<f32>,
    wave2_skew: Prop<f32>,
}

impl Wave {
    pub fn new(fg_texture: TextureLayer, bg_texture: TextureLayer) -> Self {
        Self {
            fg_texture: TexturePropCore::new(fg_texture).into_prop(PropertyInfo::new("FG").set_display_pane(DisplayPane::Tree)),
            bg_texture: TexturePropCore::new(bg_texture).into_prop(PropertyInfo::new("BG").set_display_pane(DisplayPane::Tree)),
            wave1_speed: NumPropCore::new_slider(9.0, -30.0..30.0, 0.1).into_prop(PropertyInfo::new("Wave 1 Speed")),
            wave1_scale: NumPropCore::new_slider(28.5, 0.0..50.0, 0.5).into_prop(PropertyInfo::new("Wave 1 Scale")),
            wave1_skew: NumPropCore::new_slider(0.6, -1.0..1.0, 0.01).into_prop(PropertyInfo::new("Wave 1 Skew")),
            wave2_speed: NumPropCore::new_slider(-11.5, -30.0..30.0, 0.1).into_prop(PropertyInfo::new("Wave 2 Speed")),
            wave2_scale: NumPropCore::new_slider(34.0, 0.0..50.0, 0.5).into_prop(PropertyInfo::new("Wave 2 Scale")),
            wave2_skew: NumPropCore::new_slider(-0.5, -1.0..1.0, 0.01).into_prop(PropertyInfo::new("Wave 2 Skew")),
        }
    }
}

impl Component for Wave {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(
            self.fg_texture,
            self.bg_texture,
            self.wave1_speed,
            self.wave1_scale,
            self.wave1_skew,
            self.wave2_speed,
            self.wave2_scale,
            self.wave2_skew,
        )
    }

    fn detach(&mut self) {
        fork_properties!(
            self.fg_texture,
            self.bg_texture,
            self.wave1_speed,
            self.wave1_scale,
            self.wave1_skew,
            self.wave2_speed,
            self.wave2_scale,
            self.wave2_skew,
        );
    }
}

impl Texture for Wave {
    fn next_frame(&mut self, t: f64, ctx: &PatternContext) -> PixelFrame {
        let fg = self.fg_texture.write().next_frame(t, ctx);
        let bg = self.bg_texture.write().next_frame(t, ctx);
        let t = t as f32;
        (0..ctx.num_pixels()).map(|x_int| {
            let x = x_int as f32;
            // let t = t + ((x/10.0 + t).sin() / 2.0);
            let mut wave1_val = skew_sin(*self.wave1_skew.read(), 1.0, (x + *self.wave1_speed.read() * t) / *self.wave1_scale.read());
            let mut wave2_val = skew_sin(*self.wave2_skew.read(), 1.0, (x + *self.wave2_speed.read() * t) / *self.wave2_scale.read());
            wave1_val = wave1_val / 2.0 + 0.5;
            wave2_val = wave2_val / 2.0 + 0.5;
            let wave = (wave1_val + wave2_val) / 2.0;
            let c1: Pixel = *bg.get(x_int as usize).unwrap();
            c1.mix(*fg.get(x_int as usize).unwrap(), wave)
        }).collect()
    }
}

