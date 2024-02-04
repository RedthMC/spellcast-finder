import init, { load_word_list, find } from './pkg/spellcast_finder.js';
await init();
const response = await fetch("https://raw.githubusercontent.com/RedthMC/spellcast-finder/master/resources/words.txt");
const wordList = await response.text();
const loaded = load_word_list(wordList);
console.log(`loaded: ${loaded}`);
for (const cell of document.querySelectorAll(".cell-background")) {
    cell.addEventListener("keydown", event => {
        let key = event.key
        console.log(key);
        if (key.length != 1) return;
        if (!key.match(/[A-z]/g)) return;
        key = key.toUpperCase();
        cell.textContent = key;
        cell.nextSibling.nextSibling?.focus()
    });
}

function findWord() {
    const result = find("abcdfergfidjsllkdopewfisd", {
        double_letter: -1,
        triple_letter: -1,
        double_score: -1,
    });
    console.log(result);
    const text = result ? `${result.word}<br>${result.score} Points` : "No Result"
    document.querySelector("p").innerHTML = text;
}
