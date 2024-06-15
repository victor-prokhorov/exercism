#![feature(if_let_guard)]
/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = str::replace(isbn, "-", "");
    if isbn.is_empty() || isbn.len() != 10  {
        return false;
    }
    let mut op: i32 = 10;
    let mut s: i32 = 0;
    for i in isbn.chars() {
        match i {
            'X' if op == 1 => { s += 10 * op; op -= 1; },
            '-' => (),
            _ if let Some(x) = i.to_digit(10) => { s += x as i32 * op; op -= 1; },
            _ => return false,
        }
    }
    if op != 0 {
        return false;
    }
    
    s % 11 == 0
}
