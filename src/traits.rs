/// Mod for traits regarding onboard functions. (i.e. Camera stream, LEDs, barometer, battery status, etc)
pub mod drone {
    use std::error::Error;
    
    // TODO Calibration trait
    /// Trait for drone calibration
    pub trait Calibrate {
        /// Calibrate the drone
        fn calibrate(&mut self) -> Result<(), Box<dyn Error>>;
    }

    /// Trait for drones that can hover
    pub trait Hover {
        /// Enable hover mode
        fn hover(&mut self) -> Result<(), Box<dyn Error>>;
    }


    // TODO Emergency trait
    // TODO VideoStream trait
    // TODO PhotoStream trait
    // TODO ReturnToSender trait
}

/// Mod for exposed control traits of drones (i.e. functions that affect rotor stats and
/// calibration)
pub mod control {
    use std::error::Error;

    /// pub trait for drones that can flip
    pub trait Flips {
        /// Preforms a Front Flip
        fn front_flip() -> Result<(), Box<dyn Error>>;

        /// Preforms a Back Flip
        fn back_flip() -> Result<(), Box<dyn Error>>;

        /// Side flip left
        fn left_flip() -> Result<(), Box<dyn Error>>;

        /// Side flip right
        fn righ_flip() -> Result<(), Box<dyn Error>>;
    }

    /// pub trait for drones that can do multi-dimensional flips (i.e. front + left at the same time)
    pub trait MultiFlips {
        /// Performs a front flip & a left-side flip at the same time
        fn front_left_flip() -> Result<(), Box<dyn Error>>;

        /// Performs a front flip & a right-side flip at the same time
        fn front_right_flip() -> Result<(), Box<dyn Error>>;

        /// Performs a back flip & a left-side flip at the same time
        fn back_left_flip() -> Result<(), Box<dyn Error>>;

        /// Performs a back flip & a right-side flip at the same time
        fn back_right_flip() -> Result<(), Box<dyn Error>>;
    }

    /// Movement pub trait (i.e. controls for Up, Down, Forward, Backwards, Left and Right);
    pub trait Movement {

        // TODO Determine length of time parameter (milliseconds or seconds)

        // X Axis
        /// Move the drone left for n milliseconds (takes in a usize)
        fn left(time: usize) -> Result<(), Box<dyn Error>>;

        /// Move the drone right for n milliseconds (takes in a usize)
        fn right(time: usize) -> Result<(), Box<dyn Error>>;

        // Y Axis

        /// Move the drone up for n milliseconds (takes in a usize)
        fn up(time: usize) -> Result<(), Box<dyn Error>>;
        /// Move the drone down for n milliseconds (takes in a usize)
        fn down(time: usize) -> Result<(), Box<dyn Error>>;

        // Z Axis

        /// Move the drone forwards for n milliseconds (takes in a usize)
        fn forwards(time: usize) -> Result<(), Box<dyn Error>>;
        /// Move the drone backwards for n milliseconds (takes in a usize)
        fn backwards(time: usize) -> Result<(), Box<dyn Error>>;

        // TODO Determine length of time parameter (milliseconds or seconds)

        /// Move the drone left for n milliseconds (takes in a usize)
        fn rot_left(time: usize) -> Result<(), Box<dyn Error>>;

        /// Move the drone right for n milliseconds (takes in a usize)
        fn rot_right(time: usize) -> Result<(), Box<dyn Error>>;
    }


    /// pub trait for Takeoff and Landing
    pub trait FlightControl {
        /// Performs a takeoff
        fn take_off(&mut self) -> Result<(), Box<dyn Error>>;

        /// Performs a landing
        fn land(&mut self) -> Result<(), Box<dyn Error>>;
    }
}
