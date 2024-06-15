use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut r: BTreeMap<char, i32> = BTreeMap::new();
    for (k, v) in h {
        for i in v {
            r.insert(i.to_ascii_lowercase(), *k);
        }
    }
    r
}
