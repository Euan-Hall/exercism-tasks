pub fn factors(mut n: u64) -> Vec<u64> {
   let mut prime_factors: Vec<u64> = Vec::new();

   let mut p = 2;
   while n >= p*p {
       println!("{} % {}", n, p);
       if n % p == 0 {
           prime_factors.push(p);
           n /= p;
        }
       else {p += 1;}
    }
    if n == 1 { return prime_factors }
    prime_factors.push(n);
    prime_factors
}
