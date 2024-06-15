/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// E(x) = (ax + b) mod m
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let m: i32 = 26;
    // check their are co-primes
    if gcd(a, m) == 1 {
    let s: String = plaintext
        .chars()
        .filter(|c| c.is_alphanumeric())
        .enumerate()
        .flat_map(|(i, x)| {
            match x {
                '1'..='9' => {
                    vec![x]
                },
                _ => {
                    let x: i32 = x.to_ascii_lowercase() as i32 - 'a' as i32;
                    let integer_form: i32 = (a * x as i32 + b) % 26 + 'a' as i32;
                    if i != 0 && i % 5 == 0 {
                        vec![
                            ' ',
                            char::from_u32(integer_form.try_into().unwrap()).unwrap(),
                        ]
                    } else {
                        vec![char::from_u32(integer_form.try_into().unwrap()).unwrap()]
                    }
                }
            }
        })
        .collect();
    return Ok(s);
    }
    Err(AffineCipherError::NotCoprime(a))
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// <https://rosettacode.org/wiki/Modular_inverse>
fn mod_inv(a: i32, module: i32) -> i32 {
  let mut mn = (module, a);
  let mut xy = (0, 1);

  while mn.1 != 0 {
    xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
    mn = (mn.1, mn.0 % mn.1);
  }

  while xy.0 < 0 {
    xy.0 += module;
  }
  xy.0
}

/// D(y) = a^-1(y - b) mod m
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    let m = 26;
    if gcd(a, m) == 1 {
        return Ok(ciphertext.chars().filter(|c| c.is_alphanumeric()).map(|x| {
            match x {
                '1'..='9' => x,
                _ => {
                    let y = x as i32 - 'a' as i32;
                    let mut integer_form: i32 = mod_inv(a, m) * (y - b) % m;
                    if integer_form < 0 {
                        integer_form += m;
                    }
                    char::from_u32((integer_form + 'a' as i32).try_into().unwrap()).unwrap()
                }
            }
        }).collect::<String>());
    }
    Err(AffineCipherError::NotCoprime(a))
}
