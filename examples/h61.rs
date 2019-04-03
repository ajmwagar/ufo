use std::error::Error;
use std::time::Duration;
use std::thread;

use ufo::traits::control::*;
use ufo::traits::drone::*;
use ufo::drones::jjrc::h61;

fn main() -> Result<(), Box<dyn Error>> {
    let delay = Duration::from_millis(2000);

    let mut Driver = h61::Driver::new();

    Driver.connect()?;

    println!("Calibrating...");
    Driver.calibrate()?;

    println!("Sent!");

    thread::sleep(delay);
    
    println!("Taking off...");

    Driver.take_off()?;

    println!("Sent!");

    thread::sleep(delay);

    println!("Landing...");

    Driver.land()?;
    
    
    println!("Sent!");


    Ok(())
}
