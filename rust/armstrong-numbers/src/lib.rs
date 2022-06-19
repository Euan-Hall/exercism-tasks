pub fn is_armstrong_number(num: u32) -> bool {
    // Split each digit of the number into a vecor
    let mut x = num;
    let mut nums: Vec<u32> = Vec::new();
    loop {
        nums.push(x % 10);
        x /= 10;
        if x == 0 {break}
    }

    // Map each digit to the power of the inputs length and check if the sum is equal to the input.
    let len: u32 = nums.len() as u32;
    nums = nums.iter().map(|x| x.pow(len)).collect();
    let sum: u32 = nums.iter().sum();
    if sum  == num {return true;}
    false
}
