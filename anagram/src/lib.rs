use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();
    let chars: Vec<char> = word.to_lowercase().chars().collect();
    let mut word_map: HashMap<char, i32> = HashMap::new();
    for c in chars {
        *word_map.entry(c).or_insert(0) += 1;
    }
    for anagram in possible_anagrams {
        if word.to_lowercase() == anagram.to_lowercase() {
            continue;
        }
        let ana_chars: Vec<char> = anagram.to_lowercase().chars().collect();
        let mut ana_map: HashMap<char, i32> = HashMap::new();
        for a in ana_chars {
            *ana_map.entry(a).or_insert(0) += 1;
        }
        if word_map == ana_map {
           result.insert(&anagram);
        }
    }
    return result;
}
