# 🚀 👾  U.F.O: Universal Flying Objects

[![ufo](https://docs.rs/ufo_rs/badge.svg)](https://docs.rs/ufo_rs) [![dependency status](https://deps.rs/repo/github/ajmwagar/ufo/status.svg)](https://deps.rs/repo/github/ajmwagar/ufo) [![Gitter chat](https://badges.gitter.im/gitterHQ/gitter.png)](https://gitter.im/ufo_rs/community)

A `Drone/UAV/Quadcopter/RC Plane` programming library for `Rust`.

__FYI: this is still a WIP, the traits and structs exist, but examples and documentation are lacking... Proceed with caution.__

__NOTE: I broke my only quadcopter. When I get a new one, I will continue to work on this library. Until then I will accept pull requests and issues, but no active development will happen.__

## Goals

1. Build an extensible modular library that can support an infinite number of UAVs.
2. Handle as much of the boiler plate as possible. (Things just work)
3. Low Latency Between Drone and Controller

## Non-Goals

- Implement A UI for controlling UAVs __(NOTE: I may decide to build a UI later on, but it will be in its own repository)__
  - For controlling the drones using a keyboard checkout [mothership](https://github.com/ajmwagar/mothership)
- Support other types of robotics/RCs, right now this crate is for RC Drones and Planes. This may change at a later date, but not right now.

## Supported drones

Right now `UFO` supports the following drones:
- `JJRC H61`

Feel free to implement controls for your own drone and submit it in a pull request.

## Installation and usage

Add this to your `Cargo.toml`

```
[dependencies]
ufo_rs = "*"
```

And put something like this in `src/main.rs`

```rust
/// Import std stuff
use std::error::Error;
use std::time::Duration;
use std::thread;

// Import traits
use ufo_rs::traits::control::*;
use ufo_rs::traits::drone::*;

// Import controller
use ufo_rs::drones::jjrc::h61;

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

    // Wait again for 1 second
    thread::sleep(Duration::from_millis(1000));

    // Land
    println!("Landing...");

    driver.land()?;
    
    println!("Sent!");


    // Ta-dah!
    Ok(())
}

```

See the `examples/` directory for more information.

## Help out

If you want to help with U.F.O, we could use support in two areas:

1. Writing Unit Tests, I want to make sure the `U.F.O.` is a safe and stable way to program drones. If we can find areas prone to bugs, we can build a more stable library quickly. 
2. Adding support for more drones. If you would like to help with this or to see an example, please take a look at `src/drones/jjrc/h61.rs`. 

## Inspiration:

Inspired by:

- [Gobot](https://gobot.io): Really cool robotics library for `Go`
- A lack of a drone library for `Rust`
- Flying things with code is fun 😄!
