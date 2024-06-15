#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    if num == 1 {
        return Some(Classification::Deficient);
    }

    // let mut v: Vec<u64> = vec![1];
    // let l = f64::sqrt(num as f64) as u64;
    // for i in 2..=l {
    //     if num % i == 0 {
    //         if num / i == i {
    //             v.push(i);
    //         } else {
    //             v.push(i + num / i)
    //         }
    //     }
    // }

    match (2..=f64::sqrt(num as f64) as u64)
        .fold(1, |acc, x| {
            if num % x == 0 {
                if num / x == x {
                    return acc + x;
                }
                return acc + x + num / x;
            }
            acc
        })
        .cmp(&num)
    {
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Less => Some(Classification::Deficient),
        std::cmp::Ordering::Greater => Some(Classification::Abundant),
    }
}
