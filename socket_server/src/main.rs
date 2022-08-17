use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn find_nth_odd(n: u32) -> u32 {
    let mut i = 0;
    let mut odd_count = 0;

    while odd_count != n {
        i += 1;
        if i % 2 == 1 {
            odd_count += 1;
        }
    }

    i
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0; 50];
    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                let mut num = [0u8; 4];
                num.clone_from_slice(&data[0..size]);
                let num = u32::from_be_bytes(num);
                let nth_odd = find_nth_odd(num);
                stream.write_all(&nth_odd.to_be_bytes()).unwrap();
            }
            Err(_) => {
                println!(
                    "An error has occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(std::net::Shutdown::Both).unwrap();
                break;
            }
        }
    }
    println!("drop")
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Listening at port 3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);
}
