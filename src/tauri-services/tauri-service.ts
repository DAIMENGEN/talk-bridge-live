import {EventCallback, listen as tauriListen, Options, UnlistenFn} from "@tauri-apps/api/event";

export class TauriService {

    static listen<T>(eventName: string, callback: EventCallback<T>, options?: Options): Promise<UnlistenFn> {
        return tauriListen(eventName, callback, options);
    }

}