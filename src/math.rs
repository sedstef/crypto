
pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut divisor = 2;

    while divisor * divisor <= n {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }
        divisor += 1;
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}

