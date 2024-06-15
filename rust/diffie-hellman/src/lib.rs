use num::{BigUint};
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut primes: Vec<u64> = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 773, 967, 3461, 6131,
    ];
    primes.retain(|&x| x < p);
    let mut rng = rand::thread_rng();
    primes[rng.gen_range(0..primes.len())]
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // A = g**a mod p
    let p = BigUint::from(p);
    let g = BigUint::from(g);
    let a = BigUint::from(a);
    *g.modpow(&a, &p).to_u64_digits().get(0).unwrap()
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // s = B**a mod p
    let p = BigUint::from(p);
    let b_pub = BigUint::from(b_pub);
    let a = BigUint::from(a);
    *b_pub.modpow(&a, &p).to_u64_digits().get(0).unwrap()
}
