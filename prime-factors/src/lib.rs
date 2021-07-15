pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut factors = Vec::<u64>::new();
    loop {
        if let Some(x) = (2..n).find(|x| n % x == 0) {
            factors.push(x);
            n = n / x;
        } else {
            factors.push(n);
            return factors;
        }
    }
}
