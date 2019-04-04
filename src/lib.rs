use std::net::UdpSocket;
use std::net::TcpStream;
use std::error::Error;
use std::io::prelude::*;
use hex;

pub mod drones;
pub mod traits;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod errors {
    use std::fmt;
    
    /// Error for issues with socket connection
    pub struct ConnectionError;

    // Implement std::fmt::Display for ConnectionError
    impl fmt::Display for ConnectionError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "An Connection Error Occurred, Please confirm that you are connected to the drone's WiFi network!") // user-facing output
        }
    }

    // Implement std::fmt::Debug for ConnectionError
    impl fmt::Debug for ConnectionError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
        }
    }


    /// Error for issues with drone executing commands
    pub struct CommandError;

    // Implement std::fmt::Display for CommandError
    impl fmt::Display for CommandError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "An Command Error Occurred, Please confirm the hex values for your drone are valid") // user-facing output
        }
    }

    // Implement std::fmt::Debug for CommandError
    impl fmt::Debug for CommandError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
        }
    }
}

/// Connection of a drone
pub struct DroneUdpConnection {
    /// Ip to listen on
    bind_ip: String,
    /// Port to listen on
    bind_port: String,

    /// IP to connect to
    connect_ip: String,
    /// Port to connect to
    connect_port: String,

    /// Socket of drone
    sock: UdpSocket
}

impl DroneUdpConnection {

    /// Create a new drone connection takes in `bind_ip`, `bind_port`, `connect_ip`, and `connect_port` all as `Strings`
    pub fn new(bind_ip: String, bind_port: String, connect_ip: String, connect_port: String) -> Self {
        let mut bind_url = String::new();

        bind_url.push_str(&bind_ip);
        bind_url.push_str(":");
        bind_url.push_str(&bind_port);

        DroneUdpConnection {
            bind_ip,
            bind_port,
            connect_ip,
            connect_port,
            // Not yet connected
            sock: UdpSocket::bind(bind_url).unwrap()
        }
    }

    /// Connect DroneConnection to drone
    pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        // // Create binding url
        // let mut bind_url = String::new();

        // bind_url.push_str(&self.bind_ip);
        // bind_url.push_str(":");
        // bind_url.push_str(&self.bind_port);


        // // Bind to port
        // self.sock = UdpSocket::bind(bind_url)?;

        // Create connection url
        let mut conn_url = String::new();
        conn_url.push_str(&self.connect_ip);
        conn_url.push_str(":");
        conn_url.push_str(&self.connect_port);

        // Connect to socket
        self.sock.connect(conn_url)?;

        Ok(())

    }

    /// Send a static command to the drone
    pub fn send_command(&mut self, hex_str: String) -> Result<(), Box<dyn Error>> {
        // Convert hex codes
        let command: &[u8] = &hex::decode(&hex_str)?;

        println!("{:?}", command);
        // println!("{:?}", String::from_utf8(command.to_vec());

        // Send to socket
        self.sock.send(command)?;

        // Return we're okay
        Ok(())
    }

}

/// Status of a drone 
pub struct DroneStatus{
    /// Battery charge of the drone
    bat_charge: usize,
    /// Capacity of drone battery
    bat_cap: usize
}

pub struct DroneTcpConnection {
    /// Ip to listen on
    bind_ip: String,
    /// Port to listen on
    bind_port: String,

    /// IP to connect to
    connect_ip: String,
    /// Port to connect to
    connect_port: String,

    /// Socket of drone
    sock: TcpStream
}

impl DroneTcpConnection {

    /// Create a new drone connection takes in `bind_ip`, `bind_port`, `connect_ip`, and `connect_port` all as `Strings`
    pub fn new(bind_ip: String, bind_port: String, connect_ip: String, connect_port: String) -> Self {
        let mut connect_url = String::new();

        connect_url.push_str(&connect_ip);
        connect_url.push_str(":");
        connect_url.push_str(&connect_port);

        DroneTcpConnection {
            bind_ip,
            bind_port,
            connect_ip,
            connect_port,
            // Not yet connected
            sock: TcpStream::connect(connect_url).unwrap()
        }
    }

    // /// Connect DroneConnection to drone
    // pub fn connect(&mut self) -> Result<(), Box<dyn Error>> {
    //     // self.sock = UdpSocket::bind(bind_url)?;

    //     // Create connection url
    //     let mut conn_url = String::new();
    //     conn_url.push_str(&self.connect_ip);
    //     conn_url.push_str(":");
    //     conn_url.push_str(&self.connect_port);

    //     // Connect to socket
    //     self.sock.connect(conn_url)?;

    //     Ok(())

    // }

    /// Send a static command to the drone
    pub fn send_command(&mut self, hex_str: String) -> Result<(), Box<dyn Error>> {
        // Convert hex codes
        let command: &[u8] = &hex::decode(&hex_str)?;

        println!("{:?}", command);
        // println!("{:?}", String::from_utf8(command.to_vec());

        // Send to socket
        self.sock.write(command)?;

        // Return we're okay
        Ok(())
    }

    pub fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buffer: &mut[u8] = &mut [0; 128];

        self.sock.read(&mut buffer)?;

        Ok(buffer.to_owned())
    }

}
