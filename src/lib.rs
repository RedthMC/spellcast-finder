mod word;
mod word_finder;

use wasm_bindgen::prelude::*;
use word::Letter;
use word_finder::{Multipliers, Pos, SearchResult, WordList};

static mut WORD_LIST: Option<WordList> = None;


#[wasm_bindgen]
pub fn load_word_list(word_list: &str) -> bool {
    unsafe {
        WORD_LIST = WordList::load(word_list).ok();
        return WORD_LIST.is_some();
    }
}

#[wasm_bindgen]
pub fn find(board: &str, multipliers: PosMultiplier) -> Option<JsSearchResult> {
    let word_list = unsafe { WORD_LIST.as_ref()? };
    let mut iter = board.bytes().map_while(Letter::new);
    let board = [iter.next()?; 25];
    return word_list.find(&board, multipliers.into()).map(JsSearchResult::from);
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

    fn double_letter(this: &PosMultiplier) -> i32;
    fn triple_letter(this: &PosMultiplier) -> i32;
    fn double_score(this: &PosMultiplier) -> i32;
}

// #[wasm_bindgen]
// pub struct JsPosMultipliers {
//     pub double_letter: i32,
//     pub triple_letter: i32,
//     pub double_score: i32,
// }

impl Into<Multipliers<Option<Pos>>> for PosMultiplier {
    fn into(self) -> Multipliers<Option<Pos>> {
        let index = Multipliers {
            double_letter: double_letter(&self),
            triple_letter: triple_letter(&self),
            double_score: double_score(&self),
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
