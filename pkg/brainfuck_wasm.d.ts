/* tslint:disable */
/* eslint-disable */
/**
* @param {string} string
* @returns {InterpreterContext}
*/
export function new_interpreter(string: string): InterpreterContext;
/**
* @param {string} text
*/
export function generate_from_text(text: string): void;
/**
*/
export class InterpreterContext {
  free(): void;
/**
*/
  step(): void;
/**
*/
  run(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_interpretercontext_free: (a: number) => void;
  readonly interpretercontext_step: (a: number) => void;
  readonly interpretercontext_run: (a: number) => void;
  readonly new_interpreter: (a: number, b: number) => number;
  readonly generate_from_text: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
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
