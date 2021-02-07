use draw_a_box::Weight;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

pub fn main() {
    let titles = fs::read_to_string("./titles.txt").unwrap();
    let mut parsed: Vec<_> = titles
        .lines()
        .map(|line| {
            let mut split = line.rsplitn(2, ' ');
            (split.next().unwrap(), parse(split.next().unwrap()))
        })
        .map(|(c, weights)| {
            (
                c,
                [
                    *weights.get(&Direction::Up).unwrap(),
                    *weights.get(&Direction::Right).unwrap(),
                    *weights.get(&Direction::Down).unwrap(),
                    *weights.get(&Direction::Left).unwrap(),
                ],
            )
        })
        .collect();

    parsed.sort_by_key(|&(_, weights)| weights);
    for (c, weights) in parsed {
        println!(
            "(Weight::{:?}, Weight::{:?}, Weight::{:?}, Weight::{:?}) => {},",
            weights[0], weights[1], weights[2], weights[3], c,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Word {
    Direction(RawDirection),
    Weight(Weight),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RawDirection {
    Up,
    Right,
    Down,
    Left,
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

/// Tries to parse the title of a unicode codepoint and check if it's a light or heavy box drawing
/// character.
fn parse(title: &str) -> HashMap<Direction, Weight> {
    let lowercased = title.to_lowercase();
    let words = lowercased
        .split_ascii_whitespace()
        .filter_map(|word| match word {
            "up" => Some(Word::Direction(RawDirection::Up)),
            "right" => Some(Word::Direction(RawDirection::Right)),
            "down" => Some(Word::Direction(RawDirection::Down)),
            "left" => Some(Word::Direction(RawDirection::Left)),
            "horizontal" => Some(Word::Direction(RawDirection::Horizontal)),
            "vertical" => Some(Word::Direction(RawDirection::Vertical)),
            "light" => Some(Word::Weight(Weight::Light)),
            "heavy" => Some(Word::Weight(Weight::Heavy)),
            "and" => None,
            w => panic!("Found unknown word: {}", w),
        });

    let mut directions = Vec::new();
    let mut weight = Weight::Empty;
    let mut map = HashMap::new();
    map.insert(Direction::Up, Weight::Empty);
    map.insert(Direction::Right, Weight::Empty);
    map.insert(Direction::Down, Weight::Empty);
    map.insert(Direction::Left, Weight::Empty);

    for word in words {
        match word {
            Word::Direction(dir) => match dir {
                RawDirection::Up => directions.push(Direction::Up),
                RawDirection::Right => directions.push(Direction::Right),
                RawDirection::Down => directions.push(Direction::Down),
                RawDirection::Left => directions.push(Direction::Left),
                RawDirection::Horizontal => {
                    directions.push(Direction::Right);
                    directions.push(Direction::Left);
                }
                RawDirection::Vertical => {
                    directions.push(Direction::Up);
                    directions.push(Direction::Down);
                }
            },
            Word::Weight(w) => {
                match (weight == Weight::Empty, directions.is_empty()) {
                    (true, false) => {
                        for &dir in &directions {
                            map.insert(dir, w);
                        }
                        directions.clear();
                        weight = Weight::Empty;
                    }
                    (false, false) => {
                        for &dir in &directions {
                            map.insert(dir, weight);
                        }
                        directions.clear();
                        weight = w;
                    }
                    (true, true) => {
                        weight = w;
                    }
                    (false, true) => {
                        // Already found weight, but no directions to go with it.
                        // Would overwrite the first weight without adding any directions.
                        panic!("Would overwrite weight without adding any directions.")
                    }
                };
            }
        }
    }

    for &dir in &directions {
        map.insert(dir, weight);
    }

    map
}
