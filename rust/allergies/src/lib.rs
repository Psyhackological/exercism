#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_as_value = *allergen as u32;
        self.score & allergen_as_value != 0 // Check if the bit is set (not 0)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let all_allergens = [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        all_allergens
            .iter()
            .filter(|&&allergen| self.is_allergic_to(&allergen))
            .cloned() // Clone each allergen to return a Vec<Allergen>
            .collect()
    }
}
