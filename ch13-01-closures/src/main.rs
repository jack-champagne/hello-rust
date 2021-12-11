use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 5;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // Cannot do the following:
    // let my_invalid_closure = |x| x;

}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };


    if intensity < 25 {
        println!(
            "Today, do {} pushups.",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps.",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today and stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes.",
                expensive_closure(intensity)
            );
        }
    }
}

