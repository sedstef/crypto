// --- Prime factorization (trial division) ---
pub fn prime_factors(mut n: u64) -> Vec<(u64, u32)> {
    let mut res = Vec::new();
    if n < 2 {
        return res;
    }


    if n % 2 == 0 {
        let mut count = 0;
        while n % 2 == 0 {
            n /= 2;
            count += 1;
        }
        res.push((2, count));
    }


    let mut p = 3u64;
    while p.checked_mul(p).map_or(false, |pp| pp <= n) {
        if n % p == 0 {
            let mut count = 0u32;
            while n % p == 0 {
                n /= p;
                count += 1;
            }
            res.push((p, count));
        }
        p += 2;
    }


    if n > 1 {
        res.push((n, 1));
    }


    res
}

// Define a function named 'is_prime' that takes a number as parameter and returns true if it's prime, false otherwise
pub fn is_prime(num: usize) -> bool {
    if num <= 1 {
        return false; // Numbers less than or equal to 1 are not prime
    }

    // Check if num is divisible by any number from 2 to the square root of num
    for i in 2..=(num as f64).sqrt() as usize {
        if num % i == 0 {
            return false; // If num is divisible by any number other than 1 and itself, it's not prime
        }
    }

    true // If num is not divisible by any number other than 1 and itself, it's prime
}
