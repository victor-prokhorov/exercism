pub fn verse(n: u32) -> String {
    // let m = n-1;  // thread 'test_song_3_0' panicked at 'attempt to subtract with overflow'
    let a = match n {
        1 => format!("{n} bottle"),
        0 => format!("No more bottles"),
        _ => format!("{n} bottles"),
    };
    let b = match n {
        1 => format!("Take it down and pass it around, no more bottles of beer on the wall."),
        0 => format!("Go to the store and buy some more, 99 bottles of beer on the wall."),
        2 => format!("Take one down and pass it around, {} bottle of beer on the wall.", n-1), // {m}
        _ => format!("Take one down and pass it around, {} bottles of beer on the wall.", n-1), // {m}
    };
    let c = a.to_lowercase();
    // println!(format!("{a} of beer on the wall, {a} of beer.\n{b}\n")); // error
    println!("{a} of beer on the wall, {c} of beer.\n{b}\n");
    format!("{a} of beer on the wall, {c} of beer.\n{b}\n").to_string()
    
}
pub fn sing(start: u32, end: u32) -> String {
    let mut x = String::new();
    for n in end..start+1
    {
        x = verse(n) + "\n" + &x; //  x = x + verse(n);    expected `&str`, found `String`   wat???
    }
    x.trim().to_owned() + "\n"
}