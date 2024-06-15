pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    if n == 1 {
        return 3;
    }
    let mut prime_count = 2;
    let mut candidate = 5;
    let mut last_prime = 3;
    while prime_count <= n {
        if is_prime(candidate) {
            last_prime = candidate;
            prime_count += 1;
        }
        candidate += 2;
    }
    last_prime
}

fn is_prime(n: u32) -> bool {
    if n == 2 || n == 3 {
        return true;
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    for i in (5..=(n as f32).sqrt() as u32).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
    }
    true
}
