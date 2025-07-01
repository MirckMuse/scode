/* tslint:disable */
/* eslint-disable */

export interface EditorStateConfig {
    doc?: string | Text;
}



export abstract class Line {
    abstract a: number;
}


/**
*/
export class Line {
  free(): void;
/**
* 行起始位置
*/
  from: number;
/**
* 行终点位置
*/
  to: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_line_free: (a: number) => void;
  readonly __wbg_get_line_to: (a: number) => number;
  readonly __wbg_set_line_to: (a: number, b: number) => void;
  readonly __wbg_get_line_from: (a: number) => number;
  readonly __wbg_set_line_from: (a: number, b: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
