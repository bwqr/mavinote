import { deserializeOption } from "./serde";
import type { Deserializer } from "./serde/serde/deserializer";

export interface Folder {
    id: number;
    name: String;
    state: State;
}

export function deserializeFolder(deserializer: Deserializer): Folder {
    return {
        id: deserializer.deserializeI32(),
        name: deserializer.deserializeStr(),
        state: deserializeState(deserializer),
    };
}

export interface Note {
    id: number;
    folderId: number;
    commitId: number;
    title: string | null;
    text: string;
    state: State;
}

export function deserializeNote(deserializer: Deserializer): Note {
    return {
        id: deserializer.deserializeI32(),
        folderId: deserializer.deserializeI32(),
        commitId: deserializer.deserializeI32(),
        title: deserializeOption(deserializer, (d) => d.deserializeStr()),
        text: deserializer.deserializeStr(),
        state: deserializeState(deserializer),
    };
}

export enum State {
    Clean,
    Modified,
    Deleted
}

function deserializeState(deserializer: Deserializer): State {
    const index = deserializer.deserializeVariantIndex();

    switch (index) {
        case 0:
            return State.Clean;
        case 1:
            return State.Modified;
        case 2:
            return State.Deleted;
        default:
            throw new Error('Unknown index for State');
    }
}

export class ReaxError {
    static deserialize(deserializer: Deserializer): ReaxError {
        const index = deserializer.deserializeVariantIndex();

        switch (index) {
            case 0:
                return HttpError.deserialize(deserializer);
            case 1:
                return MessageError.deserialize(deserializer);
            default:
                throw new Error('Unknown variant on ReaxError');
        }
    }
}

export abstract class HttpError extends ReaxError {
    abstract name: string;

    static NoConnection = new class extends HttpError { name = 'NoConnection' };
    static UnexpectedResponse = new class extends HttpError { name = 'UnexpectedResponse' };
    static Unknown = new class extends HttpError { name = 'Unknown' };

    static deserialize(deserializer: Deserializer): HttpError {
        const index = deserializer.deserializeVariantIndex();

        switch (index) {
            case 0:
                return this.NoConnection;
            case 1:
                return this.UnexpectedResponse;
            case 2:
                return new Unauthorized(deserializeOption(deserializer, (d) => d.deserializeI32()));
            case 3:
                return this.Unknown;
            default:
                throw new Error('Unknown variant on HttpError');
        }
    }
}

export class Unauthorized extends HttpError {
    name = 'Unauthorized'

    constructor(public accountId: number | null) {
        super();
    }
};

export class MessageError extends ReaxError {
    constructor(public message: string) {
        super();
    }

    static deserialize(deserializer: Deserializer): MessageError {
        return new MessageError(deserializer.deserializeStr());
    }
}
