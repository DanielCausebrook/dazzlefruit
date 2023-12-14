use palette::WithAlpha;

use crate::{fork_properties, view_properties};
use crate::pattern_builder::component::Component;
use crate::pattern_builder::component::data::{DisplayPane, PixelFrame};
use crate::pattern_builder::component::layer::filter::{Filter, FilterLayer};
use crate::pattern_builder::component::layer::{Layer, LayerInfo, LayerView};
use crate::pattern_builder::component::property::PropertyInfo;
use crate::pattern_builder::component::property::{Prop, PropCore, PropView};
use crate::pattern_builder::component::layer::texture::{Texture, TextureLayer};
use crate::pattern_builder::component::property::layer::LayerVecPropCore;
use crate::pattern_builder::pattern_context::PatternContext;

#[derive(Clone)]
pub struct Group {
    layers: Prop<Vec<GroupedLayer>>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            layers: LayerVecPropCore::new().into_prop(PropertyInfo::unnamed().set_display_pane(DisplayPane::Tree)),
        }
    }

    pub fn add_texture(&self, texture: TextureLayer) {
        self.layers.write().push(GroupedLayer::Texture(texture))
    }

    pub fn add_filter(&self, filter: FilterLayer) {
        self.layers.write().push(GroupedLayer::Filter(filter))
    }
}

impl Component for Group {
    fn view_properties(&self) -> Vec<PropView> {
        view_properties!(self.layers)
    }

    fn detach(&mut self) {
        fork_properties!(self.layers);
    }
}

impl Texture for Group {
    fn next_frame(&mut self, t: f64, ctx: &PatternContext) -> PixelFrame {
        let mut pixel_data = self.layers.write().iter_mut()
            .fold(None, |active_option, layer| {
                match layer {
                    GroupedLayer::Texture(pixel_layer) => {
                        let pixel_data: PixelFrame = pixel_layer.next_frame(t, ctx).into_iter().map(|pixel| pixel.with_alpha(pixel.alpha * *pixel_layer.opacity().read())).collect();
                        match active_option {
                            Some(active) => Some(pixel_data.blend(active, *pixel_layer.blend_mode().read())),
                            None => Some(pixel_data),
                        }
                    },
                    GroupedLayer::Filter(filter_layer) => {
                        active_option.map(|active| filter_layer.next_frame(t, active, ctx))
                    }
                }
            })
            .unwrap_or_else(|| vec![].into());
        pixel_data.resize_with_transparent(ctx.num_pixels());
        pixel_data
    }

    fn into_layer(self, info: LayerInfo) -> TextureLayer where Self: Sized {
        TextureLayer::new(self, info, "group")
    }
}

#[derive(Clone)]
enum GroupedLayer {
    Texture(TextureLayer),
    Filter(FilterLayer),
}

impl Component for GroupedLayer {
    fn view_properties(&self) -> Vec<PropView> {
        match self {
            GroupedLayer::Texture(l) => l.view_properties(),
            GroupedLayer::Filter(l) => l.view_properties(),
        }
    }

    fn detach(&mut self) {
        match self {
            GroupedLayer::Texture(l) => l.detach(),
            GroupedLayer::Filter(l) => l.detach(),
        }
    }
}

impl Layer for GroupedLayer {
    fn layer_type(&self) -> String {
        match self {
            GroupedLayer::Texture(l) => l.layer_type(),
            GroupedLayer::Filter(l) => l.layer_type(),
        }
    }

    fn info(&self) -> &LayerInfo {
        match self {
            GroupedLayer::Texture(l) => l.info(),
            GroupedLayer::Filter(l) => l.info(),
        }
    }

    fn view(&self) -> LayerView {
        match self {
            GroupedLayer::Texture(l) => l.view(),
            GroupedLayer::Filter(l) => l.view(),
        }
    }
}

