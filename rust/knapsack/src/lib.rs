pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    items
        .iter()
        .enumerate()
        .filter_map(|(i, item)| {
            if item.weight > max_weight {
                None
            } else {
                Some(item.value + maximum_value(max_weight - item.weight, &items[i + 1..]))
            }
        })
        .max()
        .unwrap_or(0)
}
