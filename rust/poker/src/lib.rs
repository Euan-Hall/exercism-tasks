/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // Split each element in a the array of strings
    let hand_vec: Vec<_> = hands.iter().map(|&x| x.split(" ")).collect();

    // Loop through each hand
    for hand in hand_vec.iter(){
        // Hand is five of a kind
        let all_eq = is_all_same(hand);
    }    
    
}


pub fn is_all_same(arr: &std::str::Split<'_, &str>) -> bool {
    arr = arr.collect();
    arr.iter().min() == arr.iter().max()
}
