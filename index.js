import init, { load_word_list, find } from './pkg/spellcast_finder.js';
await init();
const response = await fetch("https://raw.githubusercontent.com/RedthMC/spellcast-finder/master/resources/words.txt");
const wordList = await response.text();
const loaded = load_word_list(wordList);
console.log(`loaded: ${loaded}`);
const findButton = document.querySelector("#find-button");
const resultBox = document.querySelector("p");
const cells = [...document.querySelectorAll(".cell-background")];

document.addEventListener("keyup", event => {
    if (event.key !== "Enter") return;
    findButton.click();
    event.preventDefault();
});

findButton.addEventListener("keydown", event => {
    if (event.key === "Enter") event.preventDefault();
});

for (const cell of cells) {
    cell.addEventListener("keydown", event => {
        let key = event.key
        switch (key) {
            case "ArrowUp":
                cell.previousSibling?.previousSibling?.previousSibling?.previousSibling?.previousSibling?.previousSibling?.previousSibling?.previousSibling?.previousSibling?.previousSibling?.focus();
                return;
            case "ArrowDown":
                cell.nextSibling?.nextSibling?.nextSibling?.nextSibling?.nextSibling?.nextSibling?.nextSibling?.nextSibling?.nextSibling?.nextSibling?.focus();
                return;
            case "ArrowLeft":
                cell.previousSibling?.previousSibling?.focus();
                return;
            case "ArrowRight":
                cell.nextSibling?.nextSibling?.focus();
                return;
        }
        if (key.length !== 1) return;
        if (!key.match(/[A-z]/g)) return;
        key = key.toUpperCase();
        cell.textContent = key;
        (cell.nextSibling?.nextSibling || findButton).focus();
    });
}

findButton.onclick = () => {
    findButton.classList.remove("clicked");
    void findButton.offsetWidth;
    findButton.classList.add("clicked");
    
    for (const cell of cells) {
        cell.classList.remove("path");
    }

    const board = cells.map(e => e.innerText.toLowerCase()).join("");
    const result = find(board, {
        double_letter: -1,
        triple_letter: -1,
        double_score: -1,
    });
    const text = result ? `${result.word}<br>${result.score} Points` : "No Result"
    resultBox.innerHTML = text;
    for (const path of result.path) {
        cells[path].classList.add("path");
    }

    result.free();
};
