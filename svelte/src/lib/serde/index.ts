import type { Deserializer } from "./serde/deserializer";

export function deserializeVec<T>(deserializer: Deserializer, deserialize: (d: Deserializer) => T): T[] {
    const len = deserializer.deserializeLen();
    const vector = new Array(len);

    for (let i = 0; i < len; i++) {
        vector[i] = deserialize(deserializer);
    }

    return vector;
}

export function deserializeOption<T>(deserializer: Deserializer, deserialize: (d: Deserializer) => T): T | null {
    const tag = deserializer.deserializeOptionTag();

    if (!tag) {
        return null;
    }

    return deserialize(deserializer);
}
