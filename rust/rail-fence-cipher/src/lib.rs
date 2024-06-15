// pub struct RailFence { rails: Vec<Vec<char>> } 
pub struct RailFence { rails: u32 } 
// pair
// x . . . . . x . o . . . h + 1         h to get to the bottom
// . x . . . x . x . o . . h - 1 h - 3   half = h - 1 (self col) - 1 (ending column)
// . . x . x . o . x . o . h - 3 h - 1   dist between h x 2 + 1
// . . . x . o . . . x . o h + 1
//         o              o
//         o is + 1
// half = 4-1-1 = 2
// step = 2*2+1 = 5, ok
// 
// for each next step, we need to substract 1 from the half
// half = 2-1 = 1
// step = 2*1+1=3,ok, this one will alternate with the next one
//
// step = 2*0 + 1 = 1, ok , -> 1 3 1 3
// unpair
//
// the last one is the same as the first one but with offset of h = 3
// x . . . x . . . . . . . . . .  3 - 1 - 1, 1 * 2 + 1 = 3, ok, half is 1
// . x . x . x . . . . . . . . .  1, 1, 1
// . . x . . . x . . . . . . . .  
// wasnt able to find the formula, will just do literal zigzaging back and forwrard
#[derive(Copy, Clone)]
enum Dir { Down, Up } // down is +1 +1 up is +1 - 1
                      // direction for y basicaly
impl RailFence {
    pub fn new(rails: u32) -> Self {
        // Self { rails: vec![vec![];rails as usize] }
        Self { rails }
    }

    pub fn encode(&self, text: &str) -> String {
        // it takes rails.len() to reach the bottom
        let len = self.rails as usize;
        let mut rails: Vec<Vec<char>> = vec![vec![];len];
        let (mut x, mut y, mut d) = (0, 0, Dir::Down);
        for c in text.chars() {
            // rails[y][x] when encoding we don't need to track [x]
            rails[y].push(c);
            match (y, d) {
                // hit the bottom limit
                (y_delta, Dir::Down) if y_delta == len-1 =>{d = Dir::Up;y=len-1-1;},
                (y_delta, Dir::Up) if y_delta == 0 =>{d=Dir::Down;y=0+1},
                (_, Dir::Down) =>{y+=1;},
                (_, Dir::Up) =>{y-=1},
                _=>panic!("well in theory we should not hit that"),
            }
        }
        // for (i, rail) in rails.iter_mut().enumerate() {
        //     dbg!(i, &rail);
        //     let half = len - 1 - 1;
        //     for c in text.chars().skip(i).step_by(half * 2 + 1).take(3) { rail.push(c); }
        // }
        let mut s = String::new();
        for rail in rails.iter() {
            for c in rail { s.push(*c); }
        }
        // dbg!(&rails);
        // panic!();
        s
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut s = String::new();
        // let len = cipher.len();
        let len = self.rails as usize;
        let mut rails: Vec<Vec<char>> = vec![vec![];len];
        let mut v = vec![0;len];
        // for (i, _) in cipher.chars().enumerate() {
        //     let index = i % len;
        //     v[index] = v[index]+1;
        // }
        let (mut x, mut y, mut d) = (0, 0, Dir::Down);
        // calc how many we should pack into each row
        for c in cipher.chars() {
            v[y] = v[y] + 1;
            match (y, d) {
                (y_delta, Dir::Down) if y_delta == len-1 =>{d = Dir::Up;y=len-1-1;},
                (y_delta, Dir::Up) if y_delta == 0 =>{d=Dir::Down;y=0+1},
                (_, Dir::Down) =>{y+=1;},
                (_, Dir::Up) =>{y-=1},
                _=>panic!("well in theory we should not hit that"),
            }
        }

        let mut cipher_chars = cipher.chars();
        let c = cipher_chars.by_ref();
        for (i, count) in v.iter().enumerate() {
            c.take(*count).for_each(|letter| {rails[i].push(letter)});
        }
        dbg!(&rails);
        dbg!(self.rails,&v,cipher.len());
        // 25 = 7 + 12 + 6
        // height = 3 

        // todo: refine this loop which is reused evertywhere
        // now get values
        let (mut x, mut y, mut d) = (0, 0, Dir::Down);
        for c in cipher.chars() {
            // rails[y][x] when encoding we don't need to track [x]
            let removed = rails[y].remove(0);
            s.push(removed);
            match (y, d) {
                // hit the bottom limit
                (y_delta, Dir::Down) if y_delta == len-1 =>{d = Dir::Up;y=len-1-1;},
                (y_delta, Dir::Up) if y_delta == 0 =>{d=Dir::Down;y=0+1},
                (_, Dir::Down) =>{y+=1;},
                (_, Dir::Up) =>{y-=1},
                _=>panic!("well in theory we should not hit that"),
            }
        }
        s
            

    }
}

// 17 chars, 
// e.......i.......e....... 3
// .x.....m.s.....m........
// ..e...s...a...o...........
// ...r.i.....w.s...............
// ....c.......e.............
