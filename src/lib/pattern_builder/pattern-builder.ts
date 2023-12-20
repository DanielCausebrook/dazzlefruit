export class PatternBuilder {
    rootId: RandId;
    rootStack: PropView<LayerStackPropMetadata>;
    layerConfigs: Map<RandId, AnyComponent>;
    selectedId: RandId|null;

    constructor(data) {
        this.rootStack = data.root_stack;
        this.layerConfigs = new Map<RandId, AnyComponent>(Object.entries(data.components));
        console.log(this.layerConfigs);
        this.selectedId = null;
    }

    getRootStack(): PropView<LayerStackPropMetadata> {
        return this.rootStack;
    }

    getLayerConfig(layerId: RandId): AnyComponent|null {
        return this.layerConfigs.get(layerId) ?? null;
    }

    getSelectedLayerId(): RandId|null {
        return this.selectedId;
    }

    // getSelectedLayerConfig(): LayerConfig|null {
    //     return this.layerConfigs.get(this.selectedId) ?? null;
    // }

    setSelectedLayer(layerId: RandId) {
        this.selectedId = layerId;
    }
}