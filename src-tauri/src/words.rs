use rand::thread_rng;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// A type of word list, either by theme or age
pub enum WordListType {
    EarlyPrimary,
    MidPrimary,
    LatePrimary,
}

#[derive(Serialize, Deserialize)]
pub struct WordGrid {
    pub grid: Vec<char>,
}

/// Gets a list of words based on the [WordListType]
fn get_words(from_list: WordListType, max_word_length: usize) -> Vec<String> {
    match from_list {
        WordListType::EarlyPrimary => include_str!("../data/01_early_primary.txt"),
        WordListType::MidPrimary => include_str!("../data/02_late_primary.txt"),
        WordListType::LatePrimary => include_str!("../data/03_early_secondary.txt"),
    }
    .lines()
    .filter_map(|s| {
        let trimmed = s.trim();
        if trimmed.starts_with("--")
            || trimmed.is_empty()
            || trimmed.len() < 3
            || trimmed.len() > max_word_length
        {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
    .collect()
}

/// Builds a 2d grid using a brute force method that randomly places words until it
/// either reaches the maximum number of words or fails a certain number of times.
/// The remaining spaces are filled with random letters.
pub fn build_grid(from_list: WordListType, grid_size: usize) -> WordGrid {
    // setup
    let mut rng = thread_rng();
    let mut grid = vec![' '; grid_size * grid_size];
    let mut words = get_words(from_list, grid_size);
    words.shuffle(&mut rng);

    // try to fill the grid

    // fill the remaining places
    for idx in 0..grid.len() {
        if grid[idx] == ' ' {
            grid[idx] = rng.gen_range(b'A'..b'Z') as char;
        }
    }

    WordGrid { grid }
}
