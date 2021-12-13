use day_13::Data;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input_snoozeysleepy.txt").unwrap();
    // let input = fs::read_to_string("./input_mohammedsouleymane.txt").unwrap();
    // let input = fs::read_to_string("./input_chrisbiscardi.txt").unwrap();
    // let input = fs::read_to_string("./input_philds.txt").unwrap();
    // let input = fs::read_to_string("./input_jesperdramsch.txt").unwrap();
    // let input = fs::read_to_string("./input_marcelblijleven.txt").unwrap();
    // let input = fs::read_to_string("./input_tbpaolini.txt").unwrap();
    // let input = fs::read_to_string("./input_snhmibby.txt").unwrap();
    // let input = fs::read_to_string("./input_tginsberg.txt").unwrap();
    // let input = fs::read_to_string("./input_letfish.txt").unwrap();
    // let input = fs::read_to_string("./input_linagkar.txt").unwrap();
    // let input = fs::read_to_string("./input_jeffomatic.txt").unwrap();
    // let input = fs::read_to_string("./input_misfits42.txt").unwrap();
    // let input = fs::read_to_string("./input_fadi88.txt").unwrap();
    // let input = fs::read_to_string("./input_gobler.txt").unwrap();
    // let input = fs::read_to_string("./input.txt").unwrap();
    let data: Data = input.parse().unwrap();
    println!("Part one answer: {}", data.clone().part_one());
    println!("Part two answer: {}", data.part_two());
}
