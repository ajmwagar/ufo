/// Import std stuff
use std::error::Error;
use std::time::Duration;
use std::thread;

// Import traits
use ufo::traits::control::*;
use ufo::traits::drone::*;

// Import controller
use ufo::drones::jjrc::h61;

fn main() -> Result<(), Box<dyn Error>> {
    let delay = Duration::from_millis(2000);

    // Create a new drone driver
    let mut driver = h61::Driver::new();

    // Connect to drone
    driver.connect()?;

    // Calibrate drone 

    println!("Calibrating...");

    driver.calibrate()?;

    println!("Sent!");

    // Wait 2 seconds
    thread::sleep(delay);
    
    // Take off
    println!("Taking off...");

    driver.take_off()?;

    println!("Sent!");

    // Wait again
    thread::sleep(delay);

    // Land
    println!("Landing...");

    driver.land()?;
    
    println!("Sent!");

    // Ta-dah!
    Ok(())
}

