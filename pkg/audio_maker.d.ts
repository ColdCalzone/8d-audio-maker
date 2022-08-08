/* tslint:disable */
/* eslint-disable */
/**
* @param {number} x
*/
export function write_to_audio(x: number): void;
/**
* @returns {number}
*/
export function get_audio_pointer(): number;
/**
* @returns {number}
*/
export function get_audio_length(): number;
/**
* @param {number} x
*/
export function write_to_hrir(x: number): void;
/**
* @param {number} x
*/
export function write_rate(x: number): void;
/**
*/
export function convert_data_to_audio_blob(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly write_to_audio: (a: number) => void;
  readonly get_audio_pointer: () => number;
  readonly write_to_hrir: (a: number) => void;
  readonly convert_data_to_audio_blob: () => void;
  readonly get_audio_length: () => number;
  readonly write_rate: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
