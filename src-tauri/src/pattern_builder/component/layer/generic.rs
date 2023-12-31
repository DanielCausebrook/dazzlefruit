use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::layer::{Layer, LayerCore, LayerInfo, LayerTypeInfo};
use crate::pattern_builder::component::layer::io_type::IOType;
use crate::pattern_builder::component::property::PropView;
use crate::pattern_builder::pattern_context::PatternContext;

pub struct GenericLayer<C> where C: LayerCore + Clone {
    info: LayerInfo,
    type_info: LayerTypeInfo,
    input_type: &'static IOType<C::Input>,
    output_type: &'static IOType<C::Output>,
    core: C,
}

impl<C> GenericLayer<C> where C: LayerCore + Clone {
    pub fn new(core: C, type_info: LayerTypeInfo, input_type: &'static IOType<C::Input>, output_type: &'static IOType<C::Output>) -> Self {
        Self {
            info: LayerInfo::default(),
            type_info,
            input_type,
            output_type,
            core,
        }
    }
    pub fn set_type_info(mut self, type_info: LayerTypeInfo) -> Self {
        self.type_info = type_info;
        self
    }
}

impl<C> Clone for GenericLayer<C> where C: LayerCore + Clone {
    fn clone(&self) -> Self {
        Self {
            info: self.info.clone(),
            type_info: self.type_info.clone(),
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
    fn type_info(&self) -> &LayerTypeInfo {
        &self.type_info
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

