type RandId = string;

type BlendMode = 'Normal'|'AlphaMask';
type LayerTypeId = 'texture'|'filter'|'texture-generator';

type NumRange = {
    start: number,
    end: number,
}

type AnyLayer = Layer<TextureLayerMetadata> |
    Layer<FilterLayerMetadata> |
    Layer<TextureGeneratorLayerMetadata>;
type Layer<T extends LayerMetadata> = {
    id: RandId,
    type: LayerTypeId,
    name: PropView<UnsupportedPropMetadata>,
    description: PropView<UnsupportedPropMetadata>,
    data: T,
    properties: [AnyPropView],
}
type LayerMetadata = Object;
type TextureLayerMetadata = {
    blend_mode: BlendMode,
    opacity: number,
}
type FilterLayerMetadata = {};
type TextureGeneratorLayerMetadata = {};

type DisplayPane = 'Tree'|'Config'|'TreeAndConfig';

type AnyPropView =
    PropView<NumPropMetadata> |
    PropView<NumVecPropMetadata> |
    PropView<ColorPropMetadata> |
    PropView<LayerPropMetadata> |
    PropView<LayerVecPropMetadata> |
    PropView<LayerStackPropMetadata> |
    PropView<UnsupportedPropMetadata>;
type PropView<T extends PropMetadata> = {
    id: RandId,
    type: T['type'],
    name: string|null,
    description: string|null,
    display_pane: DisplayPane,
    value: T['value'],
    data: T['data'],
}
type PropMetadata = {
    type: string,
    value: any,
    data: Object,
}
type NumPropMetadata = {
    type: 'num',
    value: number,
    data: {
        slider: null | { range: NumRange, step: number },
    }
};
type NumVecPropMetadata = {
    type: 'num-vec',
    value: [number],
    data: {
        sliders: [null | { range: NumRange, step: number }],
    }
};
type ColorPropMetadata = {
    type: 'color',
    value: number[],
    data: {},
}
type LayerPropMetadata = {
    type: 'layer',
    value: RandId,
    data: {},
};
type LayerVecPropMetadata = {
    type: 'layer-vec',
    value: RandId[],
    data: {},
};
type LayerStackPropMetadata = {
    type: 'layer-vec',
    value: RandId[],
    data: {
        errors: {
            layer_id: RandId|null,
            from_type_name: string,
            into_type_name: string
        }[]

    },
};
type UnsupportedPropMetadata = {
    type: 'computed'|'raw',
    value: null,
    data: {},
}


