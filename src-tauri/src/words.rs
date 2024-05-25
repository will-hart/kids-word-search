use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// A type of word list, either by theme or age
pub enum WordListType {
    EarlyPrimary,
    MidPrimary,
    LatePrimary,
    Dragons,
}

impl WordListType {
    pub fn get_options() -> Vec<WordListType> {
        use WordListType::*;
        vec![EarlyPrimary, MidPrimary, LatePrimary, Dragons]
    }
}

#[derive(Clone, Debug)]
enum WordDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn get_random_worddirection(rng: &mut ThreadRng) -> WordDirection {
    use WordDirection::*;
    [
        North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
    ]
    .choose(rng)
    .unwrap()
    .clone()
}

#[derive(Serialize, Deserialize)]
pub struct WordGrid {
    pub words: Vec<String>,
    pub grid: Vec<char>,
}

/// Gets a list of words based on the [WordListType]
fn get_words(from_list: WordListType, max_word_length: usize) -> Vec<String> {
    match from_list {
        WordListType::EarlyPrimary => include_str!("../data/01_early_primary.txt"),
        WordListType::MidPrimary => include_str!("../data/02_late_primary.txt"),
        WordListType::LatePrimary => include_str!("../data/03_early_secondary.txt"),
        WordListType::Dragons => include_str!("../data/dragons.txt"),
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

    let max_words = grid_size * 2;

    let mut selected_words = vec![];

    // try to fill the grid
    for word in words.iter() {
        let mut num_failures = 0;
        let mut placed = true;

        while !place_word(&mut rng, &mut grid, grid_size, word) {
            num_failures += 1;

            if num_failures > 5 {
                placed = false;
                break;
            }
        }

        if placed {
            // save the word
            selected_words.push(word.clone());

            // check if our grid is filled up
            if selected_words.len() >= max_words {
                break;
            }
        }
    }

    // fill the remaining places
    for idx in 0..grid.len() {
        if grid[idx] == ' ' {
            grid[idx] = rng.gen_range(b'A'..b'Z') as char;
        }
    }

    WordGrid {
        grid,
        words: selected_words,
    }
}

/// Attempts to randomly place a word on the grid
fn place_word(rng: &mut ThreadRng, grid: &mut [char], grid_size: usize, word: &str) -> bool {
    let direction: WordDirection = get_random_worddirection(rng);

    // determine the ranges of x starting points given the direction
    let (x_min, x_max) = match direction {
        WordDirection::North | WordDirection::South => (0, grid_size),
        WordDirection::NorthEast | WordDirection::East | WordDirection::SouthEast => {
            (0, grid_size - word.len())
        }
        WordDirection::SouthWest | WordDirection::West | WordDirection::NorthWest => {
            (word.len(), grid_size)
        }
    };

    let (y_min, y_max) = match direction {
        WordDirection::North | WordDirection::NorthEast | WordDirection::NorthWest => {
            (word.len(), grid_size)
        }
        WordDirection::SouthEast | WordDirection::South | WordDirection::SouthWest => {
            (0, grid_size - word.len())
        }
        WordDirection::East | WordDirection::West => (0, grid_size),
    };

    if (x_min..x_max).is_empty() || (y_min..y_max).is_empty() {
        // no placement available for this direction, abort early.
        return false;
    }

    let x = rng.gen_range(x_min..x_max);
    let y = rng.gen_range(y_min..y_max);

    // work out how to place the chars based on the word direction
    let pos_delta = match direction {
        WordDirection::North => -(grid_size as isize),
        WordDirection::NorthEast => -(grid_size as isize - 1),
        WordDirection::East => 1 as isize,
        WordDirection::SouthEast => grid_size as isize + 1,
        WordDirection::South => grid_size as isize,
        WordDirection::SouthWest => grid_size as isize - 1,
        WordDirection::West => -1,
        WordDirection::NorthWest => -(grid_size as isize + 1),
    };

    let mut start_pos = x + y * grid_size;
    println!(
        "Checking {word}({}) {direction:?}, starting at {start_pos} with delta {pos_delta}",
        word.len()
    );

    // check if the grid is occupied
    for _ in word.chars() {
        if grid[start_pos] != ' ' {
            return false;
        }

        start_pos = start_pos.checked_add_signed(pos_delta).unwrap();
    }

    // if not, place the word
    let mut start_pos = x + y * grid_size;
    println!(" -> Word fits, placing");
    for char in word.chars() {
        grid[start_pos] = char.clone();
        println!("{start_pos} + {pos_delta}");
        start_pos = start_pos.checked_add_signed(pos_delta).unwrap();
    }

    true
}
