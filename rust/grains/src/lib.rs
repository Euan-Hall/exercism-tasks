pub fn square(s: u32) -> u64 {
    match s {
        x if x >= 1 && x <= 64 => {2_u64.pow(s-1)},
        _ => panic!("Square must be between 1 and 64")
    }
}

pub fn total() -> u64 {
    (1..65).map(|x| 2_u64.pow(x-1)).sum::<u64>()
}
