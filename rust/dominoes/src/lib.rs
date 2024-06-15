type Dom = (u8, u8);

pub fn chain(input: &[Dom]) -> Option<Vec<Dom>> {
    if input.is_empty() { return Some(Vec::new()) }
    // visibly there a rule you can play `doublet` but only that
    if input.len() == 1 { let d = input.first().unwrap();if d.0 == d.1 {return Some(vec![*d]);}else{return None}}
    let mut input: Vec<Dom> = input.into_iter().cloned().collect();
    // before cmp rotate them all in the same dir
    // probabaly not the most effecient tho
    let mut input:Vec<Dom> = input.iter().map(|d| if d.0<d.1{*d}else{rot(*d)}).collect();
    input.sort_by(|l,r|l.0.cmp(&r.0));
    dbg!(&input);
    let mut r = Vec::new();
    // need also a way for detecting a cycle
    // do we need backtrack variable if we clone the input?
    // for next iteration we could iter over shared borrow then pushing to b instead de cloning
    let mut b: Vec<Dom> = Vec::new();
    loop {
        if r.is_empty() && !input.is_empty() {
            let d = input.remove(0);
            r.push(d);
        } else {
            //                         to pop, normal or need to reverse
            let mut dom_to_pop: Option<(usize, bool)> = None;
            // here we iter to find next domino
            for (i, dom) in input.iter().enumerate() {
                if r.last().unwrap().1 == dom.0 {
                    dom_to_pop = Some((i, true));
                } else if r.last().unwrap().1 == dom.1  {
                    dom_to_pop = Some((i, false));
                }
            }
            // if we found it we remove it from the input 
            // and isnert into resp
            if let Some((i, norm)) = dom_to_pop {
                let d = input.remove(i);
                if norm {
                    r.push(d);
                } else {
                    r.push(rot(d));
                }
            } else {
                // here if wasn't able to build a full chain in one go
                dbg!(&input);
                dbg!(&r);
                break None;
            }
        }
        if input.is_empty() {
            dbg!(&r, &input);
            break Some(r);
        }
    }
}

fn rot(i: Dom) -> Dom {
    let (l, r) = i;
    (r, l)
}
// l is fixed we can rot only r
// fn d_can_match<'a, 'b>(lhs: &'a Dom, rhs: &'b Dom) -> Option<&'b Dom> {
//     if lhs.1 == rhs.1 {
//         return Some(rhs);
//     } else lhs.1 == rhs.0 {
//         return Some();
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_rotate_dom() {
        assert_eq!(rot((1, 2)), (2, 1));
        assert_eq!(rot((10, 20)), (20, 10));
    }
}


        //     if lhs.1 == rhs.1 {
        //         return Some(rhs);
        //     } else lhs.1 == rhs.0 {
        //         return Some();
        //     }
        // dbg!(&r, &b);
        // b.push(r.pop().unwrap());
        // todo: handle empty input when removing
        // if r.is_empty() {
        //     let d = input.remove(0);
        //     r.push(d);
        // } else {

        //     // let d = match input.get(0) {
        //     //     Some(_) => Some(input.remove(0)),
        //     //     None => None,
        //     // };
        //     // if let Some(d) = d {
        //     //     if d.0 == r.last().unwrap().1 {
        //     //         dbg!(0);
        //     //         r.push(d);
        //     //     } else if d.1 == r.last().unwrap().1 {
        //     //         dbg!(1);
        //     //         r.push(rot(d));
        //     //     } else {
        //     //         dbg!(2);
        //     //         b.push(d);
        //     //     }
        //     // } else {
        //     //     break;
        //     // };
        // }

        // break;
