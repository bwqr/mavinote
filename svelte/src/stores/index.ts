import { goto } from "$app/navigation";
import { HttpError, ReaxError } from "../models";

export function handleError(e: string): Promise<void> {
    const error = ReaxError.deserialize(JSON.parse(e));

    if (error instanceof HttpError && error === HttpError.Unauthorized) {
        goto('/login');
    }

    return Promise.reject(error);
}
