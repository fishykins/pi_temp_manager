//use gpio::{GpioIn, GpioOut};
use std::fs;
use std::{thread, time};

fn fan_control(on: bool) {}

fn main() {
    let filename = "test";
    let sleep_time = time::Duration::from_millis(1000);
    let cooling_start = 65;
    let cooling_stop = 45;
    let mut is_cooling = false;

    // Let's open GPIO23 and -24, e.g. on a Raspberry Pi 2.
    //let mut gpio23 = gpio::sysfs::SysFsGpioInput::open(23).unwrap();
    //let mut gpio24 = gpio::sysfs::SysFsGpioOutput::open(24).unwrap();

    loop {
        let contents =
            fs::read_to_string(filename).expect("Something went wrong reading the temperature");

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

        println!("{} C", temperature);
        thread::sleep(sleep_time);
    }
}
