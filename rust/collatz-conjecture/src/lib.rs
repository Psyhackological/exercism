pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        _ => {
            let mut n = n;
            let mut steps = 0_u64;
            while n != 1 {
                match n % 2 {
                    0 => {
                        steps += 1;
                        n /= 2;
                    }
                    1 => {
                        // reverse n * 3 + 1 -> (MAX_VALUE - 1) / 3
                        if n > (u64::MAX - 1) / 3 {
                            return None;
                        }
                        steps += 1;
                        n = n * 3 + 1;
                    }
                    _ => unreachable!(),
                }
            }
            Some(steps)
        }
    }
}
