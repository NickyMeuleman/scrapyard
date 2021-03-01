//  on every year that is evenly divisible by 4
//   except every year that is evenly divisible by 100
//     unless the year is also evenly divisible by 400

pub fn is_leap_year(year: u64) -> bool {
    let div_by_4 = year % 4 == 0;
    let div_by_100 = year % 100 == 0;
    let div_by_400 = year % 400 == 0;
    div_by_4 && (!div_by_100 || div_by_400)
}

pub fn is_leap_year_v2(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
