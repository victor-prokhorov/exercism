use std::collections::HashSet;

const SET_PRICES: [u32; 5] = [800, 1520, 2160, 2560, 3000];

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut sets = Vec::<HashSet<u32>>::new();
    for book in books.iter() {
        if let Some(set) = sets
            .iter_mut()
            // find sets selection where there still no book
            .filter(|set| !set.contains(&book))
            // can't overflow because we are only checking for sets that still dosen't contains this book
            // can't unerflow because `min_by_key` return `None` if the iterator is empty without executing closure
            // we simulate what would be the price of this book if we add it to the set
            // but we are interested in the highest discount possible
            .min_by_key(|set| SET_PRICES[set.len()] - SET_PRICES[set.len() - 1])
        {
            set.insert(*book);
        } else {
            // if we can't insert the book into existing set to get higher discount
            // we will start a new set with this book
            sets.push(HashSet::from([*book]));
        }
    }
    // can't underflow here neither because `else` the closure wan't be called on empty iterator
    sets.iter().map(|set| SET_PRICES[set.len() - 1]).sum()
}
