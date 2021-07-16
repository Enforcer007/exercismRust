use std::{collections::HashSet, iter::FromIterator};

fn sort_to_lower(x: String) -> String {
    let mut lower_case_x = x.to_lowercase().chars().collect::<Vec<char>>();
    lower_case_x.sort();
    String::from_iter(lower_case_x)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lower_case = sort_to_lower(String::from(word));
    let sorted_anagrams = possible_anagrams
        .iter()
        .map(|x| sort_to_lower(String::from(*x)))
        .collect::<Vec<String>>();
    sorted_anagrams
        .iter()
        .zip(possible_anagrams.iter())
        .filter(|x| *x.0 == lower_case && *x.1.to_lowercase() != word.to_lowercase())
        .map(|x| *x.1)
        .collect::<HashSet<&str>>()
}
