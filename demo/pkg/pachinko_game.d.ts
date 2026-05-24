/* tslint:disable */
/* eslint-disable */

export class Game {
    free(): void;
    [Symbol.dispose](): void;
    auto_drop_center(): void;
    ball_count(): number;
    /**
     * Flat array [x,y,r, x,y,r, ...]
     */
    balls_flat(): Float32Array;
    bin_count(): number;
    bin_scores(): Uint32Array;
    bins(): Uint32Array;
    drop_ball(x: number): void;
    height(): number;
    constructor(width: number, height: number, bin_count: number);
    /**
     * Flat array [x,y,r, x,y,r, ...]
     */
    pegs_flat(): Float32Array;
    reset(): void;
    score(): number;
    update(dt: number): void;
    width(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_game_free: (a: number, b: number) => void;
    readonly game_auto_drop_center: (a: number) => void;
    readonly game_ball_count: (a: number) => number;
    readonly game_balls_flat: (a: number) => [number, number];
    readonly game_bin_count: (a: number) => number;
    readonly game_bin_scores: (a: number) => [number, number];
    readonly game_bins: (a: number) => [number, number];
    readonly game_drop_ball: (a: number, b: number) => void;
    readonly game_height: (a: number) => number;
    readonly game_new: (a: number, b: number, c: number) => number;
    readonly game_pegs_flat: (a: number) => [number, number];
    readonly game_reset: (a: number) => void;
    readonly game_score: (a: number) => number;
    readonly game_update: (a: number, b: number) => void;
    readonly game_width: (a: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
