use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let passports = parse(input);
    println!("part one answer: {}", part_one(passports.clone()));
    println!("part two answer: {}", part_two(passports));
}

fn part_one(passports: Vec<HashMap<&str, &str>>) -> u32 {
    count_valid(passports, false)
}

fn part_two(passports: Vec<HashMap<&str, &str>>) -> u32 {
    count_valid(passports, true)
}

fn parse(input: String) -> Vec<HashMap<&'static str, &'static str>> {
    // forgive me, I'll clean up the names later
    let passport_str_blocks: Vec<&str> = input.split("\n\n").collect();

    let mut passport_blocks_vec_of_str: Vec<Vec<&str>> = Vec::new();
    for block in passport_str_blocks {
        let line: Vec<&str> = block.split("\n").collect();
        passport_blocks_vec_of_str.push(line);
    }

    let mut passport_blocks: Vec<Vec<&str>> = Vec::new();
    for block in passport_blocks_vec_of_str {
        let mut passport_line: Vec<&str> = Vec::new();
        for line in block {
            let mut parts: Vec<&str> = line.split(" ").collect();
            passport_line.append(&mut parts);
        }
        passport_blocks.push(passport_line);
    }

    let mut passports_final: Vec<HashMap<&str, &str>> = Vec::new();
    for passport in passport_blocks {
        let mut passport_map: HashMap<&str, &str> = HashMap::new();
        for line in passport {
            let items: Vec<&str> = line.split(":").collect();
            passport_map.entry(items[0]).or_insert(items[1]);
        }
        passports_final.push(passport_map);
    }

    // ERROR: returns a value referencing data owned by the current function
    passports_final.clone()
}

fn count_valid(passports: Vec<HashMap<&str, &str>>, validate: bool) -> u32 {
    let mut count = 0;
    for mut passport in passports {
        if passport.len() == 8 {
            if validate {
                if validate_id(&passport) {
                    count += 1;
                }
            } else {
                count += 1;
            }
        } else if passport.len() == 7 {
            let entry = passport.entry("cid");
            if let Entry::Vacant(_) = entry {
                if validate {
                    if validate_id(&passport) {
                        count += 1;
                    }
                } else {
                    count += 1;
                }
            }
        }
    }
    count
}

fn validate_num(num: &str, min: u16, max: u16) -> bool {
    let num: u16 = num.parse().unwrap();
    min <= num && num <= max
}

fn validate_hgt(hgt: &str) -> bool {
    let (value, unit) = hgt.split_at(hgt.len() - 2);
    match unit {
        "cm" => validate_num(value, 150, 193),
        "in" => validate_num(value, 59, 76),
        _ => false,
    }
}

fn validate_hcl(hcl: &str) -> bool {
    match hcl.split_at(1) {
        ("#", chars) => validate_hex(chars),
        _ => false,
    }
}

fn validate_hex(chars: &str) -> bool {
    chars.chars().all(|c| c.is_ascii_hexdigit())
}

fn validate_ecl(ecl: &str) -> bool {
    let valid_colors: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid_colors.contains(&ecl)
}

fn validate_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.chars().all(|c| c.is_numeric())
}

fn validate_id(id: &HashMap<&str, &str>) -> bool {
    id.iter().all(|(&key, &val)| match key {
        "byr" => validate_num(val, 1920, 2002),
        "iyr" => validate_num(val, 2010, 2020),
        "eyr" => validate_num(val, 2020, 2030),
        "hgt" => validate_hgt(val),
        "hcl" => validate_hcl(val),
        "ecl" => validate_ecl(val),
        "pid" => validate_pid(val),
        "cid" => true,
        _ => false,
    })
}