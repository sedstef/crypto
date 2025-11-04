
pub fn prime_factors(mut n: usize) -> Vec<usize> {
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

pub fn get_primes(moduli: usize) -> Vec<usize> {
    let mut primes = Vec::new();
    for number in 0..=moduli {
        if is_prime(number) {
            primes.push(number as usize);
        }
    }
    primes
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

pub fn remainder_table(moduli: usize, function: fn(usize, usize) -> usize) -> Vec<Vec<usize>> {
    let mut data = Vec::new();

    for row in 0..=moduli {
        let mut row_data = Vec::new();
        for col in 0..=moduli {
            let value = function(row, col).rem_euclid(moduli);
            row_data.push(value);
        }
        data.push(row_data);
    }
    data
}

pub fn get_gcd(mut a: usize, mut b: usize, rows: &mut Vec<EuclideanRow> ) -> usize {
    while b != 0 {
        let quotient = a / b;
        let remainder = a % b;

        rows.push(EuclideanRow {
            a,
            b,
            quotient,
            remainder,
        });

        // advance the algorithm
        a = b;
        b = remainder;
    }
    a
}

pub struct EuclideanRow {
    pub a: usize,
    pub b: usize,
    pub quotient: usize,
    pub remainder: usize,
}


#[cfg(test)]
mod tests {
    // bring outer symbols into scope
    use super::*;

    #[test]
    fn test_is_prime() {
        let cases = [
            (1, false),
            (2, true),
            (3, true),
            (5, true),
            (7, true),
            (29, true),
        ];

        for (value, expected) in cases {
            assert_eq!(is_prime(value), expected, "Failed on input ({})", value);
        }
    }

    #[test]
    fn test_get_gcd() {
        let mut rows: Vec<EuclideanRow> =  Vec::new();
        let gcd = get_gcd(54, 24, &mut rows);
        assert_eq!(gcd, 6, "Failed on input ({})", gcd);
        for row in rows{
            println!("{} = {} * {} + {}", row.a, row.b, row.quotient, row.remainder);
        }
    }
}
