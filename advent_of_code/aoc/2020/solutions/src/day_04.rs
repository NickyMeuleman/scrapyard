use std::{collections::HashMap, fmt::Display};

use crate::{AoCData, AoCResult};

#[derive(Debug, Clone)]
pub struct Data<'a>(Vec<HashMap<&'a str, &'a str>>);

fn count_valid(passports: &Vec<HashMap<&str, &str>>, validate: bool) -> u32 {
    let mut count = 0;
    for passport in passports {
        if passport.len() == 8 {
            if validate {
                if validate_id(&passport) {
                    count += 1;
                }
            } else {
                count += 1;
            }
        } else if passport.len() == 7 {
            if let None = passport.get("cid") {
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
    chars
        .chars()
        .all(|c| c.is_ascii_hexdigit())
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

impl<'a> AoCData<'a> for Data<'a> {
    fn try_new(input: &'a str) -> AoCResult<Self> {
        // forgive me, I'll clean up the names later, maybe
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
                passport_map
                    .entry(items[0])
                    .or_insert(items[1]);
            }
            passports_final.push(passport_map);
        }

        Ok(Self(passports_final))
    }

    fn part_1(&self) -> AoCResult<impl Display> {
        Ok(count_valid(&self.0, false))
    }

    fn part_2(&self) -> AoCResult<impl Display> {
        Ok(count_valid(&self.0, true))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_1() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let data = Data::try_new(input).unwrap();
        let result = data.part_1().unwrap().to_string();
        assert_eq!(result, "2");
    }

    #[test]
    fn part_2() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        let data = Data::try_new(input).unwrap();
        let result = data.part_2().unwrap().to_string();
        assert_eq!(result, "2");
    }
}
