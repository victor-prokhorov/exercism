use std::ops::Rem;

pub struct Matcher<T>
where
    T: ToString,
{
    matcher: Box<dyn Fn(T) -> bool>, // closure that check if current element match criteria
    subs: String, // if yes substitute with this
}

impl<T: ToString> Matcher<T> {
    pub fn new<F>(matcher: F, subs: &str) -> Matcher<T>
    where
        // note: we are defining closure that impl trait, we don't need `dyn` here
        F: Fn(T) -> bool + 'static,
    {
        Self {
            matcher: Box::new(matcher),
            subs: subs.to_owned(),
        }
    }
}

pub struct Fizzy<T: ToString> {
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString + Clone + 'static> Fizzy<T> {
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }
    pub fn apply(self, iter: impl Iterator<Item = T>) -> impl Iterator<Item = String> {
        iter.map(move |x| {
            // let mut s = String::new(); // FizzBuzz, in one go, apply all matchers
            // // matcher.matcher... rename this
            // for Matcher {matcher,  subs} in &self.matchers {
            //     if matcher(x.clone()) { s.push_str(subs) }
            // }
            // memo: diff between `then` and `and_then` https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=404a5fea4d01eb6e04c1849a831fd6d0
            let s:String = self.matchers.iter().filter_map(|Matcher {matcher, subs}| matcher(x.clone()).then(|| subs.clone())).collect();
            s.is_empty().then(|| x.to_string()).unwrap_or_else(|| s)
            // todo: check if can combine into one liner
            // self.matchers.iter()
            //     .filter_map(|Matcher {matcher, subs}| matcher(x.clone()).then_some(subs.clone())).map(|x| {dbg!(x.to_string());x})
            //     .fold(x.to_string(), |mut acc, s| {
            //         acc.push_str(s.as_str());
            //         acc
            //     })
            // if s.is_empty() {
            //     x.to_string()
            // }else{
            //     s
            // }
        })
    }
}

pub fn fizz_buzz<T: ToString + Clone + From<u8> + Rem<Output = T> +  'static>() -> Fizzy<T>
where
    <T as Rem>::Output: PartialEq,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|x: T| x % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|x: T| x % T::from(5) == T::from(0), "buzz"))
}
