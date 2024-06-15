use std::cmp::Ordering;

pub fn find<R: AsRef<[T]>, T: Ord>(r: R, key: T) -> Option<usize> {
    let r_ref = r.as_ref();

    let m = r_ref.len() / 2;

    match r_ref.get(m)?.cmp(&key) {
        Ordering::Greater => {
            if let Some(x) = find(&r_ref[..m], key) {
                return Some(x);
            }
        }
        Ordering::Less => {
            if let Some(x) = find(&r_ref[m + 1..], key) {
                return Some(x + m + 1);
            }
        }
        Ordering::Equal => return Some(m),
    }

    None
}
