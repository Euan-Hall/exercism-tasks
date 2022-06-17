#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}



pub fn sublist<T: PartialEq>(a: &[T], b:  &[T]) -> Comparison {
    let m = a.len();
    let n = b.len();
    
    if m == 0 && n == 0 {Comparison::Equal}
    else if (m != 0 && n == 0) || (m > n && a.windows(n).any(|v| v == b)) {Comparison::Superlist}
    else if (m == 0 && n != 0) || (m < n && b.windows(m).any(|v| v == a)) {Comparison::Sublist}
    else if m == n && a.windows(m).any(|v| v == b) {Comparison::Equal}
    else {Comparison::Unequal}

}

