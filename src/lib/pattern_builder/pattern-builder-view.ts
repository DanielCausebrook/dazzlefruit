import {invoke} from "@tauri-apps/api/tauri";

export class PatternBuilderView {
    #patterns: Map<RandId, LazyPattern>;
    selectedPatternId: RandId|null;

    constructor(data) {
        this.#patterns = new Map();
        for (const patternInfo of data) {
            const info = new PatternInfo(patternInfo.id, patternInfo.name);
            this.#patterns.set(info.id, new LazyPattern(info));
        }
        this.selectedPatternId = null;
    }

    getPatternsInfo(): PatternInfo[] {
        return Array.from(this.#patterns.values()).map(lazy => lazy.info);
    }

    async getPatternView(patternId: RandId): Promise<PatternView|null> {
        let lazyPattern = this.#patterns.get(patternId) ?? null;
        if (lazyPattern === null) {
            console.error(`No pattern with id ${patternId}.`);
            return null;
        }
        if (lazyPattern.view === null) {
            let patternViewData = JSON.parse(await invoke("view_pattern", {id: patternId}));
            lazyPattern.view = new PatternView(lazyPattern.info, patternViewData);
        }
        return lazyPattern.view;
    }

    getSelectedPatternId(): RandId|null {
        return this.selectedPatternId;
    }

    setSelectedPattern(patternId: RandId) {
        this.selectedPatternId = patternId;
    }
}

class LazyPattern {
    info: PatternInfo;
    view: PatternView|null;

    constructor(info: PatternInfo) {
        this.info = info;
        this.view = null;
    }
}

export class PatternInfo {
    readonly id: RandId;
    readonly name: string;

    constructor(id: RandId, name: string) {
        this.id = id;
        this.name = name;
    }
}

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

    getSelectedLayerId(): RandId|null {
        return this.selectedLayerId;
    }

    setSelectedLayer(layerId: RandId) {
        this.selectedLayerId = layerId;
    }
}