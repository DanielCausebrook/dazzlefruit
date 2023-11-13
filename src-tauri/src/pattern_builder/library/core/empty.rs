use std::marker;
use palette::WithAlpha;
use crate::{impl_component, impl_component_config};
use crate::pattern_builder::component::{ComponentInfo};
use crate::pattern_builder::component::data::{FrameSize, PixelFrame};
use crate::pattern_builder::component::filter::Filter;
use crate::pattern_builder::component::texture::Texture;
use crate::pattern_builder::component::property::cloning::BlendModeProperty;
use crate::pattern_builder::component::texture_generator::TextureGenerator;

pub trait EmptyType: Send + Sync + Clone + 'static {
    fn get_component_type() -> &'static str;
}

#[derive(Clone)]
pub struct EmptyPixelLayer {}
impl EmptyType for EmptyPixelLayer {
    fn get_component_type() -> &'static str { "empty_pixel" }
}
#[derive(Clone)]
pub struct EmptyFilterLayer {}
impl EmptyType for EmptyFilterLayer {
    fn get_component_type() -> &'static str { "empty_filter" }
}
#[derive(Clone)]
pub struct EmptyTextureGenerator {}
impl EmptyType for EmptyTextureGenerator {
    fn get_component_type() -> &'static str { "empty_producer" }
}

#[derive(Clone)]
pub struct Empty<T: EmptyType> {
    info: ComponentInfo,
    blend_mode: BlendModeProperty,
    ty: marker::PhantomData<T>,
}

impl<T: EmptyType> Empty<T> {
    pub fn new() -> Self {
        Self {
            info: ComponentInfo::new("Empty"),
            blend_mode: BlendModeProperty::default(),
            ty: marker::PhantomData::default(),
        }
    }
}

impl_component_config!(<T: EmptyType> self: Empty<T>, self.info, []);

impl_component!(<T: EmptyType> self: Empty<T>, *self, T::get_component_type());

impl TextureGenerator for Empty<EmptyTextureGenerator> {
    fn next_texture(&mut self) -> Box<dyn Texture> {
        Box::new(Empty::new())
    }
}

impl Texture for Empty<EmptyPixelLayer> {
    fn get_blend_mode(&self) -> &BlendModeProperty {
        &self.blend_mode
    }

    fn next_frame(&mut self, _t: f64, num_pixels: FrameSize) -> PixelFrame {
        vec![palette::named::BLACK.into_linear().transparent(); num_pixels as usize]
    }
}

impl Filter for Empty<EmptyFilterLayer> {
    fn next_frame(&mut self, _t: f64, active: PixelFrame) -> PixelFrame { active }
}