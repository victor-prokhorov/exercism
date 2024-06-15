//  1. normalize
//  the spaces and punctuation are removed from the English text and the message is downcased
//   2. organize into rects
//  c - number of col
//  r - row
//  r x c - size of rect should respect c >= r && c - r <= 1
// 3. The coded message is obtained by reading down the columns going left to right
// 4. (r X c), with c chunks of r length, pad with ' ' to have filled rows
//

pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    let mut res = String::new();

    let mut normalized: Vec<char> = input
        .chars()
        // flat_map
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    // dbg!(&normalized);
    // dbg!(normalized.len());
    // c >= r && c - r <= 1
    // 54
    // 49:-5 64:+10
    let len = normalized.len() as u32;
    // dbg!(binary_search_sqrt(normalized.len() as u32));
    let mut c = binary_search_sqrt(normalized.len() as u32);
    let mut r = 0;
    //
    if c.pow(2) == len {
        r = c;
    } else if c.pow(2) > len {
        // 49
        r = c - 1;
    } else {
        let tmp = c;
        c += 1;
        r = tmp;
    }
    debug_assert!(c >= r);
    debug_assert!(c - r <= 1);
    // dbg!(c, r, c as i32 - r as i32, c >= r);
    // fix missing whitespaces
    while normalized.len() < (c * r) as usize {
        normalized.push(' ');
    }
    let rect: Vec<_> = normalized.chunks(c as usize).collect();
    dbg!(c, r, &rect);
    let mut need_whitespace = 0;
    // todo: can be done without indexing
    for j in 0..rect[0].len() {
        for i in 0..rect.len() {
            // dbg!(rect[i][j]);
            // ugly af to refactor
            if need_whitespace == r {
                need_whitespace = 0;
                res.push(' ');
            }
            res.push(rect[i][j]);
            need_whitespace += 1;
        }
    }
    // dbg!(&res);
    res
}

// pseudo code from wikipedia
// function binary_search(A, n, T) is
//     L := 0
//     R := n − 1
//     while L ≤ R do
//         m := floor((L + R) / 2)
//         if A[m] < T then
//             L := m + 1
//         else if A[m] > T then
//             R := m − 1
//         else:
//             return m
//     return unsuccessful

fn binary_search_sqrt(n: u32) -> u32 {
    let mut low = 0;
    let mut high = n;
    let mut mid = 0;

    while low <= high {
        mid = (low + high) / 2;
        if mid.pow(2) == n {
            return mid;
        }
        if mid.pow(2) < n {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    mid
}
