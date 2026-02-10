use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_digit(letters: &[char], digits: &[u8], letter: char) -> Option<u8> {
    if letters.len() != digits.len() {
        return None;
    }
    letters
        .iter()
        .enumerate()
        .filter_map(|(i, c)| if *c == letter { Some(digits[i]) } else { None })
        .next()
}

fn letters(input: &str) -> (usize, Vec<char>) {
    let mut first_letters = HashSet::new();
    let mut other_letters = HashSet::new();

    let words = input
        .split(|c: char| !c.is_alphabetic())
        .filter(|s| !s.is_empty());

    for word in words {
        let mut chars = word.chars();
        let first = chars.next().unwrap();
        first_letters.insert(first);
        other_letters.remove(&first);

        for ch in chars {
            if !first_letters.contains(&ch) {
                other_letters.insert(ch);
            }
        }
    }

    let first_letters_count = first_letters.len();
    let mut result: Vec<char> = first_letters.into_iter().collect();
    result.extend(other_letters.into_iter());

    (first_letters_count, result)
}

fn terms(input: &str) -> Vec<(char, i64)> {
    let mut sign = -1;
    input
        .split(|c: char| !c.is_alphabetic())
        .filter(|s| !s.is_empty())
        .rev()
        .flat_map(|word| {
            let word_len = word.len();
            let sign2 = sign;
            sign = 1;
            word.chars()
                .enumerate()
                .map(|(i, c)| (c, sign2 * 10_i64.pow((word_len - i - 1) as u32)))
                .collect::<Vec<(char, i64)>>()
        })
        .collect()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (first_letters_count, letters) = letters(input);
    let terms = terms(input);

    (0..=9)
        .permutations(letters.len())
        .par_bridge()
        .map(|digits| {
            if digits.iter().take(first_letters_count).any(|d| *d == 0) {
                None
            } else {
                let sum: i64 = terms
                    .iter()
                    .map(|(letter, exp)| {
                        get_digit(&letters, &digits, *letter).unwrap() as i64 * *exp
                    })
                    .sum();
                if sum == 0 {
                    Some(
                        letters
                            .iter()
                            .zip(digits.iter())
                            .map(|(&l, &d)| (l, d))
                            .collect(),
                    )
                } else {
                    None
                }
            }
        })
        .find_any(|v| v.is_some())?
}
