pub fn factors(n: u64) -> Vec<u64> {
    let mut primeFactors: Vec<u64> = Vec::new();
    let mut prime = 2;
    let mut number = n;
    loop {
        if number % prime == 0 {
            primeFactors.push(prime);
            number /= prime;
        } else {
            prime += 1;
        }
        if number == 1 {
            break;
        }
    }
    primeFactors
}