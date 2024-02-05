import './app.css'
import App from './App.svelte'
import init, { load_word_list } from "./wasm/spellcast_finder.js"

async function load_wasm() {
    await init();
    const response = await fetch("https://raw.githubusercontent.com/RedthMC/spellcast-finder/master/resources/words.txt");
    const wordList = await response.text();
    const loaded = load_word_list(wordList);
    console.log(`loaded word list: ${loaded}`);
}

load_wasm();
export const app = new App({ target: document.body });
