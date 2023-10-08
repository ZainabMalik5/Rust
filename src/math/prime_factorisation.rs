fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    // Handle 2 as a special case
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    // Check for other prime factors
    let mut divisor = 3;
    while divisor * divisor <= n {
        while n % divisor == 0 {
            factors.push(divisor);
            n /= divisor;
        }
        divisor += 2; // Skip even numbers
    }

    // If n is still greater than 1, it's a prime factor itself
    if n > 1 {
        factors.push(n);
    }

    factors
}

fn main() {
    let num = 36; // Replace with the number you want to factorize
    let factors = prime_factors(num);

    println!("Prime factors of {} are: {:?}", num, factors);
}
