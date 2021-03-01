use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock(i32);

const MINUTES_IN_DAY: i32 = 60 * 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let num = Self::to_internal_num(hours * 60 + minutes);
        Self(num)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let num = Self::to_internal_num(self.0 + minutes);
        Self(num)
    }

    fn hours(&self) -> i32 {
        self.0 / 60
    }

    fn minutes(&self) -> i32 {
        self.0 % 60
    }

    fn to_internal_num(minutes: i32) -> i32 {
        // because % is not the modulus operator but the remainder operator:
        // get the smallest positive remainder
        minutes.rem_euclid(MINUTES_IN_DAY)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
