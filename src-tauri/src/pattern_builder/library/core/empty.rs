use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::frame::{ColorPixel, Frame};
use crate::pattern_builder::component::layer::{LayerCore, LayerTypeInfo};
use crate::pattern_builder::component::property::PropView;
use crate::pattern_builder::component::layer::texture::{TextureLayer};
use crate::pattern_builder::pattern_context::PatternContext;

pub fn empty_texture_layer() -> TextureLayer {
    TextureLayer::new(Empty::<EmptyTexture>::new(), LayerTypeInfo::new("Empty"))
}

struct EmptyTexture {}
struct EmptyFilter {}

pub struct Empty<T: Send + Sync + 'static> (std::marker::PhantomData<T>);

impl<T> Empty<T> where T: Send + Sync + 'static {
    pub fn new() -> Self { Self (Default::default()) }
}

impl<T> Clone for Empty<T> where T: Send + Sync + 'static {
    fn clone(&self) -> Self {
       Self(self.0.clone())
    }
}

impl LayerCore for Empty<EmptyTexture> {
    type Input = ();
    type Output = Frame<ColorPixel>;
    fn next(&mut self, _: (), _t: f64, ctx: &PatternContext) -> Frame<ColorPixel> {
        Frame::<ColorPixel>::empty(ctx.num_pixels())
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!()
    }

    fn detach(&mut self) {
        fork_properties!();
    }
}

impl LayerCore for Empty<EmptyFilter> {
    type Input = Frame<ColorPixel>;
    type Output = Frame<ColorPixel>;
    fn next(&mut self, active: Frame<ColorPixel>, _t: f64, _ctx: &PatternContext) -> Frame<ColorPixel> {
        active
    }

    fn view_properties(&self) -> Vec<PropView> {
        view_properties!()
    }

    fn detach(&mut self) {
        fork_properties!();
    }
}