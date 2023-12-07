type RandId = string;

type BlendMode = 'Normal'|'AlphaMask';
type ComponentTypeId = 'texture'|'filter'|'texture-generator';

type NumRange = {
    start: number,
    end: number,
}

type AnyComponent = Component<TextureComponentMetadata> |
    Component<FilterComponentMetadata> |
    Component<TextureGeneratorComponentMetadata>;
type Component<T extends ComponentMetadata> = {
    id: RandId,
    type: ComponentTypeId,
    name: PropView<UnsupportedPropMetadata>,
    description: PropView<UnsupportedPropMetadata>,
    data: T,
    properties: [AnyPropView],
}
type ComponentMetadata = Object;
type TextureComponentMetadata = {
    blend_mode: BlendMode,
    opacity: number,
}
type FilterComponentMetadata = {};
type TextureGeneratorComponentMetadata = {};

type DisplayPane = 'Tree'|'Config'|'TreeAndConfig';

type AnyPropView =
    PropView<NumPropMetadata> |
    PropView<ColorPropMetadata> |
    PropView<ComponentPropMetadata> |
    PropView<ComponentVecPropMetadata> |
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
type ColorPropMetadata = {
    type: 'color',
    value: number[],
    data: {},
}
type ComponentPropMetadata = {
    type: 'component',
    value: RandId,
    data: {},
};
type ComponentVecPropMetadata = {
    type: 'component-vec',
    value: RandId[],
    data: {},
};
type UnsupportedPropMetadata = {
    type: 'computed'|'raw',
    value: null,
    data: {},
}


