import { writable } from "svelte/store";

export const path = writable(Int32Array.of())


export const board = writable({
    letters: Array.from(Array(25), (_, i) => String.fromCharCode(65 + i)),
    double_letter: -1,
    triple_letter: -1,
    double_score: -1,
});