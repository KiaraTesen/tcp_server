#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate chrono;


use std::net::{TcpListener};
use std::{thread};


mod lp_handle_client;
mod pso_handle_client;
mod de_handle_client;

// Crate para obtener argumentos del usario
use clap::{arg, Command};

fn main() {


    let matches = Command::new("TCP Rust Server")
        .version("0.1.0")
        .author("Hermilo Cortés. <hermilocg@tec.mx>")
        .about("TCP Rust server that accept serialized struct.")
        .arg(arg!(--ip <VALUE>).required(true))
        .arg(arg!(--port <VALUE>).required(true))
        .arg(arg!(--service <VALUE>).required(true).help("What service to run the program in").value_parser(["DE", "LP","PSO"]))
        .arg(arg!(--subnet <VALUE>).required(true))
        .get_matches();


    let ip_dir = matches.get_one::<String>("ip").expect("required").to_string();

    let port = matches.get_one::<String>("port").expect("required").to_string();

    let service = matches.get_one::<String>("service").expect("required").to_string();

    let subnet_ip = matches.get_one::<String>("subnet").expect("required").to_string();

    let ip_port_dir = ip_dir + ":" + &port;

    println!("Listened from IP {}", ip_port_dir);
    println!("Service {}", service);
    
    let listener = TcpListener::bind(ip_port_dir).expect("Could not bind");

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("failed: {}", e),
            Ok(stream) => {

                let subnet_ip_temporal = subnet_ip.clone();
                let service_test = service.clone();
                
                thread::spawn(move || {

                    match service_test.as_str(){
                        "LP"  => lp_handle_client::handle_client(stream, subnet_ip_temporal).unwrap_or_else(|error| eprintln!("{:?}", error)),
                        "PSO" => pso_handle_client::handle_client(stream, subnet_ip_temporal).unwrap_or_else(|error| eprintln!("{:?}", error)),
                        "DE" => de_handle_client::handle_client(stream, subnet_ip_temporal).unwrap_or_else(|error| eprintln!("{:?}", error)),
                        _  => println!("anything"),
                    }
                    
                });
            }
        }
    }

}
