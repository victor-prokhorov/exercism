// todo: cloned -> boxed vec -> hash?
// #[derive(Debug, PartialEq, Eq)]
#[derive(Debug)]
pub struct CustomSet<T: Copy + std::cmp::PartialEq + std::fmt::Debug + std::cmp::Ord> {
    data: Vec<T>,
}
impl<T: Copy + std::cmp::PartialEq + std::fmt::Debug + std::cmp::Ord> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        // i think this will make tests pass but i have a doubt
        // not sure if repetitve elmenets will work like
        // [a, a, b] == [a, b, b] i think my impl will fail here
        // track occurences
        if self.data.len() == other.data.len() {
            for i in self.data.iter() {
                if !other.contains(i) {
                    return false;
                }
            }
            return true;
        }
        false
    }
}
impl<T: Copy + std::cmp::PartialEq + std::fmt::Debug + std::cmp::Ord> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        Self {
            data: input.iter().cloned().collect(),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    pub fn add(&mut self, element: T) {
        dbg!(&self.data, &element);
        if !self.contains(&element) {
            self.data.push(element);
        }
        self.data.sort();
        dbg!(&self.data, &element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        // todo:
        // first implement naive
        // then check for better algo
        //for i in self.data.iter() {
        if self.is_empty() {
            return true;
        }
        if self.data.len() == 0 && other.data.len() == 0 {
            return true;
        }
        if let Some(p) = other.data.iter().position(|x| *x == self.data[0]) {
            // from that point, find index of each element
            // give offset to each
            // check if all elements are egual

            // probabaly can quickly check for len
            if self
                .data
                .iter()
                .zip(other.data.iter().skip(p))
                .all(|(l, r)| l == r)
            {
                return true;
            }
        }
        // could have derived that from interesection
        false
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        // take any, iter over each elem if found return false
        // could have user intersection for that
        for i in self.data.iter() {
            if other.contains(i) {
                return false;
            }
        }
        true
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        dbg!(&self.data, &other.data);
        let mut data = Vec::new();

        // it's actually dosent matter if the order
        // probably order them once then so it's not done on every call
        // avoid cloning and sorting eaaach time, do it once on creation
        let mut s = self.data.clone();
        s.sort();
        let mut o = other.data.clone();
        o.sort();
        dbg!(&s, &o);
        //if s.len() > o.len() {
        // probably can check for len
            for (count, i) in s.iter().enumerate() {
                dbg!(&i);
                if let Some(p) = o.iter().position(|x| *x == *i) {
                    dbg!(&p);
                    s.iter()
                        .skip(count)
                        .zip(o.iter().skip(p))
                        .for_each(|(l, r)| {
                            println!("====");
                            dbg!(&l, &r);
                            if l == r {
                                if !data.contains(l) {
                                    data.push(*l);
                                }
                            }
                        })
                }
            }
        // } else {
        // }
        data.sort();
        dbg!(&data);
        Self { data }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut diff = Vec::new();
        for i in self.data.iter() {
            if !other.contains(i) {
                diff.push(*i)
            }
        }
        diff.sort();
        Self { data: diff }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut uni = self.data.clone();
        let mut order = true;
        if uni.len() > 2 {
            order = uni[0] < uni[1];
        }

        // uni.sort();
        for i in other.data.iter() {
            if !uni.contains(i) {
                uni.push(*i);
            }
        }
        // do this need to be sorted?
        uni.sort_by(|l, r| if order { r.cmp(l) } else { l.cmp(r) });
        dbg!(&uni, &self.data, &other.data);
        Self { data: uni }
    }
}
