// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    // Create hashmap
    let mut hm: HashMap<&str, u8> = HashMap::new();
    
    // Populate hashmap with magazine
    for item in note.iter() {
        *hm.entry(item).or_insert(0) += 1;
    }
    
    // Check if item exists in magazine, if it does, remove it, if it doesn't stop and return
    // false.
    for item in magazine.iter() {
        if !hm.contains_key(item){ continue }
        match hm.get_mut(item).unwrap(){
            0 => (),
            w => *w -= 1,
        }
    }
    hm.values().sum::<u8>() == 0
}

