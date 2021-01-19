use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::cmp;
use std::hash;

struct Cacher<T, U, V>
where
    T: Fn(&U) -> V,
    U: cmp::Eq + hash::Hash,
{
    calculation: T,
    values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(&U) -> V,
    U: cmp::Eq + hash::Hash,
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }
    fn value(&mut self, arg: U) -> &V {
        match self.values.entry(arg) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let v = (self.calculation)(entry.key());
                // insert sets the value of the entry with the VacantEntry's key, and returns a mutable reference to it.
                entry.insert(v)
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating...");
        thread::sleep(Duration::from_secs(2));
        *num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
fn main() {
    let simulated_user_specified_value = 50;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
