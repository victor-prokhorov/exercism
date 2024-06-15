// Diamond for letter 'E':
// ····A···· 0
//
//
//
// ···BxB··· 1 l+3
// ··C·x·C·· 2
// ·D··x··D· 3
// E···x···E 4 3+l          4: - c - 'A' - 1(edge case)
// ·D·····D·
// ··C···C··
// ···B·B···
//
// ····A····
//
//  0. 4 A 4 - single letter edge case
//  g = letter count from first ('A') to the goal ('E') then first and last:
//
//  g | letter | g
//
//  1. 4 - 1 B 1 4 - 1 
//  2. 4 - 2 C 3 4 - 2
//  3. 4 - 3 D 5 ...
//  4. E 7 ...
//  c = current count
//      +1    +2
//  
//  
//  g - c | letter | curr - 1 + mid ' ' + curr - 1 |
//
//
//
pub fn get_diamond(c: char) -> Vec<String> {
    let mut dia = Vec::new(); // todo: capacity can be known
    let base_len = c as u8 - b'A';
    let full_len = c as u8 - b'A';

    // repeated
    let mut s = String::new();
    for _ in 0..base_len.saturating_sub(1) { s.push(' '); } s.push('A'); for _ in 0..base_len.saturating_sub(1) { s.push(' '); }
    dia.push(s.clone());
    if c as u8 == b'A' {return dia};
    dia[0].push(' ');
    dia[0].insert(0, ' ');


    let mut current_letter = b'B';
    dbg!(c);
    // push all expect first and goal to buff to reuse? no, just use the array you got
    while current_letter != c as u8 + 1 {
        let current_c: char = char::from_u32(current_letter.try_into().unwrap()).unwrap();
        let current_count = current_letter as u8 - b'A';
        dbg!(current_c, current_count);
        // todo: new string and clone on each iter...
        let mut s = String::new();

        // VecDeque to push in the same pattern from middle char whitespace would be better

        for _ in 0..base_len-current_count {s.push(' ')}
        s.push(current_c);
        for _ in 0..current_count-1 {s.push(' ')}
        s.push(' '); // mid ' '
        for _ in 0..current_count-1 {s.push(' ')}
        s.push(current_c);
        for _ in 0..base_len-current_count {s.push(' ')}

        dia.push(s.clone());

        current_letter += 1;
    }

    for i in (1..dia.len()-1).rev() {
        dbg!(&dia[i]);
        dia.push(dia[i].clone());
    }

    // repeated
    let mut s = String::new();
    for _ in 0..base_len.saturating_sub(1) { s.push(' '); } s.push('A'); for _ in 0..base_len.saturating_sub(1) { s.push(' '); }
    dia.push(s.clone());
    if c as u8 == b'A' {return dia};
    let l = dia.len() - 1;
    dia[l].push(' ');
    dia[l].insert(0, ' ');

    dia
}
