use once_cell::sync::Lazy;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashSet;
use std::sync::Mutex;

static DATABASE: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

#[derive(Default)]
pub struct Robot(String);

impl Robot {
    #[must_use]
    pub fn new() -> Self {
        let mut database = DATABASE.lock().unwrap();
        let mut name = Self::generate_name();
        while database.contains(&name) {
            name = Self::generate_name();
        }
        database.insert(name.clone());
        Self(name)
    }

    #[must_use]
    fn generate_name() -> String {
        let mut rng = rand::thread_rng();
        let letters = Uniform::from('A'..='Z');
        let digits = Uniform::from('0'..='9');
        let mut name = String::new();
        for _ in 0..2 {
            name.push(letters.sample(&mut rng));
        }
        for _ in 0..3 {
            name.push(digits.sample(&mut rng));
        }
        name
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.0.as_str()
    }

    pub fn reset_name(&mut self) {
        let mut database = DATABASE.lock().unwrap();
        let mut name = Self::generate_name();
        while database.contains(&name) {
            name = Self::generate_name();
        }
        database.insert(name.clone());
        database.remove(&self.0);
        self.0 = name;
    }
}
