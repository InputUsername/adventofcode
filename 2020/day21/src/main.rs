use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug, Clone)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl std::str::FromStr for Food {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim_end_matches(')').split(" (contains ");
        Ok(Self {
            ingredients: parts
                .next()
                .unwrap()
                .split(' ')
                .map(ToOwned::to_owned)
                .collect(),
            allergens: parts
                .next()
                .unwrap()
                .split(", ")
                .map(ToOwned::to_owned)
                .collect(),
        })
    }
}

fn main() {
    let foods: Vec<Food> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

//     let foods: Vec<Food> = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
// trh fvjkl sbzzf mxmxvkd (contains dairy)
// sqjhc fvjkl (contains soy)
// sqjhc mxmxvkd sbzzf (contains fish)".lines().map(|l| l.parse().unwrap()).collect();

    part1(&foods);
    part2(&foods);
}

fn analyze_foods(foods: &[Food]) -> (BTreeMap<&String, HashMap<&String, u32>>, HashSet<&String>) {
    let mut allergen_counts: BTreeMap<&String, HashMap<&String, u32>> = BTreeMap::new();
    let mut anti_candidates = HashSet::new();
    for Food { ingredients, allergens } in foods.iter() {
        for allergen in allergens.iter() {
            let counts= allergen_counts.entry(allergen).or_default();
            for ingredient in ingredients.iter() {
                *counts.entry(ingredient).or_default() += 1;
            }
        }
        anti_candidates.extend(ingredients);
    }

    for (_, counts) in allergen_counts.iter_mut() {
        let (_, max) = counts.iter().max_by_key(|(_, count)| **count).unwrap();
        let mut remove = Vec::new();
        for (ingredient, count) in counts.iter() {
            if count == max {
                anti_candidates.remove(ingredient);
            } else {
                remove.push(*ingredient);
            }
        }
        for ingredient in remove.iter() {
            counts.remove(ingredient);
        }
    }

    (allergen_counts, anti_candidates)
}

fn part1(foods: &[Food]) {
    let (_, anti_candidates) = analyze_foods(&foods);

    let mut total = 0;
    for Food { ingredients, .. } in foods.iter() {
        for ingredient in ingredients.iter() {
            if anti_candidates.contains(ingredient) {
                total += 1;
            }
        }
    }

    println!("{}", total);
}

fn part2(foods: &[Food]) {
    let (mut allergen_counts, anti_candidates) = analyze_foods(&foods);

    let mut all_ingredients: HashSet<&String> = HashSet::new();

    for (_, counts) in allergen_counts.iter_mut() {
        for ac in anti_candidates.iter() {
            counts.remove(ac);
        }

        all_ingredients.extend(counts.keys());
    }

    let mut ingredients: BTreeMap<&String, &String> = BTreeMap::new();

    while ingredients.len() != all_ingredients.len() {
        for (allergen, counts) in allergen_counts.iter() {
            if counts.len() == 1 && !ingredients.contains_key(allergen) {
                ingredients.insert(allergen, counts.keys().next().unwrap());
            }
        }
        for (_, counts) in allergen_counts.iter_mut() {
            for ingredient in ingredients.values() {
                counts.remove(ingredient);
            }
        }
    }

    println!("{}", ingredients.into_values().cloned().collect::<Vec<_>>().join(","));
}
