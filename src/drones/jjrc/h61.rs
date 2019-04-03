// Example implementation of a custom drone controller

use crate::traits::*;
use std::error::Error;

/// Bind IP
const BIND_IP: &str = "172.16.10.59";

/// Port to receive data on
const BIND_PORT: &str = "8080";

/// IP of drone
const CONN_IP: &str = "172.16.10.1";

/// UDP port to connect to drone on;
const CONN_UDP_PORT: &str = "8080";

// Hex Codes for UDP socket
/// Hex codes for different commands
pub enum UdpHex {
    /// Hex code for calibration
    Calibrate,
    /// Hex code for takeoff
    Start,
    /// Hex code for landing
    Land,
    /// Hex code for hovering
    Hover,
    /// Hex code for stopping
    Stop,
    /// Hex code for moving up
    Up,
    /// Hex code for moving down
    Down,
    /// Hex code for moving left
    Left,
    /// Hex code for moving right
    Right,
    /// Hex code for moving forwards
    Forwards,
    /// Hex code for moving backwards
    Backwards,
    /// Hex code for rotating left
    RotLeft,
    /// Hex code for rotating right
    RotRight,



}

impl UdpHex {
    /// Returns hex string of command
    pub fn value(&self) -> String {
        // Define hex codes
        match self {
            UdpHex::Calibrate => "ff087e3f403fd0121200cb".to_string(),
            UdpHex::Start => "ff087e3f403f90121240c7".to_string(),
            UdpHex::Land => "ff087e3f403f9012128087".to_string(),
            UdpHex::Hover => "ff08fc3f403f9012120089".to_string(),
            UdpHex::Stop => "ff087e3f403f901212a069".to_string(),
            // TODO Check if these need to be static
            UdpHex::Up => "ff08fc3f403f9012120089".to_string(),
            UdpHex::Down => "ff08003f403f9012120085".to_string(),
            UdpHex::Left => "ff087e3f40009012120046".to_string(),
            UdpHex::Right => "ff087e3f407e90121200c8".to_string(),
            UdpHex::Forwards => "ff087e3f013f9012120046".to_string(),
            UdpHex::Backwards => "ff087e3f7f3f90121200c8".to_string(),
            UdpHex::RotLeft => "ff087e00403f9012120046".to_string(),
            UdpHex::RotRight => "ff087e7e403f90121200c8".to_string(),
        }

    }
}


// Code for drone programmer-facing API

/// UFO controls of the JJRC H61 foldable drone
pub struct Driver {

    /// UDP Drone controller, handles connections etc.
    connection: crate::DroneUdpConnection,
    // /// Status of drone
    // status: crate::DroneStatus,
    // /// Tuple with (X,Y,Z) coordinates of drone, optional but helpful for ReturnToSender and altitude lock
    // pos: (isize,isize,isize)
}

// Implement constructor
impl Driver {
    /// Create a new driver and binds to default IP
    pub fn new() -> Self {
        // create new connection
        Driver {
            connection: crate::DroneUdpConnection::new(BIND_IP.to_string(), BIND_PORT.to_string(), CONN_IP.to_string(), CONN_UDP_PORT.to_string())
        }
    }

    /// Connect to drone
    pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection.connect()
    }
}

// Implement calibrate command
impl drone::Calibrate for Driver {
    fn calibrate(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection.send_command(UdpHex::Calibrate.value())
    }
}

// Implement hover command
impl drone::Hover for Driver {
    fn hover(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection.send_command(UdpHex::Hover.value())
    }
}


// Implement FlightControl for H61 Driver
impl control::FlightControl for Driver {
    fn take_off(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection.send_command(UdpHex::Start.value())
    }

    fn land(&mut self) -> Result<(), Box<dyn Error>> {
        self.connection.send_command(UdpHex::Land.value());
        self.connection.send_command(UdpHex::Stop.value())
    }
}

// Add movement controls
impl control::Movement for Driver {
    fn left(&mut self, time: usize) -> Result<(), Box<dyn Error>> {
        self.connection.send_command(UdpHex::Left.value())
    }

    fn right(&mut self, time: usize) -> Result<(), Box<dyn Error>> {
        self.connection.send_command(UdpHex::Right.value())
    }

    fn up(&mut self, time: usize) -> Result<(), Box<dyn Error>> {
        self.connection.send_command(UdpHex::Up.value())

    }
    fn down(&mut self, time: usize) -> Result<(), Box<dyn Error>> {
        self.connection.send_command(UdpHex::Down.value())
    }

    // Z Axis

    fn forwards(&mut self, time: usize) -> Result<(), Box<dyn Error>> {
        self.connection.send_command(UdpHex::Forwards.value())

    }
    fn backwards(&mut self, time: usize) -> Result<(), Box<dyn Error>> {
        self.connection.send_command(UdpHex::Backwards.value())

    }

    // TODO Determine length of time parameter (milliseconds or seconds)

    fn rot_left(&mut self, time: usize) -> Result<(), Box<dyn Error>> {
        self.connection.send_command(UdpHex::RotLeft.value())

    }
    fn rot_right(&mut self, time: usize) -> Result<(), Box<dyn Error>> {
        self.connection.send_command(UdpHex::RotRight.value())
    }
}
