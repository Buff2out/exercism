fn main() {
    let n: u64 = std::env::args().nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(35);

    println!("{} -> {}", n, if is_prime(n) { "prime" } else { "composite" });
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 { return false; }

    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;

    let witnesses = get_witnesses(n);

    for &a in witnesses {
        if !miller_rabin_test(a, d, n, s) {
            return false;
        }
    }
    true
}

// O(1) lookup based on input size
fn get_witnesses(n: u64) -> &'static [u64] {
    const WITNESSES: &[(u64, &[u64])] = &[
        (2_046, &[2]),
        (1_373_652, &[2, 3]),
        (9_080_190, &[31, 73]),
        (25_326_000, &[2, 3, 5]),
        (4_759_123_140, &[2, 7, 61]),
        (1_112_004_669_632, &[2, 13, 23, 1662803]),
        (2_152_302_898_746, &[2, 3, 5, 7, 11]),
        (3_474_749_660_382, &[2, 3, 5, 7, 11, 13]),
        (341_550_071_728_320, &[2, 3, 5, 7, 11, 13, 17]),
        (3_825_123_056_546_413_050, &[2, 3, 5, 7, 11, 13, 17, 19, 23]),
        (u64::MAX, &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]),
    ];

    WITNESSES.iter()
        .find(|(limit, _)| *limit >= n)
        .map(|(_, w)| *w)
        .unwrap()
}

fn miller_rabin_test(a: u64, d: u64, n: u64, s: u32) -> bool {
    let mut x = mod_pow(a, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }

    for _ in 1..s {
        x = mod_mul(x, x, n);
        if x == n - 1 {
            return true;
        }
    }
    false
}

// === Math Core ===

#[inline]
fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

fn mod_pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut res = 1;
    base %= m;
    while exp > 0 {
        if exp % 2 == 1 {
            res = mod_mul(res, base, m);
        }
        base = mod_mul(base, base, m);
        exp /= 2;
    }
    res
}
