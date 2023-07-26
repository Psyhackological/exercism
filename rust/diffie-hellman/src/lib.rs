use primal::is_prime;
use rand::Rng;

/// Computes the modular exponentiation of a number.
///
/// # Arguments
///
/// * `base` - The base number.
/// * `exp` - The exponent to raise the base to.
/// * `modulus` - The modulus to perform the operation under.
///
/// # Return
///
/// The result of `base^exp mod modulus`.
// For large inputs, built-in integer exponentiation and modulus aren't efficient, use modular exponentiation instead.
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    loop {
        let random_number = rng.gen_range(2..p);
        if is_prime(random_number) {
            return random_number;
        }
    }
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub, a, p)
}
