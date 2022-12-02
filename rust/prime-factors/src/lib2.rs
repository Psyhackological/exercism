pub fn factors(n: u64) -> Vec<u64> {
    // This math problem overcomed me and I must admit that
    // I took a solution from https://the-algorithms.com/algorithm/prime-factors?lang=rust
    // However I want to learn, not only copy code so there are my notes

    let mut prime_factors = vec![]; // our return value
    let mut i = 2; // starting prime value (1 is not a prime number)
    let mut n = n; // shadow n value to make it mutable

    // 0 does not have any prime factors
    if n == 0 {
        return prime_factors; // early return empty Vec<u64>
    }

    // that means there are still factors to be found
    while i * i <= n {
        // n not disible by i
        if n % i != 0 {
            // if i not equal to 2, then double increment by 2 (1+1) - skipping 4
            if i != 2 {
                i += 1; // simple increment by 1
            }
            i += 1; // simple increment by 1
                    // else disible by i (no remainder)
        } else {
            n /= i; // reduce (divide) n by i
            prime_factors.push(i); // pushing found prime factor
        }
    }

    // if n is greater than 1, then n still has some prime factor in it
    if n > 1 {
        prime_factors.push(n);
    }

    prime_factors // Vec<u64> result of prime factors
}
