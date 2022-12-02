pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = Vec::new();
    let mut factor = 1;
    let mut n = n;

    while n >= 2 {
        factor += 1;
        if n % factor == 0 {
            prime_factors.push(factor);
            n /= factor;
            factor = 1;
        }
    }

    prime_factors
}
