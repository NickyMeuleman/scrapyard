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
        // each output digit is a BTreeSet
        // iterate over each one and count the length of that set
        // len 7 -> display 8
        // len 3 -> display 7
        // len 2 -> display 1
        // len 4 -> display 4
        // count the amount of occurrences of these numbers

        self.pairs
            .iter()
            .flat_map(|pair| pair.digits.iter().map(|digit| digit.len()))
            .filter(|num| [7, 3, 2, 4].contains(num))
            .count()
    }

    fn part_two(&self) -> usize {
        // # Steps for each individual line, for each pair in self.pairs:
        // every digit is represented in the pair.combinations list.
        // each digit is a BTreeSet
        // The goal is to figure out which set corresponds to which displayed digit

        // the length of the set gives a hint to which displayed digit the set represents
        // len 7 -> display 8
        // len 3 -> display 7
        // len 2 -> display 1
        // len 4 -> display 4

        // Then, in order, that means:
        // len 6 and superset of 4-set -> display 9
        // len 5 and superset of 1-set -> display 3
        // len 6 and superset of 1-set -> display 0
        // len 6 -> display 6
        // len 5 and is subset of 6-set -> display 5
        // len 5 -> 2

        // at this point you can translate from set <-> number

        // loop over the 4 given numbers in pair.digits
        // each number is a BTreeSet
        // for each set, translate it to a number
        // concatenate all these numbers (1 + 2 + 3 + 4 = 1234) to get a result for a single pair

        // add each result for a pair to a total result
        // that total is the solution to part2

        let mut total = 0;

        for pair in &self.pairs {
            let mut todo = pair.combinations.clone();
            let mut map: HashMap<u8, &Combination> = HashMap::new();

            // len 2 -> display 1
            let one = todo.iter().find(|set| set.len() == 2).unwrap().clone();
            map.insert(1, &one);
            todo.remove(&one);

            // len 3 -> display 7
            let seven = todo.iter().find(|set| set.len() == 3).unwrap().clone();
            map.insert(7, &seven);
            todo.remove(&seven);

            // len 4 -> display 4
            let four = todo.iter().find(|set| set.len() == 4).unwrap().clone();
            map.insert(4, &four);
            todo.remove(&four);

            // len 7 -> display 8
            let eight = todo.iter().find(|set| set.len() == 7).unwrap().clone();
            map.insert(8, &eight);
            todo.remove(&eight);

            // len 6 and superset of 4-set -> display 9
            let nine = todo
                .iter()
                .find(|set| set.len() == 6 && set.is_superset(map.get(&4).unwrap()))
                .unwrap()
                .clone();
            map.insert(9, &nine);
            todo.remove(&nine);

            // len 5 and superset of 1-set -> display 3
            let three = todo
                .iter()
                .find(|set| set.len() == 5 && set.is_superset(map.get(&1).unwrap()))
                .unwrap()
                .clone();
            map.insert(3, &three);
            todo.remove(&three);

            // len 6 and superset of 1-set -> display 0
            let zero = todo
                .iter()
                .find(|set| set.len() == 6 && set.is_superset(map.get(&1).unwrap()))
                .unwrap()
                .clone();
            map.insert(0, &zero);
            todo.remove(&zero);

            // len 6 -> display 6
            let six = todo.iter().find(|set| set.len() == 6).unwrap().clone();
            map.insert(6, &six);
            todo.remove(&six);

            // len 5 and is subset of 6-set -> display 5
            let five = todo
                .iter()
                .find(|set| set.len() == 5 && set.is_subset(map.get(&6).unwrap()))
                .unwrap()
                .clone();
            map.insert(5, &five);
            todo.remove(&five);

            // len 5 -> 2
            let two = todo.iter().find(|set| set.len() == 5).unwrap().clone();
            map.insert(2, &two);
            todo.remove(&two);

            assert_eq!(todo.len(), 0);
            assert_eq!(map.len(), 10);

            let num: usize = pair
                .digits
                .iter()
                .map(|set| {
                    map.iter()
                        .find(|(_, &map_set)| set == map_set)
                        .unwrap()
                        .0
                        .to_string()
                })
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
