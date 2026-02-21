pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
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
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        match allergen {
            Allergen::Eggs => (self.score & 1) != 0,
            Allergen::Peanuts => (self.score & 2) != 0,
            Allergen::Shellfish => (self.score & 4) != 0,
            Allergen::Strawberries => (self.score & 8) != 0,
            Allergen::Tomatoes => (self.score & 16) != 0,
            Allergen::Chocolate => (self.score & 32) != 0,
            Allergen::Pollen => (self.score & 64) != 0,
            Allergen::Cats => (self.score & 128) != 0,
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let all_variants = [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        all_variants
            .into_iter()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
}
