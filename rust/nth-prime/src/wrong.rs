pub fn nth(n: u32) -> u32 {
    if n == 2 {
        return 0;
    }

    let mut counter: u32 = 0;
    for i in 3..=n {
        let mut status: bool = true;
        for j in 2..i {
            if i % j == 0 {
                status = false;
                break;
            }
        }
        if status {
            counter += 1;
        }
    }
    counter
}
