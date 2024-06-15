// #![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    integers: Vec<Value>,
    errors: Vec<Error>,
    hm: std::collections::HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            integers: Vec::new(),
            errors: Vec::new(),
            hm: std::collections::HashMap::new(),
        }
    }

    #[must_use]
    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    ///# Errors
    ///
    /// Will throw `StackUnderflow` if wasn't able to perform operation.
    /// TODO: add other cases
    pub fn eval<'a>(&mut self, input: &'a str) -> Result {
        dbg!("**************", &input);

        let mut ops: Vec<&str> = vec![];

        // while input.find(':').is_some() {
        //     dbg!("WHILE LET SOME");
        let mut v_i = vec![];
        let vars_count = input
            .chars()
            .enumerate()
            .filter(|&c| {
                let r = c.1 == ':';
                if r {
                    v_i.push(c.0);
                }
                r
            })
            .count();
        dbg!(&v_i);
        // merge this and the enxt
        if vars_count == 0 {
            // let search_inside = &input[s..];

            let start_index = input.find(':');
            let end_index = input.find(';');

            // dbg!(&search_inside);
            // dbg!(&start_index.unwrap() + s, &end_index.unwrap() + s);

            match (start_index, end_index) {
                (Some(_), None) | (None, Some(_)) => return Err(Error::InvalidWord),
                (Some(s), Some(e)) => {
                    // let s = s + v_i[i];
                    // let e = e + v_i[i];

                    let var = &input[s + 2..e - 1];
                    let v: Vec<&str> = var.split(' ').collect();

                    if self.hm.contains_key(&v[0].to_string().to_ascii_uppercase()) {
                        // reassigning the same name
                        // since recursion is used we need to expand it right now

                        let mut expans_vec = v[1..].into_iter().map(|&x| x).collect::<Vec<&str>>();

                        let pos = expans_vec.iter().position(|&e| {
                            e.to_ascii_uppercase() == v[0].to_string().to_ascii_uppercase()
                        });

                        // std::mem::replace(&expans_vec[pos.unwrap()], *self.hm.get(&v[0].to_string().to_ascii_uppercase()))
                        let prev = self
                            .hm
                            .get(v[0].to_string().to_ascii_uppercase().as_str())
                            .unwrap();
                        if let Some(i) = pos {
                            std::mem::replace(&mut expans_vec[i], prev);
                        }

                        // dbg!(&v[0].to_string().to_ascii_uppercase());

                        // dbg!(&self.hm);

                        self.hm.insert(
                            v[0].to_string().to_ascii_uppercase(),
                            // var_expans.to_ascii_uppercase(),
                            expans_vec.join(" ").to_ascii_uppercase(),
                        );

                        // dbg!(&self.hm);

                        let mut l_ops: Vec<&str> = input[..s].split(' ').collect();
                        let r_ops: Vec<&str> = input[e + 1..].split(' ').collect();

                        dbg!(&l_ops, &r_ops);
                        l_ops.extend(r_ops);
                        ops = l_ops.into_iter().filter(|&s| !s.is_empty()).collect();
                    } else {
                        // here need to expand if collision
                        let var_expans = v[1..].join(" ");

                        // should be in previous block as well
                        if v[0].chars().next().unwrap().is_numeric() {
                            return Err(Error::InvalidWord);
                        }

                        // dbg!(&var_expans);
                        if self.hm.contains_key(&var_expans) {
                            let ns = self.hm.get(&var_expans).unwrap();
                            self.hm.insert(
                                v[0].to_string().to_ascii_uppercase(),
                                // var_expans.to_ascii_uppercase(),
                                // *self.hm.get(&var_expans).unwrap().to_string(),
                                (*ns.clone()).to_string(),
                            );
                        } else {
                            dbg!(&self.hm);
                            if self.hm.contains_key(&var_expans.to_ascii_uppercase()) {
                                let ns = self.hm.get(&var_expans.to_ascii_uppercase()).unwrap();
                                self.hm.insert(
                                    v[0].to_string().to_ascii_uppercase(),
                                    ns.to_ascii_uppercase(),
                                );
                            } else {
                                self.hm.insert(
                                    v[0].to_string().to_ascii_uppercase(),
                                    var_expans.to_ascii_uppercase(),
                                );
                            }
                            dbg!(&self.hm);
                        }

                        let mut l_ops: Vec<&str> = input[..s].split(' ').collect();
                        let r_ops: Vec<&str> = input[e + 1..].split(' ').collect();
                        dbg!(&l_ops, &r_ops);

                        l_ops.extend(r_ops);
                        ops = l_ops.into_iter().filter(|&s| !s.is_empty()).collect();
                    }
                }
                _ => ops = input.split(' ').collect::<Vec<&str>>(),
            }
        }

        for i in 0..vars_count {
            let s = v_i[i];
            let search_inside = &input[s..];

            let start_index = search_inside.find(':');
            let end_index = search_inside.find(';');

            // dbg!(&search_inside);
            // dbg!(&start_index.unwrap() + s, &end_index.unwrap() + s);

            match (start_index, end_index) {
                (Some(_), None) | (None, Some(_)) => return Err(Error::InvalidWord),
                (Some(s), Some(e)) => {
                    let s = s + v_i[i];
                    let e = e + v_i[i];

                    let var = &input[s + 2..e - 1];
                    let v: Vec<&str> = var.split(' ').collect();

                    if self.hm.contains_key(&v[0].to_string().to_ascii_uppercase()) {
                        // reassigning the same name
                        // since recursion is used we need to expand it right now

                        let mut expans_vec = v[1..].into_iter().map(|&x| x).collect::<Vec<&str>>();

                        let pos = expans_vec.iter().position(|&e| {
                            e.to_ascii_uppercase() == v[0].to_string().to_ascii_uppercase()
                        });

                        // std::mem::replace(&expans_vec[pos.unwrap()], *self.hm.get(&v[0].to_string().to_ascii_uppercase()))
                        let prev = self
                            .hm
                            .get(v[0].to_string().to_ascii_uppercase().as_str())
                            .unwrap();
                        if let Some(i) = pos {
                            std::mem::replace(&mut expans_vec[i], prev);
                        }

                        // dbg!(&v[0].to_string().to_ascii_uppercase());

                        // dbg!(&self.hm);

                        self.hm.insert(
                            v[0].to_string().to_ascii_uppercase(),
                            // var_expans.to_ascii_uppercase(),
                            expans_vec.join(" ").to_ascii_uppercase(),
                        );

                        // dbg!(&self.hm);

                        let mut l_ops: Vec<&str> = input[..s].split(' ').collect();
                        let r_ops: Vec<&str> = input[e + 1..].split(' ').collect();

                        dbg!(&l_ops, &r_ops);
                        l_ops.extend(r_ops);
                        ops = l_ops.into_iter().filter(|&s| !s.is_empty()).collect();
                    } else {
                        // here need to expand if collision
                        let var_expans = v[1..].join(" ");

                        // should be in previous block as well
                        if v[0].chars().next().unwrap().is_numeric() {
                            return Err(Error::InvalidWord);
                        }

                        // dbg!(&var_expans);
                        if self.hm.contains_key(&var_expans) {
                            let ns = self.hm.get(&var_expans).unwrap();
                            self.hm.insert(
                                v[0].to_string().to_ascii_uppercase(),
                                // var_expans.to_ascii_uppercase(),
                                // *self.hm.get(&var_expans).unwrap().to_string(),
                                (*ns.clone()).to_string(),
                            );
                        } else {
                            dbg!(&self.hm);
                            if self.hm.contains_key(&var_expans.to_ascii_uppercase()) {
                                let ns = self.hm.get(&var_expans.to_ascii_uppercase()).unwrap();
                                self.hm.insert(
                                    v[0].to_string().to_ascii_uppercase(),
                                    ns.to_ascii_uppercase(),
                                );
                            } else {
                                self.hm.insert(
                                    v[0].to_string().to_ascii_uppercase(),
                                    var_expans.to_ascii_uppercase(),
                                );
                            }
                            dbg!(&self.hm);
                        }

                        let mut l_ops: Vec<&str> = input[..s].split(' ').collect();
                        let r_ops: Vec<&str> = input[e + 1..].split(' ').collect();
                        dbg!(&l_ops, &r_ops);

                        l_ops.extend(r_ops);
                        ops = l_ops.into_iter().filter(|&s| !s.is_empty()).collect();
                    }
                }
                _ => ops = input.split(' ').collect::<Vec<&str>>(),
            }
            // }
        }

        if !ops.is_empty() {
            let pos = &ops.iter().position(|&e| e == ";");
            dbg!(&pos);
            if let &Some(x) = pos {
                for _ in 0..=x {
                    ops.remove(0);
                    dbg!(&ops);
                }
            }
            dbg!(&ops);
            self.basic_commands(&ops);
        }

        if !self.errors.is_empty() {
            let err = self
                .errors
                .pop()
                .expect("we already checked that the vec is not empty");
            return Err(err);
        }

        self.integers.iter().for_each(|&v| self.stack.push(v));

        Ok(())
    }

    fn basic_commands(
        &mut self,
        operations: &[&str],
        // operations: &Vec<&str>,
    ) {
        dbg!(&self.hm, &operations);

        for (_, &s) in operations.iter().enumerate() {
            dbg!(&s);

            match s {
                _ if self.hm.contains_key(&s.to_ascii_uppercase()) => {
                    let string = &**self.hm.get(&s.to_ascii_uppercase()).unwrap();
                    let c = string.to_ascii_uppercase();
                    let v: Vec<&str> = c.split(' ').collect();
                    self.basic_commands(&v);
                }
                "+" => {
                    let r = self.integers.pop();
                    let l = self.integers.pop();
                    if let (Some(l), Some(r)) = (l, r) {
                        self.integers.push(l + r);
                    } else {
                        self.errors.push(Error::StackUnderflow);
                        break;
                    }
                }
                "-" => {
                    let r = self.integers.pop();
                    let l = self.integers.pop();
                    if let (Some(l), Some(r)) = (l, r) {
                        self.integers.push(l - r);
                    } else {
                        self.errors.push(Error::StackUnderflow);
                        break;
                    }
                }
                "*" => {
                    let r = self.integers.pop();
                    let l = self.integers.pop();
                    if let (Some(l), Some(r)) = (l, r) {
                        self.integers.push(l * r);
                    } else {
                        self.errors.push(Error::StackUnderflow);
                        break;
                    }
                }
                "/" => {
                    let r = self.integers.pop();
                    if r == Some(0) {
                        self.errors.push(Error::DivisionByZero);
                        break;
                    }
                    let l = self.integers.pop();
                    if let (Some(l), Some(r)) = (l, r) {
                        self.integers.push(l / r);
                    } else {
                        self.errors.push(Error::StackUnderflow);
                        break;
                    }
                }
                stack_manipulation_or_number => {
                    match stack_manipulation_or_number.to_ascii_uppercase().as_str() {
                        "DUP" => {
                            if let Some(x) = self.integers.last() {
                                self.integers.push(*x);
                            } else {
                                self.errors.push(Error::StackUnderflow);
                                break;
                            }
                        }
                        "OVER" => {
                            let len = self.integers.len();
                            if len > 1 {
                                self.integers.push(self.integers[len - 2]);
                            } else {
                                self.errors.push(Error::StackUnderflow);
                                break;
                            }
                        }
                        "DROP" => {
                            if self.integers.pop().is_some() {
                            } else {
                                self.errors.push(Error::StackUnderflow);
                                break;
                            }
                        }
                        "SWAP" => {
                            let len = self.integers.len();
                            if len > 1 {
                                self.integers.swap(len - 2, len - 1);
                            } else {
                                self.errors.push(Error::StackUnderflow);
                                break;
                            }
                        }
                        _ => {
                            if s.len() > 1 && !s.chars().next().unwrap_or(' ').is_numeric() {
                                self.errors.push(Error::UnknownWord);
                                break;
                            }
                            self.integers
                                .push(s.parse::<i32>().expect("failed to parse"));
                        }
                    }
                }
            }
        }
    }
}
