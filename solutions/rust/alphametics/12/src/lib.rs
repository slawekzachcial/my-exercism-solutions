use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn get_digit(letters: &[char], digits: &[u8], letter: char) -> Option<u8> {
    let index = letters.binary_search(&letter).unwrap();
    Some(digits[index])
}

fn letters(input: &str) -> (Vec<char>, Vec<usize>) {
    let mut first_letters = HashSet::new();
    let mut all_letters = HashSet::new();

    let words = input
        .split(|c: char| !c.is_alphabetic())
        .filter(|s| !s.is_empty());

    for word in words {
        let mut chars = word.chars();
        let first = chars.next().unwrap();
        first_letters.insert(first);
        all_letters.insert(first);
        for ch in chars {
            all_letters.insert(ch);
        }
    }

    let mut letters_sorted: Vec<char> = all_letters.into_iter().collect();
    letters_sorted.sort();
    let first_letter_positions = first_letters
        .iter()
        .map(|letter| letters_sorted.binary_search(letter).unwrap())
        .collect();

    (letters_sorted, first_letter_positions)
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

fn sum(terms: &[(char, i64)], letters_sorted: &[char], digits: &[u8]) -> i64 {
    terms
        .iter()
        .map(|(letter, exp)| get_digit(letters_sorted, digits, *letter).unwrap() as i64 * *exp)
        .sum()
}

fn zip_as_map(letters: &[char], digits: &[u8]) -> HashMap<char, u8> {
    letters
        .iter()
        .zip(digits.iter())
        .map(|(&l, &d)| (l, d))
        .collect()
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (letters_sorted, first_letter_positions) = letters(input);
    let terms = terms(input);

    (0..=9)
        .permutations(letters_sorted.len())
        .par_bridge()
        .map(|digits| {
            if first_letter_positions.iter().any(|pos| digits[*pos] == 0) {
                None
            } else if sum(&terms, &letters_sorted, &digits) == 0 {
                Some(zip_as_map(&letters_sorted, &digits))
            } else {
                None
            }
        })
        .find_any(|v| v.is_some())?
}
