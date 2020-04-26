/* tslint:disable */
/* eslint-disable */
/**
* @param {number} width 
* @param {number} height 
*/
export function resize(width: number, height: number): void;
/**
* @param {number} _offset_x 
* @param {number} _offset_y 
* @param {Uint32Array} data 
* @returns {number} 
*/
export function update(_offset_x: number, _offset_y: number, data: Uint32Array): number;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly resize: (a: number, b: number) => void;
  readonly update: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        