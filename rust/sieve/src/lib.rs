const PRIME_LEN: usize = 10_010;
const LIMIT: usize = 106_000;

struct PrimeCache {
    primes: [u64; PRIME_LEN],
}

impl PrimeCache {
    const fn new() -> Self {
        let mut primes = [0; PRIME_LEN];
        let mut sieve = [true; LIMIT];

        sieve[0] = false;
        sieve[1] = false;

        let mut i = 2;
        while i * i < LIMIT {
            if sieve[i] {
                let mut j = i * i;
                while j < LIMIT {
                    sieve[j] = false;
                    j += i;
                }
            }
            i += 1;
        }
        let mut num = 0;
        let mut j = 0;
        while num < LIMIT && j < PRIME_LEN {
            if sieve[num] {
                primes[j] = num as u64;
                j += 1;
            }
            num += 1;
        }
        Self { primes }
    }
}

static CACHE: PrimeCache = PrimeCache::new();

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut nth = 0;
    if upper_bound < 2 {
        return vec![];
    }
    while CACHE.primes[nth] <= upper_bound {
        nth += 1;
    }
    let slice = &CACHE.primes[0..nth];
    slice.to_vec()
}
