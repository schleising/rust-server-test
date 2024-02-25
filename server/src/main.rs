use std::net::{TcpListener, IpAddr, Ipv4Addr, SocketAddr};
use std::io::{BufRead, BufReader, Write};



fn main() {
    // Define the port to listen on
    let port: u16 = 8080;
    let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    // Define the address to listen on
    let socket_addr = SocketAddr::from((ip_address, port));

    // Create a TCP listener
    let listener = TcpListener::bind(socket_addr).unwrap();

    // Print a message to the console
    println!("Server listening on http://{}", socket_addr);

    // Loop forever, accepting connections
    for mut stream in listener.incoming() {
        // Read the stream into a buffer
        let mut reader = BufReader::new(stream.as_mut().unwrap());

        // Print a message to the console
        println!("===============================");
        println!("Connection established!");
        println!("===============================");

        let mut endpoint = String::new();

        // Read each line from the buffer
        loop {
            let mut buffer = String::new();
            reader.read_line(&mut buffer).unwrap();
            if buffer.trim().is_empty() {
                break;
            }
            print!("{}", buffer);

            if buffer.starts_with("GET") {
                // Get the endpoint from the buffer
                endpoint = buffer.split_whitespace().nth(1).unwrap().to_string();

                // Strip the leading slash
                endpoint = endpoint[1..].to_string();
            }
        }

        // Print a message to the console
        println!("===============================");
        println!("Connection closed!");
        println!("===============================");
        println!();

        // Write a response to the stream
        let response = format!("HTTP/1.1 200 OK\r\n\r\nHello, {}!", endpoint);
        stream.unwrap().write(response.as_bytes()).unwrap();
        
    }
}
