export interface Folder {
    id: number;
    name: String;
}

export class ReaxError {
    static deserialize(obj: Record<string, any> | string): ReaxError {
        if (typeof obj !== 'object') {
            throw new Error(`expected object, got ${typeof obj}`);
        }

        if ('Http' in obj) {
            return HttpError.deserialize(obj['Http']);
        } else if ('Message' in obj) {
            return MessageError.deserialize(obj['Message']);
        }

        throw new Error(`unknown variant for ReaxError, ${Object.keys(obj)}`);
    }
}

export class HttpError extends ReaxError {
    static NoConnection = new class extends HttpError { name = 'NoConnection' };
    static UnexpectedResponse = new class extends HttpError { name = 'UnexpectedResponse' };
    static Unauthorized = new class extends HttpError { name = 'Unauthorized' };
    static Unknown = new class extends HttpError { name = 'Unknown' };

    static deserialize(str: string): HttpError {
        if (str === this.NoConnection.name) {
            return this.NoConnection;
        } else if (str === this.UnexpectedResponse.name) {
            return this.UnexpectedResponse;
        } else if (str === this.Unauthorized.name) {
            return this.Unauthorized;
        } else if (str === this.Unknown.name) {
            return this.Unknown;
        }

        throw new Error(`unknown variant for HttpError ${str}`);
    }
}

export class MessageError extends ReaxError {
    constructor(public message: string) {
        super();
    }

    static deserialize(str: string): MessageError {
        if (typeof str !== 'string') {
            throw new Error(`expected string, got ${typeof str}`);
        }

        return new this(str);
    }
}
