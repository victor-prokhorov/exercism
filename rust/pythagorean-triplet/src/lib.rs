use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
//     unimplemented!("Given the sum {sum}, return all possible Pythagorean triplets, which produce the said sum, or an empty HashSet if there are no such triplets. Note that you are expected to return triplets in [a, b, c] order, where a < b < c");
    // a + b + b = sum; and a^2+b^2=c^2
    // dbg!(200i32.pow(2)+375i32.pow(2)==425i32.pow(2));
    // dbg!(200i32+375i32+425i32==1000);
    // 200i32<375i32<425i32
    // dbg!(sum); // 12
    // it can't have zero
    // the smalest possible is
    // 1,2,3 is it 12? nope can i start to decompose from 12?
    // 1,2,(12-1-2)=9 the sum is ok
    // 1+4=36 nope
    // start from c 
    // there 2 numbers smaller then c so c is at max: 12 - 2 = 10, and min: 3
    // since a^2+b^2=c^2, 10^2 = 100, 9^2=81, 100 - 81=19, if 19.sqrt() + 9 = 10,
    // dbg!(425f32.sqrt(), 425f32.sqrt().ceil());
    // fract Returns the fractional part of self.
    // dbg!(19f32.sqrt().fract()==0.0); // if true we can do loseless conversion

    let mut s = HashSet::new();
    // can't be zero, - 2 from for the end for b and c
    // however if a 9 b 10 c 11 = 12
    // what is the last realistic number of a?
    // we can vaguely cut it, if we imagine them all equal and then add 2 to make sure their
    // respect a < b < b
    // actually + 1 for a and b is +2 so +3 total
    // so 3*a + 3 = 12; 3a = 9; a = 3
    // so a_max = (sum-3)/3
    // not sure about roudning tho
    for a in 1..=(sum-3)/3 {
        // dbg!(a);
        // i^2 +   i+1^2   +    i+2^2 = sum^2 
        // brute force from here?
        // b_min = i + 1; b_max = sum (well not really)
        // c_min = i + 2; c_max = sum (well not really)
        for b in a+1..sum-a{ // we can sub a
            // dbg!(b);
            // for c in b+1..sum-b{ // and chain deps instead of using a
            // we can deduce c
            let c = sum - a - b;
                // dbg!(c);
                if a.pow(2) + b.pow(2) == c.pow(2){
                    s.insert([a,b,c]);
                }
            // }
        }
    }
    s
}
