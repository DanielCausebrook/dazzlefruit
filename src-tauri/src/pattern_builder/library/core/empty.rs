use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::PixelFrame;
use crate::pattern_builder::component::layer::filter::{Filter, FilterLayer};
use crate::pattern_builder::component::layer::LayerInfo;
use crate::pattern_builder::component::property::PropView;
use crate::pattern_builder::component::layer::texture::{Texture, TextureLayer};
use crate::pattern_builder::component::layer::texture_generator::{TextureGenerator, TextureGeneratorLayer};
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Empty {}

impl Empty {
    pub fn new() -> Self { Self {} }
    pub fn new_texture_layer() -> TextureLayer {
        Texture::into_layer(Self::new(), LayerInfo::new("Empty"))
    }
    pub fn new_filter_layer() -> FilterLayer {
        Filter::into_layer(Self::new(), LayerInfo::new("Empty"))
    }
    pub fn new_texture_generator_layer() -> TextureGeneratorLayer {
        TextureGenerator::into_layer(Self::new(), LayerInfo::new("Empty"))
    }
}

impl Component for Empty {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!()
    }

    fn detach(&mut self) {
        fork_properties!();
    }
}

impl Texture for Empty {
    fn next_frame(&mut self, _t: f64, ctx: &PatternContext) -> PixelFrame {
        PixelFrame::empty(ctx.num_pixels())
    }

    fn into_layer(self, info: LayerInfo) -> TextureLayer where Self: Sized {
        TextureLayer::new(self, info, "texture-empty")
    }
}

impl Filter for Empty {
    fn next_frame(&mut self, _t: f64, active: PixelFrame, _ctx: &PatternContext) -> PixelFrame {
        active
    }

    fn into_layer(self, info: LayerInfo) -> FilterLayer where Self: Sized {
        FilterLayer::new(self, info, "filter-empty")
    }
}

impl TextureGenerator for Empty {
    fn next_texture(&mut self) -> TextureLayer {
        Empty::new_texture_layer()
    }

    fn into_layer(self, info: LayerInfo) -> TextureGeneratorLayer where Self: Sized {
        TextureGeneratorLayer::new(self, info, "texture-generator-empty")
    }
}