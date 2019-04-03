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
    let cal_delay = Duration::from_millis(10000);
    let delay = Duration::from_millis(1000);

    // Create a new drone driver
    // TODO Fix bind error
    let mut driver = h61::Driver::new();

    // Connect to drone
    let connection_res = driver.connect();

    match connection_res {
        Ok(_) => println!("Connected to drone."),
        Err(e) => panic!("Connection Error, check connection to drone"),
    };

    // // Calibrate drone 

    // println!("Calibrating...");

    // driver.calibrate()?;

    // println!("Sent!");

    // // Wait 2 seconds
    // thread::sleep(cal_delay);
    
    // Take off
    println!("Taking off...");

    driver.take_off()?;

    println!("Sent!");

    // Wait again
    thread::sleep(delay);

    // // Take off
    // println!("Hovering...");

    // driver.hover()?;

    // println!("Sent!");

    // Wait again
    // thread::sleep(delay);

    // Land
    println!("Landing...");

    driver.land()?;
    
    println!("Sent!");

    // Ta-dah!
    Ok(())
}

