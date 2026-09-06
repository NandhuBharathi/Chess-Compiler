/* tslint:disable */
/* eslint-disable */

export class ChessCompiler {
    free(): void;
    [Symbol.dispose](): void;
    black_score(): number;
    board(): string;
    game_over(): boolean;
    move_count(): number;
    constructor();
    play_move(input: string): boolean;
    turn(): string;
    white_score(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_chesscompiler_free: (a: number, b: number) => void;
    readonly chesscompiler_black_score: (a: number) => number;
    readonly chesscompiler_board: (a: number) => [number, number];
    readonly chesscompiler_game_over: (a: number) => number;
    readonly chesscompiler_move_count: (a: number) => number;
    readonly chesscompiler_new: () => number;
    readonly chesscompiler_play_move: (a: number, b: number, c: number) => number;
    readonly chesscompiler_turn: (a: number) => [number, number];
    readonly chesscompiler_white_score: (a: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
