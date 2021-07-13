// minor edits to psychesoma's solution

use std::collections::HashSet;

pub struct Allergies(HashSet<Allergen>);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(mut score: u32) -> Self {
        let allergens = vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];
        // numerical values of allergens can be thought of as powers of 2
        // Eggs is 2^0 = 1
        // Peanuts is 2^1 = 2
        // Shellfish is 2^2 = 4
        let allergies = allergens.iter().fold(HashSet::new(), |mut acc, &allergen| {
            if score % 2 != 0 {
                acc.insert(allergen);
            }
            // reduce the power of 2 by 1
            score /= 2;
            acc
        });
        Allergies(allergies)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.0.iter().copied().collect()
    }
}
