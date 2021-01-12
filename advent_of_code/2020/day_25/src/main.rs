use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let data = parse(&input);
    println!("Part one answer: {}", part_one(data));
}

fn parse(input: &str) -> (usize, usize) {
    let mut iter = input.lines();
    let card_public_key = iter.next().unwrap();
    let door_public_key = iter.next().unwrap();
    (
        card_public_key.parse().unwrap(),
        door_public_key.parse().unwrap(),
    )
}

fn part_one(data: (usize, usize)) -> usize {
    let card_loop_size = get_loop_size(data.0);
    let door_loop_size = get_loop_size(data.1);
    let card_encryption_key = get_encryption_key(data.1, card_loop_size);
    let door_encryption_key = get_encryption_key(data.0, door_loop_size);
    assert_eq!(card_encryption_key, door_encryption_key);
    card_encryption_key
}

fn transform_cycle(mut val: usize, subject_number: usize) -> usize {
    val = val * subject_number;
    val % 20201227
}

fn get_loop_size(target: usize) -> usize {
    let mut val = 1;
    let mut loop_size = 0;
    while val != target {
        val = transform_cycle(val, 7);
        loop_size += 1;
    }
    loop_size
}

fn get_encryption_key(public_key: usize, loop_size: usize) -> usize {
    let mut val = 1;
    for _ in 0..loop_size {
        val = transform_cycle(val, public_key)
    }
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_one() {
        let input = "5764801
17807724"
            .to_owned();
        let data = parse(&input);
        assert_eq!(part_one(data), 14897079);
    }
}
