//use gpio::{GpioIn, GpioOut};
use std::fs;
use std::time::{Duration, Instant};
use std::{thread, time};

fn fan_control(on: bool) {}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn main() {
    let mut filename = fs::read_to_string("filename").expect("No file pointer provided");
    let sleep_time = time::Duration::from_millis(10000);
    let cooling_start = 65;
    let cooling_stop = 45;
    let mut is_cooling = false;

    remove_whitespace(&mut filename);

    println!("{}", filename);

    loop {
        let start = Instant::now();

        let mut contents = fs::read_to_string(&filename)
            .expect("Something went wrong reading the temperature file");

        remove_whitespace(&mut contents);
        let temperature = (contents.parse::<i32>().unwrap()) / 1000;

        if temperature >= cooling_start && !is_cooling {
            is_cooling = true;
            println!("Cooling mode activated!");
        }

        if is_cooling && temperature < cooling_stop {
            is_cooling = false;
            println!("Cooling mode deactivated");
        }

        fan_control(is_cooling);

        let duration = start.elapsed();

        println!("{} C (took {:?})", temperature, duration);
        thread::sleep(sleep_time);
    }
}
