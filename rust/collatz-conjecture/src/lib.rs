pub fn collatz(n: u64) -> Option<u64> {
    let mut counter = 0;
    let mut n = n;
    if 0 == n {
        return None;
    }
    while 1 < n {
        if 0 == n % 2 {
            n /= 2;
            counter += 1;
        } else {
            n = 3 * n + 1;
            counter += 1;
        }
    }
    Some(counter)
}
