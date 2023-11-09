type RandId = string;

type BlendMode = 'Normal'|'AlphaMask';

type NumRange = {
    start: number,
    end: number,
}

type LayerConfig = {
    id: RandId,
    layer_type: 'pixel'|'filter'|'group'|'producer',
    name: SimpleProperty<string>,
    description: SimpleProperty<string|null>,
    blend_mode: SimpleProperty<BlendMode>,
    properties: [Property],
}

type Property = SimpleProperty<string|(string|null)|boolean|BlendMode|[number]|RandId|[RandId]>|NumProperty|OptionNumProperty;
type PropertyType = 'string'|'optionString'|'bool'|'blendMode'|'color'|'pixelLayer'|'layerVec'|'num'|'optionNum'|'pixelBlueprintVec';
type PaneType = 'Tree'|'Config'|'TreeAndConfig';

type SimpleProperty<T> = {
    id: RandId,
    name: string,
    description: string|null,
    display_pane: PaneType,
    property_type: PropertyType,
    value: T,
}

type NumProperty = {
    id: RandId,
    name: string,
    description: string|null,
    display_pane: PaneType,
    property_type: PropertyType,
    value: number,
    slider: {
        range: NumRange,
        step: number,
    }|null,
}

type OptionNumProperty = {
    id: RandId,
    name: string,
    description: string|null,
    display_pane: PaneType,
    property_type: PropertyType,
    value: number|null,
    range: NumRange,
    step: number,
}

