// naiv algo can be
// start iter row col
// for each + go: right down left top (each time until you met +)
// did you went to initial poisiont in 4 moves?
// yes -> +1 rectangle
// continue for each + 
// but it dosen't count the composite rectangles 
// if rectangles have 2 same points we add 1 rect to counts
//
// permutationds of all rects
// window of 2 on each rect seems ineffecient af
//
// try to loop on rect, with depth of 1
// increase until you hit the wall
// change dpeth separately for x and y 
// 
// +-+-+
// +-+-+
// +-+-+
//
// (x, y) -> limit? 
// if depth 1 was a success,
// try (+1, _) then (_, +1)
//
// so:
// 1. iter until hit +, consider it as the most top left corner
// 2. find limit on both axes, until you hit the terrain limit
// 3. verify if can produce the rectable from each possibility
// 4. ltr -> ttb -> rtl -> btt, did you went on initial position ? yes -> count += 1



pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() { return 0; }
    let mut c = 0;
    let m: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let w = lines[0].len();
    let h = lines.len();
    dbg!(w, h);

    for i in 0..h { // i - to index in height
        for j in 0..w { // j - to index in width
            if m[i][j] == '+' {
                println!("potential rect, left top corner ({i}, {j})");
                // dbg!(&m[i]);
                // we go to the limit while checking if can build a rect
                for x in j+1..w {
                    // check if valid lign
                    if m[i][x] != '-' && m[i][x] != '+'  {
                        break;
                    }
                    if m[i][x] == '+' {
                        // now we need to check if we can ttb
                        println!("we are ({i}, {x}) about to check if we can ttb");
                        for y in i+1..h {
                            if m[y][x] != '|' && m[y][x] != '+'  {
                                break;
                            }
                            if m[y][x] == '+' {
                                println!("at ({y}, {x}) -> rtl now");
                                for rx in (0..x).rev() {
                                    // dbg!(rx, m[y][rx]);
                                    if m[y][rx] != '-' && m[y][rx] != '+'  {
                                        break;
                                    }
                                    if m[y][rx] == '+' {
                                        println!("at ({y}, {rx}) -> btt now");

                                        for ry in (0..y).rev() {
                                            dbg!(m[ry][rx]);
                                            if m[ry][rx] != '|' && m[ry][rx] != '+' {
                                                break;
                                            }
                                            if m[ry][rx] == '+' {
                                                println!("bingo? at ({ry}, {rx})");
                                                if ry == i && rx == j {
                                                    c += 1;
                                                    dbg!(c);
                                                }
                                            }
                                        }

                                    }
                                    
                                }
// panic!(); // --------------------------------------------------
//         "  +-+",
//         "  | |",
//         "+-+-+",
//         "| | |",
//         "+-+-+",
                            }
                        }
                    }
                }
            }
        }
    }

    c
}
