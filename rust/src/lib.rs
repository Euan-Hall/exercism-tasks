pub fn nth(mut n: u32) -> u32 {
    n += 1;
	let mut total_primes: u32 = 0;
	let mut size_factor: u32 = 2;
	let mut primes: Vec<u32> = Vec::new();
	while total_primes < n {
		primes = sieve((size_factor * n) as usize);
		total_primes = primes.iter().sum();
		size_factor += 1;
	}
	let mut count = 0;
	for (k, item) in 2..primes.iter().enumerate().skip(2){
	    count += item;
	    if count == n {
	        return k as u32
	    }
	}
	0
}

pub fn sieve(size: usize) -> Vec<u32>{
	let mut primes: Vec<u32> = vec![1; size];
	primes[0] = 0;
	primes[1] = 0;

	let mut i: usize = 2;
	while i*i <= size {
		if primes[i] == 1 {
    		let mut j = 2*i;
    		while j < size {
    			primes[j] = 0;
    			j += i;
    		}
    	}
    	i+=1;

	}
	primes
}
