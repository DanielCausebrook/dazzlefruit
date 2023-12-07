use std::marker;
use palette::WithAlpha;
use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::{Component, LayerInfo};
use crate::pattern_builder::component::data::{FrameSize, PixelFrame};
use crate::pattern_builder::component::filter::{Filter, FilterLayer};
use crate::pattern_builder::component::property::{PropView};
use crate::pattern_builder::component::texture::{Texture, TextureLayer};
use crate::pattern_builder::component::texture_generator::{TextureGenerator, TextureGeneratorLayer};

pub trait EmptyType: Send + Sync + Clone + 'static {
    fn get_component_type() -> &'static str;
}

#[derive(Clone)]
pub struct EmptyTexture {}
impl EmptyType for EmptyTexture {
    fn get_component_type() -> &'static str { "empty_pixel" }
}
#[derive(Clone)]
pub struct EmptyFilter {}
impl EmptyType for EmptyFilter {
    fn get_component_type() -> &'static str { "empty_filter" }
}
#[derive(Clone)]
pub struct EmptyTextureGenerator {}
impl EmptyType for EmptyTextureGenerator {
    fn get_component_type() -> &'static str { "empty_producer" }
}

#[derive(Clone)]
pub struct Empty<T: EmptyType> {
    ty: marker::PhantomData<T>,
}

impl<T: EmptyType> Empty<T> {
    pub fn new() -> Self {
        Self {
            ty: marker::PhantomData::default(),
        }
    }
}

impl Empty<EmptyTexture> {
    pub fn new_texture_component() -> TextureLayer {
        Self::new().into_layer(LayerInfo::new("Empty"))
    }
}

impl Empty<EmptyFilter> {
    pub fn new_filter_component() -> FilterLayer {
        Self::new().into_layer(LayerInfo::new("Empty"))
    }
}

impl Empty<EmptyTextureGenerator> {
    pub fn new_texture_generator_component() -> TextureGeneratorLayer {
        Self::new().into_layer(LayerInfo::new("Empty"))
    }
}

impl<T> Component for Empty<T> where T: EmptyType {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!()
    }

    fn detach(&mut self) {
        fork_properties!();
    }
}

impl TextureGenerator for Empty<EmptyTextureGenerator> {
    fn next_texture(&mut self) -> TextureLayer {
        Empty::<EmptyTexture>::new().into_layer(LayerInfo::new("Empty"))
    }
}

impl Texture for Empty<EmptyTexture> {
    fn next_frame(&mut self, _t: f64, num_pixels: FrameSize) -> PixelFrame {
        vec![palette::named::BLACK.into_linear().transparent(); num_pixels as usize]
    }
}

impl Filter for Empty<EmptyFilter> {
    fn next_frame(&mut self, _t: f64, active: PixelFrame) -> PixelFrame { active }
}