use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
     // Only add words to the hashset which are equal in length to the input word
    let word_l = word.to_lowercase();

    // Map input word to hashmap
    let mut char_map = HashMap::new();
    word_l.chars().for_each(|c| *char_map.entry(c).or_insert(0) += 1);
    
    // Defining a macro (?) which looks through the chars in the given input and will return True
    // if there are no excess chars, or false if there is more chars in the input.
    let check_char = |s: &str| -> bool {
        let mut char_map = char_map.clone();
        s.chars().any(|c| {
            let v = char_map.entry(c).or_insert(0);
            *v -= 1;
            *v < 0
        })
    };
    
    // Loop through the anagrams list, only keep the items which aren't equal to the input word and
    // the number of chars match.
    possible_anagrams.iter().filter(|x| {
        x.len() == word.len() && {
            let x_lower = x.to_lowercase();
            x_lower != word_l && !check_char(&x_lower)
        }
    }).cloned().collect()
}
