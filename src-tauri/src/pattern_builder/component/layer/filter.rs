use dyn_clone::{clone_trait_object, DynClone};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::PixelFrame;
use crate::pattern_builder::component::layer::{Layer, LayerInfo};
use crate::pattern_builder::component::property::PropView;

#[derive(Clone)]
pub struct FilterLayer {
    info: LayerInfo,
    filter: Box<dyn Filter>,
}

impl FilterLayer {
    pub fn new(filter: impl Filter, info: LayerInfo) -> Self {
        Self::new_from_boxed(Box::new(filter), info)
    }

    pub fn new_from_boxed(filter: Box<impl Filter>, info: LayerInfo) -> Self {
        Self {
            info,
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
    fn next_frame(&mut self, t: f64, active: PixelFrame) -> PixelFrame {
        self.filter.next_frame(t, active)
    }
}

impl Layer for FilterLayer {
    fn type_str(&self) -> String {
        "filter".to_string()
    }

    fn info(&self) -> &LayerInfo {
        &self.info
    }
}

pub trait Filter: Component + DynClone {
    fn next_frame(&mut self, t: f64, active: PixelFrame) -> PixelFrame;
    
    fn into_layer(self, info: LayerInfo) -> FilterLayer where Self: Sized {
        FilterLayer::new(self, info)
    }
}
clone_trait_object!(Filter);

impl<T> Filter for Box<T> where T: Filter + Clone + ?Sized {
    fn next_frame(&mut self, t: f64, active: PixelFrame) -> PixelFrame {
        self.as_mut().next_frame(t, active)
    }

    fn into_layer(self, info: LayerInfo) -> FilterLayer where Self: Sized {
        FilterLayer::new_from_boxed(self, info)
    }
}