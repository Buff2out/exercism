pub trait PrimeCollector {
    fn get_counted(&mut self, prime: u64) -> Vec<u64>;
}

impl PrimeCollector for u64 {
    fn get_counted(&mut self, prime: u64) -> Vec<u64> {
        let mut res: Vec<_> = Vec::new();
        while 0 == *self % prime {
            *self /= prime;
            res.push(prime);
        }
        res
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut res: Vec<_> = Vec::new();

    let mut n_remained = n;
    let mut i = 2;
    while i * i <= n_remained {
        if n_remained % i == 0 {
            res.extend(n_remained.get_counted(i));
        }
        i += 1;
    }

    if n_remained > 1 {
        res.push(n_remained);
    }
    res
}
