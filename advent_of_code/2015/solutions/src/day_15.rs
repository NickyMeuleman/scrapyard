use std::fmt::Display;

use aoc_core::AoCError;
use itertools::Itertools;

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<Ingredient>);

#[derive(Debug, PartialEq, Clone)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

#[derive(Debug)]
struct Properties {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut ingredients = Vec::new();
        for line in input.lines() {
            let mut ingredient = Ingredient {
                name: String::new(),
                capacity: 0,
                durability: 0,
                flavor: 0,
                calories: 0,
                texture: 0,
            };
            let (name, properties) = line
                .split_once(": ")
                .ok_or(AoCError::Parsing)?;
            ingredient.name = name.to_string();
            for property in properties.split(", ") {
                let (name, amount) = property
                    .split_once(' ')
                    .ok_or(AoCError::Parsing)?;
                let amount = amount.parse()?;
                match name {
                    "capacity" => ingredient.capacity = amount,
                    "durability" => ingredient.durability = amount,
                    "flavor" => ingredient.flavor = amount,
                    "calories" => ingredient.calories = amount,
                    "texture" => ingredient.texture = amount,
                    _ => return Err(AoCError::Parsing),
                }
            }
            ingredients.push(ingredient);
        }
        Ok(Self(ingredients))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        self.0
            .iter()
            // all possible recipes. with replacement because we can use as much teaspoons of an ingredient as we want
            .combinations_with_replacement(100)
            // map combination of teaspoons to the final properties of that combination
            .map(|recipe| {
                recipe.iter().dedup_with_count().fold(
                    Properties {
                        capacity: 0,
                        durability: 0,
                        flavor: 0,
                        texture: 0,
                        calories: 0,
                    },
                    |mut acc, (spoon_count, ingr)| {
                        let spoon_count = spoon_count as i32; // it's fiiiiiiiiiiine. source: trust me
                        acc.calories += ingr.calories * spoon_count;
                        acc.capacity += ingr.capacity * spoon_count;
                        acc.durability += ingr.durability * spoon_count;
                        acc.flavor += ingr.flavor * spoon_count;
                        acc.texture += ingr.texture * spoon_count;
                        acc
                    },
                )
            })
            // map those properties to a final score
            .map(|val| {
                val.capacity.max(0) * val.durability.max(0) * val.flavor.max(0) * val.texture.max(0)
            })
            .max()
            .ok_or(AoCError::Solving)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        self.0
            .iter()
            // all possible recipes. with replacement because we can use as much teaspoons of an ingredient as we want
            .combinations_with_replacement(100)
            // map combination of teaspoons to the final properties of that combination
            .map(|recipe| {
                recipe.iter().dedup_with_count().fold(
                    Properties {
                        capacity: 0,
                        durability: 0,
                        flavor: 0,
                        texture: 0,
                        calories: 0,
                    },
                    |mut acc, (spoon_count, ingr)| {
                        let spoon_count = spoon_count as i32; // it's fiiiiiiiiiiine. source: trust me
                        acc.calories += ingr.calories * spoon_count;
                        acc.capacity += ingr.capacity * spoon_count;
                        acc.durability += ingr.durability * spoon_count;
                        acc.flavor += ingr.flavor * spoon_count;
                        acc.texture += ingr.texture * spoon_count;
                        acc
                    },
                )
            })
            // only take cookies with a 500 calorie score
            .filter(|val| val.calories == 500)
            // map those properties to a final score
            .map(|val| {
                val.capacity.max(0) * val.durability.max(0) * val.flavor.max(0) * val.texture.max(0)
            })
            .max()
            .ok_or(AoCError::Solving)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "62842880");
    }

    #[test]
    fn part_2() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "57600000");
    }
}
