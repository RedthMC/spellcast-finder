use std::ops::Add;

use crate::word::{Letter, LetterCountsMap, Word};

#[derive(Default)]
pub struct WordList(Box<[Word]>);
impl WordList {
    pub fn load(string: &str) -> WordList {
        let collected: Box<[Word]> = string.lines().map(Word::new).collect();
        return WordList(collected);
    }

    pub fn find(
        &self,
        board: &[Letter; 25],
        multipilers: Multipliers<Option<Pos>>,
    ) -> Option<SearchResult> {
        return WordFinder::pfind(&self.0, board, multipilers);
    }
}

#[derive(PartialEq, Clone, Copy, Debug, Eq, PartialOrd, Ord)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn all() -> impl Iterator<Item = Pos> {
        return (0..5).flat_map(|y| (0..5).map(move |x| Pos { x, y }));
    }

    pub fn from_index(index: i32) -> Option<Self> {
        return match index {
            (0..=24) => Some(Pos {
                x: index % 5,
                y: index / 5,
            }),
            _ => None,
        };
    }

    pub fn new(x: i32, y: i32) -> Option<Self> {
        return match (x, y) {
            (0..=4, 0..=4) => Some(Pos { x, y }),
            _ => None,
        };
    }

    pub fn index(&self) -> usize {
        return (self.x + self.y * 5) as usize;
    }

    const DIRECTIONS: [Pos; 8] = [
        Pos { x: -1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: 1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: 1, y: 0 },
        Pos { x: -1, y: 1 },
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
    ];

    fn surrounding_iter(&self) -> impl Iterator<Item = Pos> + '_ {
        Self::DIRECTIONS
            .iter()
            .filter_map(move |offset| self + offset)
    }
}

impl Add<Self> for &Pos {
    type Output = Option<Pos>;

    fn add(self, rhs: Self) -> Self::Output {
        return Pos::new(self.x + rhs.x, self.y + rhs.y);
    }
}

pub struct WordFinder<'a> {
    word_list: &'a [Word],
    board: &'a [Letter; 25],
    board_letter_counts: LetterCountsMap,
    multipilers: Multipliers<Option<Pos>>,
}

impl<'a> WordFinder<'a> {
    pub fn pfind(
        word_list: &'a [Word],
        board: &'a [Letter; 25],
        multipilers: Multipliers<Option<Pos>>,
    ) -> Option<SearchResult> {
        let board_letter_counts = LetterCountsMap::new(board);
        let finding = Self {
            word_list,
            board,
            board_letter_counts,
            multipilers,
        };
        return finding.find();
    }

    fn letters_multiplier(&self) -> Multipliers<Option<Letter>> {
        return self
            .multipilers
            .map(|pos| pos.map(|pos| self.board_get(pos)));
    }

    fn board_get(&self, pos: Pos) -> Letter {
        return self.board[pos.index()];
    }

    fn contains_enough_letters(&self, word: &Word) -> bool {
        return self.board_letter_counts.contains(&word.letter_counts);
    }

    fn find_next_letter(&self, word: &Word, current_pos: Pos, path: &mut Vec<Pos>) -> bool {
        if path.len() >= word.length() {
            return true;
        }
        let next_letter = word.letters[path.len()];
        if path.contains(&current_pos) || self.board[current_pos.index()] != next_letter {
            return false;
        }

        path.push(current_pos);

        for pos in current_pos.surrounding_iter() {
            if self.find_next_letter(word, pos, path) {
                return true;
            }
        }

        path.pop();
        false
    }

    fn find_path(& self, word: &'a Word) -> impl Iterator<Item = Vec<Pos>> + '_ {
        return Pos::all().filter_map(|pos| {
            let mut path = vec![];
            match self.find_next_letter(word, pos, &mut path) {
                true => Some(path),
                false => None,
            }
        });
    }

    fn get_best_possible_score(&self, word: &Word) -> i32 {
        return word.get_best_possible_score(self.letters_multiplier());
    }

    fn get_score_of_path(&self, word: &Word, path: &[Pos]) -> i32 {
        let multipliers = self.multipilers.map(|pos| {
            pos.filter(|pos| path.contains(pos))
                .map(|pos| self.board_get(pos))
        });
        return word.calculate_score(multipliers);
    }

    pub fn find(&self) -> Option<SearchResult> {
        let mut current_score = 0;
        let mut current_path: Option<Vec<Pos>> = None;
        let mut found_word: Option<&Word> = None;

        for word in self.word_list {
            if !self.contains_enough_letters(&word) {
                continue;
            }
            if current_score >= self.get_best_possible_score(word) {
                continue;
            }
            for path in self.find_path(&word) {
                let score = self.get_score_of_path(word, &path);
                if current_score >= score {
                    continue;
                }
                current_score = score;
                current_path = Some(path);
                found_word = Some(&word);
            }
        }

        match (current_path, found_word) {
            (Some(path), Some(word)) => Some(SearchResult {
                score: current_score,
                path,
                word: word.string.clone(),
            }),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SearchResult {
    pub score: i32,
    pub path: Vec<Pos>,
    pub word: Box<str>,
}

pub struct Multipliers<T> {
    pub double_letter: T,
    pub triple_letter: T,
    pub double_score: T,
}
impl<T> Multipliers<T> {
    pub fn map<U, F>(&self, function: F) -> Multipliers<U>
    where
        F: Fn(&T) -> U,
    {
        return Multipliers {
            double_letter: function(&self.double_letter),
            triple_letter: function(&self.triple_letter),
            double_score: function(&self.double_score),
        };
    }
}
