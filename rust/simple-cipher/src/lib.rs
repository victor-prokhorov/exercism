use rand::Rng;

// key is shift amount, s is the message
pub fn encode(key: &str, s: &str) -> Option<String> {
    dbg!(&key,&s);
    // unimplemented!("Use {key} to encode {s} using shift cipher")
    // let first = s.chars().next().unwrap() as i32 -97;
    // dbg!(first); // the shift amount
    // let buf = Vec::with_capacity(26);
    // Some(key.chars().map(|c| {
    //     dbg!(c);
    //     char::from(((c as i32 - 97 + first as i32)%26+97)as u8)
    // }).collect())
    // debug_assert!(key.len() == s.len()); // nope apearently it should handle that case
    let mut key=key.to_string();
    if !key.chars().all(|c|c.is_alphabetic()&&('a'..='z').contains(&c)){return None;}
    if s.is_empty()||key.is_empty(){return None;}
    if s.len()>key.len(){for _ in 0..s.len()-key.len(){key.push('a')}}
    dbg!(&key);
    Some(key.chars().zip(s.chars()).map(|(l,r)| {dbg!(l,r);char::from( (dbg!((r as i8-97) + (l as i8 - 97))%26+97)as u8 )}).collect())

}

pub fn decode(key: &str, s: &str) -> Option<String> {
    // unimplemented!("Use {key} to encode {s} using shift cipher")
    // this can be check_if_valid or something to use in both
    if !key.chars().all(|c|c.is_alphabetic()&&('a'..='z').contains(&c)){return None;}
    if s.is_empty()||key.is_empty(){return None;}
    dbg!(&key,&s);
    let mut key=key.to_string();
    if s.len()>key.len(){for _ in 0..s.len()-key.len(){key.push('a')}}
    Some(key.chars().zip(s.chars()).map(|(l,r)| char::from( ((26+dbg!(r as i8-97 - (l as i8 - 97)))%26+97)as u8 )).collect())
    //                                                                     ^ can probably extract
    //                                                                     this to a fn
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let mut key = String::new();
    for _ in 0..=100 { key.push(rng.gen_range('a'..='z')); }
    (key.to_string(),encode(&key,s).unwrap())
    //unimplemented!(
    //    "Generate random key with only a-z chars and encode {s}. Return tuple (key, encoded s)"
    //)
}
