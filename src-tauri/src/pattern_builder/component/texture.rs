use std::collections::HashMap;
use dyn_clone::{clone_trait_object, DynClone};
use crate::pattern_builder::component::{Layer, Component, LayerInfo};
use crate::pattern_builder::component::data::{BlendMode, FrameSize, PixelFrame};
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::property::num::NumPropCore;
use crate::pattern_builder::component::property::raw::RawPropCore;
use crate::pattern_builder::component::property::PropertyInfo;

use crate::pattern_builder::library::core::RawPixels;

#[derive(Clone)]
pub struct TextureLayer {
    info: LayerInfo,
    texture: Box<dyn Texture>,
    blend_mode: Prop<BlendMode>,
    opacity: Prop<f32>,
    cache: Option<PixelFrame>,
}

impl TextureLayer {
    pub fn new(texture: impl Texture, info: LayerInfo) -> Self {
        Self::new_from_boxed(Box::new(texture), info)
    }

    pub fn new_from_boxed(texture: Box<impl Texture>, info: LayerInfo) -> Self {
        Self {
            info,
            texture,
            blend_mode: RawPropCore::new(BlendMode::Normal).into_prop(PropertyInfo::new("Blend Mode")),
            opacity: NumPropCore::new_slider(1.0, 0.0..1.0, 0.01).into_prop(PropertyInfo::new("Opacity")),
            cache: None,
        }
    }

    pub fn blend_mode(&self) -> &Prop<BlendMode> {
        &self.blend_mode
    }

    pub fn opacity(&self) -> &Prop<f32> {
        &self.opacity
    }
}

impl Component for TextureLayer {
    fn view_properties(&self) -> Vec<PropView> {
        self.texture.view_properties()
    }

    fn detach(&mut self) {
        self.info.detach();
        self.texture.detach();
        self.blend_mode = self.blend_mode.fork();
        self.opacity = self.opacity.fork();
    }
}

impl Layer for TextureLayer {
    fn type_str(&self) -> String {
        "texture".to_string()
    }

    fn info(&self) -> &LayerInfo {
        &self.info
    }

    fn view_data(&self) -> HashMap<String, Box<dyn erased_serde::Serialize + 'static>> {
        HashMap::from([
            ("blend_mode".to_string(), Box::new(*self.blend_mode.read()) as Box<dyn erased_serde::Serialize>),
            ("opacity".to_string(), Box::new(*self.opacity.read()) as Box<dyn erased_serde::Serialize>),
        ])
    }
}

impl Texture for TextureLayer {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        self.texture.next_frame(t, num_pixels)
    }
}

pub trait Texture: Component + DynClone + Send + Sync {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame;

    fn into_layer(self, info: LayerInfo) -> TextureLayer where Self: Sized {
        TextureLayer::new(self, info)
    }
}
clone_trait_object!(Texture);

impl<T> Texture for Box<T> where T: Texture + Clone + ?Sized {
    fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
        self.as_mut().next_frame(t, num_pixels)
    }

    fn into_layer(self, info: LayerInfo) -> TextureLayer where Self: Sized {
        TextureLayer::new_from_boxed(self, info)
    }
}

// impl Texture for SharedComponent<TextureComponent> {
//     fn next_frame(&mut self, t: f64, num_pixels: FrameSize) -> PixelFrame {
//         self.write().next_frame(t, num_pixels)
//     }
// 
//     fn view_properties(&self) -> Vec<PropView> {
//         self.read().view_properties()
//     }
// 
//     fn for_each_child_component<'a>(&self, func: &(dyn FnMut(&dyn Component) + 'a)) {
//         self.read().for_each_child_component(func)
//     }
//     
//     fn detach(&mut self) {
//         self.write().detach()
//     }
// }

impl From<PixelFrame> for Box<dyn Texture> {
    fn from(value: PixelFrame) -> Self {
        Box::new(RawPixels::new(value))
    }
}
