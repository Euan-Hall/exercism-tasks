pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut nums: Vec<u32> = Vec::new();
    for factor in factors {
        if factor == &(0 as u32) { continue; } 
        let mut i = 1;
        let mut product: u32 = factor * i;
        while product < limit {
            if !nums.iter().any(|&e| e == product) { nums.push(product) };
            i += 1;
            product = factor * i;
        }
    }
    nums.iter().sum()
}
