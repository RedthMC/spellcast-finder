use std::ops::Index;

use crate::word_finder::Multipliers;

const LETTER_SCORE_MAP: [i32; 26] = [
    1, 4, 5, 3, 1, 5, 3, 4, 1, 7, 3, 3, 4, 2, 1, 4, 8, 2, 2, 2, 4, 5, 5, 7, 4, 8,
];

#[derive(PartialEq, Clone, Copy, Debug, Eq, PartialOrd, Ord)]
pub struct Letter(u8);

impl Default for Letter {
    fn default() -> Self {
        return Self(b'a');
    }
}

impl Letter {
    pub fn new(byte: u8) -> Option<Self> {
        return match byte.is_ascii_lowercase() {
            true => Some(Self(byte)),
            false => None,
        };
    }

    pub fn get_alphabetical_index(&self) -> usize {
        return (self.0 - b'a') as usize;
    }

    pub fn get_score(&self) -> i32 {
        return LETTER_SCORE_MAP[self.get_alphabetical_index()];
    }
}

impl ToString for Letter {
    fn to_string(&self) -> String {
        return (self.0 as char).to_string();
    }
}

pub struct LetterCountsMap([u8; 26]);
impl LetterCountsMap {
    pub fn new(letters: &[Letter]) -> Self {
        let mut letter_counts = [0u8; 26];
        for letter in letters.iter() {
            letter_counts[letter.get_alphabetical_index()] += 1;
        }
        return Self(letter_counts);
    }

    pub fn contains(&self, other: &Self) -> bool {
        return (0..26).all(|i| self.0[i] >= other.0[i]);
    }
}

impl Index<u8> for LetterCountsMap {
    type Output = u8;

    fn index(&self, letter: u8) -> &Self::Output {
        if !letter.is_ascii_lowercase() {
            return &0;
        }
        let index = (letter - b'a') as usize;
        return &self.0[index];
    }
}

pub struct Word {
    pub string: Box<str>,
    pub letters: Box<[Letter]>,
    pub score: i32,
    pub letter_counts: LetterCountsMap,
}

impl Word {
    pub fn new(string: &str) -> Word {
        let letters: Box<[Letter]> = string.bytes().map_while(Letter::new).collect();
        let score = letters.iter().map(|l| l.get_score()).sum();
        let letter_counts = LetterCountsMap::new(&letters);
        return Word {
            string: string.into(),
            letters,
            score,
            letter_counts,
        };
    }
}

impl ToString for Word {
    fn to_string(&self) -> String {
        return self.string.to_string();
    }
}

impl Word {
    pub fn length(&self) -> usize {
        self.letters.len()
    }

    pub fn get_best_possible_score(&self, multipliers: Multipliers<Option<Letter>>) -> i32 {
        return self.calculate_score(multipliers.map(|l| l.filter(|l| self.letters.contains(l))));
    }

    pub fn calculate_score(&self, multipliers: Multipliers<Option<Letter>>) -> i32 {
        let mut score = self.score;

        if let Some(dl) = multipliers.double_letter {
            score += dl.get_score();
        }
        if let Some(tl) = multipliers.triple_letter {
            score += tl.get_score() * 2;
        }
        if multipliers.double_score.is_some() {
            score *= 2;
        }
        if self.length() >= 6 {
            score += 10;
        }

        return score;
    }
}
