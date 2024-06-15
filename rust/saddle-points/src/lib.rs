#[must_use]
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut v = vec![];
    for (row_index, row) in input.iter().enumerate() {
        for (column_index, value) in row.iter().enumerate() {
            if row.iter().all(|other_value| value >= other_value)
                && input
                    .iter()
                    .all(|other_row| *value <= other_row[column_index])
            {
                v.push((row_index, column_index));
            }
        }
    }

    v
}
