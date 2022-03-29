use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // Capturing the Environment with Closures
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
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

struct Cacher<T, P, R>
    where T: Fn(&P) -> R,
          P: std::cmp::Eq + std::hash::Hash
{
    calculation: T,
    values: HashMap<P, R>,
}
 
impl<T, P, R> Cacher<T, P, R>
    where T: Fn(&P) -> R,
          P: std::cmp::Eq + std::hash::Hash
{
    fn new(calculation: T) -> Cacher<T, P, R> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }
 
    fn value(&mut self, arg: P) -> &R {
        use std::collections::hash_map::Entry;
        match self.values.entry(arg) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let v = (self.calculation)(entry.key());
                entry.insert(v)
            }
        }
    }
}
