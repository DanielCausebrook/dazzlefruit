use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::layer::{Layer, LayerCore, LayerInfo, LayerType};
use crate::pattern_builder::component::layer::io_type::IOType;
use crate::pattern_builder::component::property::PropView;
use crate::pattern_builder::pattern_context::PatternContext;

pub struct GenericLayer<C> where C: LayerCore + Clone {
    info: LayerInfo,
    layer_type: LayerType,
    input_type: &'static IOType<C::Input>,
    output_type: &'static IOType<C::Output>,
    core: C,
}

impl<C> GenericLayer<C> where C: LayerCore + Clone {
    pub fn new(core: C, info: LayerInfo, input_type: &'static IOType<C::Input>, output_type: &'static IOType<C::Output>) -> Self {
        Self {
            info,
            layer_type: LayerType::Generic,
            input_type,
            output_type,
            core,
        }
    }
    pub fn set_layer_type(mut self, layer_type: LayerType) -> Self {
        self.layer_type = layer_type;
        self
    }
}

impl<C> Clone for GenericLayer<C> where C: LayerCore + Clone {
    fn clone(&self) -> Self {
        Self {
            info: self.info.clone(),
            layer_type: self.layer_type.clone(),
            input_type: self.input_type,
            output_type: self.output_type,
            core: self.core.clone(),
        }
    }
}

impl<C> Component for GenericLayer<C> where C: LayerCore + Clone {
    fn view_properties(&self) -> Vec<PropView> {
        self.core.view_properties()
    }

    fn detach(&mut self) {
        self.core.detach();
        self.info.detach();
    }
}

impl<C> LayerCore for GenericLayer<C> where C: LayerCore + Clone {
    type Input = C::Input;
    type Output = C::Output;

    fn next(&mut self, input: Self::Input, t: f64, ctx: &PatternContext) -> Self::Output {
        self.core.next(input, t, ctx)
    }
}

impl<C> Layer for GenericLayer<C> where C: LayerCore + Clone {
    fn layer_type(&self) -> LayerType {
        self.layer_type
    }

    fn input_type(&self) -> &IOType<Self::Input> {
        &self.input_type
    }

    fn output_type(&self) -> &IOType<Self::Output> {
        &self.output_type
    }

    fn info(&self) -> &LayerInfo {
        &self.info
    }
}

