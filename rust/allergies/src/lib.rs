// #![warn(clippy::pedantic)]

// use Allergen::{Cats, Chocolate, Eggs, Peanuts, Pollen, Shellfish, Strawberries, Tomatoes};

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// pub enum Allergen {
//     Eggs = 1 << 0,
//     Peanuts = 1 << 1,
//     Shellfish = 1 << 2,
//     Strawberries = 1 << 3,
//     Tomatoes = 1 << 4,
//     Chocolate = 1 << 5,
//     Pollen = 1 << 6,
//     Cats = 1 << 7,
// }

// const ALLERGENS: [Allergen; 8] = [
//     Eggs,
//     Peanuts,
//     Shellfish,
//     Strawberries,
//     Tomatoes,
//     Chocolate,
//     Pollen,
//     Cats,
// ];

// pub struct Allergies {
//     score: u32,
// }

// impl Allergies {
//     #[must_use]
//     pub fn new(score: u32) -> Self {
//         Self { score }
//     }

//     #[must_use]
//     pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
//         let allergen = *allergen as u32;
//         self.score & allergen == allergen
//     }

//     #[must_use]
//     pub fn allergies(&self) -> Vec<Allergen> {
//         ALLERGENS
//             .iter()
//             .filter(|a| self.is_allergic_to(a))
//             .copied()
//             .collect()
//     }
// }

pub struct Allergies{
    score: u32
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }
    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies_list: Vec<Allergen> = Vec::new();
        for bit_position in 0..u8::BITS {
            let allergen_score = self.score & (1 << bit_position);
            if allergen_score > 0 {
                allergies_list.push(Allergen::new(allergen_score));
            }
        }
        allergies_list
    }
}