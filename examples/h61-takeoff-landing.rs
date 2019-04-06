/// Import std stuff
use std::error::Error;
use std::thread;
use std::time::Duration;

// Import traits
use ufo_rs::traits::control::*;
use ufo_rs::traits::drone::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Import controller
    use ufo_rs::drones::jjrc::h61;

    // let cal_delay = Duration::from_millis(10000);
    let delay = Duration::from_millis(1000);

    // Create a new drone driver
    // TODO Fix bind error when not connected to drone
    let mut driver = h61::Driver::new();

    // Connect to drone
    let connection_res = driver.connect();

    match connection_res {
        Ok(_) => println!("Connected to drone."),
        Err(e) => panic!("Connection Error, check connection to drone"),
    };

    // Take off
    println!("Taking off...");

    driver.take_off()?;

    println!("Sent!");

    // Wait for a second
    thread::sleep(delay);

    // Land
    println!("Landing...");

    driver.land()?;

    println!("Sent!");

    // Ta-dah!
    Ok(())
}
