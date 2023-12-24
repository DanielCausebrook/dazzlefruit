import {PatternInfo} from "./pattern-builder-view";

export class PatternView {
    readonly info: PatternInfo;
    #rootStack: PropView<LayerStackPropMetadata>;
    #layerConfigs: Map<RandId, AnyLayer>;
    preview_colors: string[];
    selectedLayerId: RandId|null;

    constructor(info: PatternInfo, data) {
        this.info = info;
        this.#rootStack = data.root_stack;
        this.#layerConfigs = new Map<RandId, AnyLayer>(Object.entries(data.components));
        this.selectedLayerId = null;
        this.preview_colors = [];
    }

    getRootStack(): PropView<LayerStackPropMetadata> {
        return this.#rootStack;
    }

    getLayerView(layerId: RandId): AnyLayer|null {
        return this.#layerConfigs.get(layerId) ?? null;
    }
}
