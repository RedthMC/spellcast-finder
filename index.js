import init, { load_word_list, find } from './pkg/spellcast_finder.js';
await init();
const loaded = load_word_list();
console.log(`loaded: ${loaded}`);
const result = find("abcdfergfidjsllkdopewfisd", {
    double_letter: -1,
    triple_letter: -1,
    double_score: -1,
});
console.log(result);