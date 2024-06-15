#[must_use]
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = (2..=upper_bound).collect();
    let mut i: usize = 0;
    while i < primes.len() {
        let mut composite: u64 = primes[i] + primes[i];
        while composite <= upper_bound {
            if let Some(p) = primes.iter().position(|x| *x == composite) {
                primes.remove(p);
            }
            composite += primes[i];
        }
        i += 1;
    }
    primes
}
