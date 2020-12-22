use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();

    let input = std::fs::read_to_string(&args[1])?;
    let foods = FoodSet::parse_list(&input);
    println!("Found {}", foods.count_of_non_allergens());
    println!("Dangerous list is {}", foods.dangerous_list());

    Ok(())
}

#[derive(Clone, Debug)]
pub struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

impl<'a> Food<'a> {
    pub fn new(input: &'a str) -> Self {
        let (left, right) = split_once(&input[0..input.len()-1], "(contains ");
        Self {
            ingredients: left.split(' ').filter(|s| !s.is_empty()).collect(),
            allergens: right.split(", ").collect(),
        }
    }
}

pub struct FoodSet<'a> {
    foods: Vec<Food<'a>>,
}

impl<'a> FoodSet<'a> {
    pub fn parse_list(input: &'a str) -> Self {
        let foods: Vec<Food> = input.lines().map(|line| Food::new(line)).collect();
        println!("Parsed foods:");
        foods.iter().for_each(|f| println!("  {:?}", f));
        FoodSet{
            foods: foods,
        }
    }

    pub fn map_allergens(&self) -> HashMap<&'a str, &'a str> {

        let mut all_allergens: HashSet<_> = self.foods.iter().flat_map(|f| f.allergens.iter()).map(|a| *a).collect();
        let mut foods = self.foods.clone();
        let mut known_allergens = HashMap::new();
        while !all_allergens.is_empty() {
            match match_one(&foods, &all_allergens) {
                Some((allergen, ingredient)) => {
                    println!("mapped {} to {}", allergen, ingredient);
                    all_allergens.remove(&allergen);
                    foods = clone_without_ingredient_allergen(&foods, &ingredient, &allergen);
                    known_allergens.insert(ingredient, allergen);
                },
                None => panic!("Unable to match an allergen!"),
            }
        }

        known_allergens
    }

    pub fn count_of_non_allergens(&self) -> usize {
        let known_allergens = self.map_allergens();
        let mut res = 0;
        let all_ingredients: HashSet<_> = self.foods.iter().flat_map(|f| f.ingredients.iter()).collect();
        for i in all_ingredients {
            if !known_allergens.contains_key(i) {
                for f in &self.foods {
                    if f.ingredients.contains(i) {
                        res += 1;
                    }
                }
            }
        }

        res
    }

    pub fn dangerous_list(&self) -> String {
        let known_allergens = self.map_allergens();
        let rev_map:HashMap<_, _>= known_allergens.iter().map(|(k,v)| (v,k)).collect();
        let all_allergens : HashSet<_> = self.foods.iter().flat_map(|f| f.allergens.iter()).map(|s| *s).collect();
        let mut sorted_allergens: Vec<_> = all_allergens.iter().map(|s| *s).collect();
        sorted_allergens.sort();
        let res = sorted_allergens.iter().fold("".to_string(), |acc, all| format!("{},{}", acc, rev_map[all]));
        String::from(&res[1..])
    }
}

fn match_one<'a>(foods: &Vec<Food<'a>>, all_allergens: &HashSet<&'a str>) -> Option<(&'a str, &'a str)> {
    for a in all_allergens {
        let foods_with_a: Vec<_> = foods.iter().filter(|f| f.allergens.contains(*a)).collect();
        println!("Examining {}, amongst {:?}", a, foods_with_a);
        let mut possible_ingredients: HashSet<_> = foods_with_a[0].ingredients.clone();
        for i in 1..foods_with_a.len() {
            possible_ingredients = possible_ingredients.intersection(&foods_with_a[i].ingredients).map(|f| *f).collect();
        }

        if possible_ingredients.len() == 1 {
            return Some((*a, possible_ingredients.iter().nth(0).unwrap()));
        }
    }

    None
}

fn clone_without_ingredient_allergen<'a>(foods: &Vec<Food<'a>>, ingredient: &str, allergen: &str) -> Vec<Food<'a>> {
    let mut res = Vec::new();
    for f in foods {
        let mut ingredients = f.ingredients.clone();
        let mut allergens = f.allergens.clone();
        ingredients.remove(ingredient);
        allergens.remove(allergen);

        res.push(Food {
            ingredients: ingredients,
            allergens: allergens,
        });
    }
    res
}

fn split_once<'a>(in_string: &'a str, split_on: &str) -> (&'a str, &'a str) {
    let mut splitter = in_string.splitn(2, split_on);
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test() {
        let food_set = FoodSet::parse_list("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)");
        assert_eq!(5, food_set.count_of_non_allergens());
        assert_eq!("mxmxvkd,sqjhc,fvjkl", food_set.dangerous_list());
    }
}