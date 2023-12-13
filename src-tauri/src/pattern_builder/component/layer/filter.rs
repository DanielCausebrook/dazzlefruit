use dyn_clone::{clone_trait_object, DynClone};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::PixelFrame;
use crate::pattern_builder::component::layer::{Layer, LayerInfo};
use crate::pattern_builder::component::property::PropView;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct FilterLayer {
    info: LayerInfo,
    layer_type: String,
    filter: Box<dyn Filter>,
}

impl FilterLayer {
    pub fn new(filter: impl Filter, info: LayerInfo, layer_type: &str) -> Self {
        Self::new_from_boxed(Box::new(filter), info, layer_type)
    }

    pub fn new_from_boxed(filter: Box<impl Filter>, info: LayerInfo, layer_type: &str) -> Self {
        Self {
            info,
            layer_type: layer_type.to_string(),
            filter,
        }
    }
}

impl Component for FilterLayer {
    fn view_properties(&self) -> Vec<PropView> {
        self.filter.view_properties()
    }

    fn detach(&mut self) {
        self.info.detach();
        self.filter.detach();
    }
}

impl Filter for FilterLayer {
    fn next_frame(&mut self, t: f64, active: PixelFrame, ctx: &PatternContext) -> PixelFrame {
        self.filter.next_frame(t, active, ctx)
    }
}

impl Layer for FilterLayer {
    fn layer_type(&self) -> String {
        self.layer_type.clone()
    }

    fn info(&self) -> &LayerInfo {
        &self.info
    }
}

pub trait Filter: Component + DynClone {
    fn next_frame(&mut self, t: f64, active: PixelFrame, ctx: &PatternContext) -> PixelFrame;
    
    fn into_layer(self, info: LayerInfo) -> FilterLayer where Self: Sized {
        FilterLayer::new(self, info, "filter")
    }
}
clone_trait_object!(Filter);

impl<T> Filter for Box<T> where T: Filter + Clone + ?Sized {
    fn next_frame(&mut self, t: f64, active: PixelFrame, ctx: &PatternContext) -> PixelFrame {
        self.as_mut().next_frame(t, active, ctx)
    }

    fn into_layer(self, info: LayerInfo) -> FilterLayer where Self: Sized {
        (*self).into_layer(info)
    }
}