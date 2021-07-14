pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut prime_counter: u32 = 1;
    let mut prime_n: u32 = 0;
    while prime_n <= n {
        prime_counter += 1;
        if isprime(prime_counter) {
            prime_n += 1;
        }
    }
    prime_counter
}

pub fn isprime(n: u32) -> bool {
    let mid_n = (n as f64).sqrt() as u32;
    if n == 1 {
        return false;
    } else {
        !(2..=mid_n).any(|x| n % x == 0)
    }
}
