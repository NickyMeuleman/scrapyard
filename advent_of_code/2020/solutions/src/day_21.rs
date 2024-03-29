use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data(Vec<(Vec<String>, Vec<String>)>);

fn get_all(data: &Vec<(Vec<String>, Vec<String>)>) -> (HashSet<String>, HashSet<String>) {
    data.iter().fold(
        (HashSet::new(), HashSet::new()),
        |mut acc, (ingredients, allergens)| {
            for ingredient in ingredients {
                acc.0.insert(ingredient.clone());
            }
            for allergen in allergens {
                acc.1.insert(allergen.clone());
            }
            acc
        },
    )
}

fn get_allergen_to_ingredient_possibilities(
    data: &Vec<(Vec<String>, Vec<String>)>,
) -> HashMap<String, HashSet<String>> {
    let (all_ingredients, all_allergens) = get_all(data);
    let mut possibilities_per_allergen: HashMap<String, HashSet<String>> = HashMap::new();
    // start by populating all allergens with every ingredient
    for allergen in &all_allergens {
        possibilities_per_allergen.insert(allergen.clone(), all_ingredients.clone());
    }
    // only keep ingedients that have that allergen listed in a dish
    for (ingredients, allergens) in data {
        let ingredients: HashSet<String> = ingredients
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let allergens: HashSet<String> = allergens
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        for allergen in allergens {
            let in_both = possibilities_per_allergen
                .get(&allergen)
                .unwrap()
                .intersection(&ingredients)
                .map(|s| s.to_owned())
                .collect();
            possibilities_per_allergen.insert(allergen, in_both);
        }
    }
    possibilities_per_allergen
}

fn allergen_to_ingredient(
    mut allergen_to_ingredients: HashMap<String, HashSet<String>>,
) -> HashMap<String, String> {
    let mut allergen_map: HashMap<String, String> = HashMap::new();
    // when an allergen with a single ingredient is found, add it to a allergen_map and remove the ingredient from all other allergen options
    while let Some(item) = allergen_to_ingredients
        .iter()
        .find(|(_, ingredients)| ingredients.len() == 1)
    {
        let (allergen, ingredients) = item;
        // add to allergen_map
        let ingredient = ingredients
            .clone()
            .drain()
            .next()
            .unwrap();
        allergen_map.insert(allergen.clone(), ingredient.clone());
        // remove from every set in allergen_to_ingredient
        for (_, set) in &mut allergen_to_ingredients {
            set.remove(&ingredient);
        }
    }
    allergen_map
}

impl AoCData<'_> for Data {
    fn try_new(input: &str) -> AoCResult<Self> {
        let mut result = Vec::new();
        for line in input.lines() {
            let parts: Vec<&str> = line.splitn(2, '(').collect();
            let ingredients: Vec<String> = parts[0]
                .trim()
                .split(" ")
                .map(|s| s.to_owned())
                .collect();
            let allergens = parts[1].replace("contains ", "");
            let allergens = allergens.replace(")", "");
            let allergens: Vec<String> = allergens
                .trim()
                .split(", ")
                .map(|s| s.to_owned())
                .collect();
            result.push((ingredients, allergens));
        }
        Ok(Self(result))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        let allergen_to_ingredients = get_allergen_to_ingredient_possibilities(&self.0);
        let possibly_allergenic_ingredients: HashSet<&String> = allergen_to_ingredients
            .values()
            .flatten()
            .collect();

        let result = self
            .0
            .iter()
            .fold(0usize, |mut acc, (ingredients_in_dish, _)| {
                ingredients_in_dish
                    .iter()
                    .for_each(|ingredient| {
                        if !possibly_allergenic_ingredients.contains(ingredient) {
                            acc += 1;
                        }
                    });
                acc
            });

        Ok(result)
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        let allergen_to_ingredients = get_allergen_to_ingredient_possibilities(&self.0);
        let allergen_to_ingredient = allergen_to_ingredient(allergen_to_ingredients);
        let mut allergen_to_ingredient: Vec<(String, String)> = allergen_to_ingredient
            .into_iter()
            .collect();

        allergen_to_ingredient
            .sort_by(|(allergen_a, _), (allergen_b, _)| allergen_a.cmp(&allergen_b));
        let result = allergen_to_ingredient
            .iter()
            .map(|(_, ingredient)| ingredient.clone())
            .collect::<Vec<String>>()
            .join(",");

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "5");
    }

    #[test]
    fn part_2() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "mxmxvkd,sqjhc,fvjkl");
    }
}
