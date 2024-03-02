/* tslint:disable */
/* eslint-disable */
/**
*/
export class NES {
  free(): void;
/**
* @param {Uint8Array} bytes
* @returns {NES}
*/
  static new_nes(bytes: Uint8Array): NES;
/**
*/
  step(): void;
/**
* @returns {number}
*/
  frame_buffer_pointer(): number;
/**
* @param {number} index
* @param {boolean} pressed
*/
  update_button(index: number, pressed: boolean): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_nes_free: (a: number) => void;
  readonly nes_new_nes: (a: number, b: number) => number;
  readonly nes_step: (a: number) => void;
  readonly nes_frame_buffer_pointer: (a: number) => number;
  readonly nes_update_button: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
