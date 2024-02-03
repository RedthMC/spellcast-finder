/* tslint:disable */
/* eslint-disable */
/**
* @param {string} word_list
* @returns {boolean}
*/
export function load_word_list(word_list: string): boolean;
/**
* @param {string} board
* @param {JsPosMultipliers} multipliers
* @returns {JsSearchResult | undefined}
*/
export function find(board: string, multipliers: JsPosMultipliers): JsSearchResult | undefined;
/**
*/
export class JsPosMultipliers {
  free(): void;
/**
*/
  double_letter: number;
/**
*/
  double_score: number;
/**
*/
  triple_letter: number;
}
/**
*/
export class JsSearchResult {
  free(): void;
/**
*/
  readonly path: Int32Array;
/**
*/
  score: number;
/**
*/
  readonly word: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly load_word_list: (a: number, b: number) => number;
  readonly find: (a: number, b: number, c: number) => number;
  readonly __wbg_jsposmultipliers_free: (a: number) => void;
  readonly __wbg_get_jsposmultipliers_double_letter: (a: number) => number;
  readonly __wbg_set_jsposmultipliers_double_letter: (a: number, b: number) => void;
  readonly __wbg_get_jsposmultipliers_triple_letter: (a: number) => number;
  readonly __wbg_set_jsposmultipliers_triple_letter: (a: number, b: number) => void;
  readonly __wbg_get_jsposmultipliers_double_score: (a: number) => number;
  readonly __wbg_set_jsposmultipliers_double_score: (a: number, b: number) => void;
  readonly __wbg_jssearchresult_free: (a: number) => void;
  readonly __wbg_get_jssearchresult_score: (a: number) => number;
  readonly __wbg_set_jssearchresult_score: (a: number, b: number) => void;
  readonly jssearchresult_path: (a: number, b: number) => void;
  readonly jssearchresult_word: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
