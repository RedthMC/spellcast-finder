mod word;
mod word_finder;

use wasm_bindgen::prelude::*;
use word::Letter;
use word_finder::{Multipliers, Pos, SearchResult, WordList};

static mut WORD_LIST: Option<WordList> = None;


#[wasm_bindgen]
pub fn load_word_list() -> bool {
    unsafe {
        WORD_LIST = WordList::load("./resources/words.txt").ok();
        return WORD_LIST.is_some();
    }
}

#[wasm_bindgen]
pub fn find(board: &str, multipliers: JsPosMultipliers) -> Option<JsSearchResult> {
    let word_list = unsafe { WORD_LIST.as_ref()? };
    let mut iter = board.bytes().map_while(Letter::new);
    let board = [iter.next()?; 25];
    return word_list.find(&board, multipliers.into()).map(JsSearchResult::from);
}

#[wasm_bindgen]
pub struct JsPosMultipliers {
    pub double_letter: i32,
    pub triple_letter: i32,
    pub double_score: i32,
}
impl Into<Multipliers<Option<Pos>>> for JsPosMultipliers {
    fn into(self) -> Multipliers<Option<Pos>> {
        let index = Multipliers {
            double_letter: self.double_letter,
            triple_letter: self.triple_letter,
            double_score: self.double_score,
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
