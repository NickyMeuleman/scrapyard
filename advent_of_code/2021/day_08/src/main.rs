use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::str::FromStr;

type Combination = BTreeSet<char>;

#[derive(Debug)]
struct Pair {
    combinations: HashSet<Combination>,
    digits: Vec<Combination>,
}

#[derive(Debug)]
struct Data {
    pairs: Vec<Pair>,
}

impl Data {
    fn part_one(&self) -> usize {
        let mut result = 0;
        // create HashMap with <Combination, possible digits>
        for pair in &self.pairs {
            let mut possibilities_map: HashMap<Combination, HashSet<u8>> = HashMap::new();
            let combinations = &pair.combinations;
            for combination in combinations {
                // each combination starts off with all 10 possibilities: 0,1,2,3,4,5,6,7,8,9
                let all_options: HashSet<u8> = (0..=9).collect();
                possibilities_map.insert(combination.clone(), all_options);
            }
            // eliminate possibilities
            // first round of eliminations: check length of Combination, some digits are eliminated (1 only has 2 letters etc)
            for mut entry in &mut possibilities_map {
                let len = entry.0.len();
                match len {
                    2 => {
                        // has to be a 1
                        entry.1.retain(|&k| k == 1);
                    }
                    3 => {
                        // has to be a 7
                        entry.1.retain(|&k| k == 7);
                    }
                    4 => {
                        // has to be a 4
                        entry.1.retain(|&k| k == 4);
                    }
                    7 => {
                        // has to be a 8
                        entry.1.retain(|&k| k == 8);
                    }
                    _ => {}
                }
            }

            let one_four_seven_eight: HashSet<Combination> = possibilities_map
                .into_iter()
                .filter(|(_, possibilities)| possibilities.len() == 1)
                .map(|(combinations, _)| combinations)
                .collect();

            let digits = &pair.digits;

            for digit in digits {
                if one_four_seven_eight.contains(digit) {
                    result += 1;
                }
            }
        }

        result
    }

    fn part_two(&self) -> usize {
        let mut total = 0;

        for pair in &self.pairs {
            let mut todo = pair.combinations.clone();
            let mut map: HashMap<u8, &Combination> = HashMap::new();

            let one = todo.iter().find(|set| set.len() == 2).unwrap().clone();
            map.insert(1, &one);
            todo.remove(&one);

            let seven = todo.iter().find(|set| set.len() == 3).unwrap().clone();
            map.insert(7, &seven);
            todo.remove(&seven);

            let four = todo.iter().find(|set| set.len() == 4).unwrap().clone();
            map.insert(4, &four);
            todo.remove(&four);

            let eight = todo.iter().find(|set| set.len() == 7).unwrap().clone();
            map.insert(8, &eight);
            todo.remove(&eight);

            let nine = {
                let len_six = todo
                    .iter()
                    .filter(|set| set.len() == 6)
                    .collect::<Vec<&Combination>>()
                    .clone();
                let four = map.get(&4).unwrap();
                len_six
                    .into_iter()
                    .find(|set| set.is_superset(four))
                    .unwrap()
                    .clone()
            };
            map.insert(9, &nine);
            todo.remove(&nine);

            let three = {
                let len_five = todo
                    .iter()
                    .filter(|set| set.len() == 5)
                    .collect::<Vec<&Combination>>()
                    .clone();
                let one = map.get(&1).unwrap();
                len_five
                    .into_iter()
                    .find(|set| set.is_superset(one))
                    .unwrap()
                    .clone()
            };
            map.insert(3, &three);
            todo.remove(&three);

            let zero = {
                let len_six = todo
                    .iter()
                    .filter(|set| set.len() == 6)
                    .collect::<Vec<&Combination>>()
                    .clone();
                let one = map.get(&1).unwrap();
                len_six
                    .into_iter()
                    .find(|set| set.is_superset(one))
                    .unwrap()
                    .clone()
            };
            map.insert(0, &zero);
            todo.remove(&zero);

            let six = todo.iter().find(|set| set.len() == 6).unwrap().clone();
            map.insert(6, &six);
            todo.remove(&six);

            let five = {
                let len_five = todo
                    .iter()
                    .filter(|set| set.len() == 5)
                    .collect::<Vec<&Combination>>()
                    .clone();
                let six = map.get(&6).unwrap();
                len_five
                    .into_iter()
                    .find(|set| set.is_subset(six))
                    .unwrap()
                    .clone()
            };
            map.insert(5, &five);
            todo.remove(&five);

            let two = todo.iter().find(|set| set.len() == 5).unwrap().clone();
            map.insert(2, &two);
            todo.remove(&two);

            assert_eq!(todo.len(), 0);
            assert_eq!(map.len(), 10);

            let num: usize = pair
                .digits
                .iter()
                .map(|set| *map.iter().find(|(_, &map_set)| set == map_set).unwrap().0)
                .map(|n| n.to_string())
                .collect::<String>()
                .parse()
                .unwrap();

            total += num;
        }

        total
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let pairs = input.lines().map(|line| parse_line(line)).collect();
        Ok(Self { pairs })
    }
}

fn parse_line(input: &str) -> Pair {
    let (left, right) = input.split_once("|").unwrap();
    let combinations: HashSet<Combination> = left
        .trim()
        .split_whitespace()
        .map(|s| parse_combination(s))
        .collect();

    let digits: Vec<Combination> = right
        .trim()
        .split_whitespace()
        .map(|s| parse_combination(s))
        .collect();

    Pair {
        combinations,
        digits,
    }
}

fn parse_combination(input: &str) -> Combination {
    let mut combination = BTreeSet::new();

    for c in input.chars() {
        combination.insert(c);
    }

    combination
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = input.parse().unwrap();
    println!("Part one answer: {}", data.part_one());
    println!("Part two answer: {}", data.part_two());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part_one_example() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let data: Data = input.parse().unwrap();
        dbg!(&data);
        assert_eq!(data.part_one(), 26);
    }

    #[test]

    fn part_two_example() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let data: Data = input.parse().unwrap();
        assert_eq!(data.part_two(), 61229);
    }
}
