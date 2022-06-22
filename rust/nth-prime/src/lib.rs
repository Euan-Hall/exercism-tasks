pub fn nth(mut n: u32) -> u32 {
    let primes: Vec<u32> = sieve();
    let mut nth: usize = 0;
    while n != 0 {
       if primes[nth] == 1 {n-=1}
       nth+=1
    }
    primes[nth]
}


pub fn sieve() -> Vec<u32> { 
    let mut primes: Vec<u32>  = vec![1; 100];
    primes[0] = 0;
    primes[1] = 0;

    let mut i = 2;
    while i*i <= 100 {
        if primes[i] == 0 { i+=1; continue}
        else {
            let mut j = 2*i;
            while j < 100{
                primes[j] = 0;
                j += 1;
            }
        }
    }
    primes
}
