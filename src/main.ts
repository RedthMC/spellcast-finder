import './app.css';
import App from './App.svelte';
import init, { find, JsSearchResult, load_word_list } from './wasm/spellcast_finder.js';
import { writable, type Writable } from 'svelte/store';

async function load_wasm() {
    await init();
    const response = await fetch('https://raw.githubusercontent.com/RedthMC/spellcast-finder/master/resources/words.txt');
    const wordList = await response.text();
    const loaded = load_word_list(wordList);
    console.log(`loaded word list: ${loaded}`);
}

load_wasm();

let lastBoard = {
    letters: Array.from(Array(25), (_, i) => String.fromCharCode(65 + i)),
    double_letter: -1,
    triple_letter: -1,
    double_score: -1,
};

export const board = writable(lastBoard);
export const result: Writable<JsSearchResult | undefined> = writable();

board.subscribe(b => lastBoard = b);

export function findWord() {
    result.set(find(lastBoard.letters.join("").toLowerCase(), lastBoard));
}

export function preventEnter(event: KeyboardEvent) {
    if (event.key === 'Enter')
        event.preventDefault();
}

export const app = new App({ target: document.body });
