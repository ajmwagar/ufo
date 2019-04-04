
/// Import std stuff
use std::error::Error;
use std::time::Duration;
use std::thread;

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

    driver.down(10)?;
    for _ in 0..10 {
        driver.forwards(10)?;
    }

    thread::sleep(Duration::from_millis(20000));

    // Land
    println!("Landing...");

    driver.land()?;
    
    println!("Sent!");

    // Ta-dah!
    Ok(())
}
