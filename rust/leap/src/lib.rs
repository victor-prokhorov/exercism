pub fn is_leap_year(year: u64) -> bool {
    // https://en.wikipedia.org/wiki/Leap_year#Algorithm
    match (year % 400 == 0, year % 100 == 0, year % 4 == 0) {
        // if (year is not divisible by 4) then (it is a common year)
        (_, _, false) => false,
        // else if (year is not divisible by 100) then (it is a leap year)
        (_, false, _) => true,
        // else if (year is not divisible by 400) then (it is a common year)
        (false, _, _) => false,
        _ => true,
    }
}