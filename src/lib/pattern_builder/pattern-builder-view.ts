export class PatternBuilderView {
    #patterns: Map<RandId, PatternInfo>;
    selectedPatternId: RandId|null;

    constructor(data) {
        this.#patterns = new Map();
        for (const patternInfo of data) {
            const info = new PatternInfo(patternInfo.id, patternInfo.name);
            this.#patterns.set(info.id, info);
        }
        this.selectedPatternId = null;
    }

    getPatternsInfo(): PatternInfo[] {
        return Array.from(this.#patterns.values());
    }

    getPatternInfo(patternId: RandId): PatternInfo|null {
        return this.#patterns.get(patternId) ?? null;
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