mod word;
mod word_finder;

use wasm_bindgen::prelude::*;
use word::Letter;
use word_finder::{Multipliers, Pos, SearchResult, WordList};

static mut WORD_LIST: Option<WordList> = None;

#[wasm_bindgen]
pub fn load_word_list(word_list: &str) -> bool {
    unsafe {
        WORD_LIST = Some(WordList::load(word_list));
        return WORD_LIST.is_some();
    }
}

#[wasm_bindgen]
pub fn find(board: &str, multipliers: PosMultiplier) -> Option<JsSearchResult> {
    let word_list = unsafe { WORD_LIST.as_ref()? };
    let board: Box<[Letter]> = board.bytes().map_while(Letter::new).collect();
    let board: [Letter; 25] = board.as_ref().try_into().ok()?;
    return word_list
        .find(&board, multipliers.into())
        .map(JsSearchResult::from);
}

#[wasm_bindgen(typescript_custom_section)]
const POS_MULTIPLIERS: &'static str = r#"
type PosMultiplier = {
    double_letter: number;
    triple_letter: number;
    double_score: number;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "PosMultiplier")]
    pub type PosMultiplier;

    #[wasm_bindgen(structural, method, getter)]
    fn double_letter(this: &PosMultiplier) -> i32;

    #[wasm_bindgen(structural, method, getter)]
    fn triple_letter(this: &PosMultiplier) -> i32;

    #[wasm_bindgen(structural, method, getter)]
    fn double_score(this: &PosMultiplier) -> i32;
}

impl Into<Multipliers<Option<Pos>>> for PosMultiplier {
    fn into(self) -> Multipliers<Option<Pos>> {
        let index = Multipliers {
            double_letter: self.double_letter(),
            triple_letter: self.triple_letter(),
            double_score: self.double_score(),
        };
        return index.map(|&i| Pos::from_index(i));
    }
}

#[wasm_bindgen]
pub struct JsSearchResult {
    pub score: i32,
    path: Box<[i32]>,
    word: Box<str>,
}
#[wasm_bindgen]
impl JsSearchResult {
    #[wasm_bindgen(getter)]
    pub fn path(&self) -> Box<[i32]> {
        return self.path.clone();
    }
    #[wasm_bindgen(getter)]
    pub fn word(&self) -> String {
        return self.word.to_string();
    }
}
impl From<SearchResult> for JsSearchResult {
    fn from(value: SearchResult) -> Self {
        return Self {
            score: value.score,
            path: value.path.iter().map(|pos| pos.index() as i32).collect(),
            word: value.word,
        };
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use super::*;

    #[test]
    fn test_name() {
        let mut string = String::new();
        let mut file = File::open("./resources/words.txt").unwrap();
        file.read_to_string(&mut string).unwrap();
        let word_list = WordList::load(&string);
        let idk_wtf = "abcdfergfidjsllkdopewfisd"
            .bytes()
            .map(Letter::new)
            .map(Option::unwrap);
        let vect: Box<[Letter]> = idk_wtf.collect();
        let board: [Letter; 25] = vect.as_ref().try_into().unwrap();
        println!("board: {:?}", board);
        let result = word_list
            .find(
                &board,
                Multipliers {
                    double_letter: None,
                    triple_letter: None,
                    double_score: None,
                },
            )
            .unwrap();
        println!("{:?}", result);
        println!("helo?");
    }
}
