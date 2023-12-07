export class PatternBuilder {
    rootId: RandId;
    layerConfigs: Map<RandId, AnyComponent>;
    selectedId: RandId|null;

    constructor(data) {
        this.rootId = data.root_id;
        this.layerConfigs = new Map<RandId, AnyComponent>(Object.entries(data.components));
        console.log(this.layerConfigs);
        this.selectedId = null;
    }

    getRootId(): RandId {
        return this.rootId;
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