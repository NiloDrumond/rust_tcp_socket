use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Instant,
};

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            let num: u32 = 10_000_000;
            let durations = [0; 10_000];
            let durations = durations.map(|_| {
                stream.write_all(&num.to_be_bytes()).unwrap();

                let mut data = [0; 6];
                let now = Instant::now();
                match stream.read(&mut data) {
                    Ok(size) => {
                        let _response = &data[0..size];
                    }
                    Err(e) => {
                        panic!("Failed to receive data: {}", e);
                    }
                };
                now.elapsed().as_millis()
            });
            let duration: u128 = durations.iter().sum::<u128>() / durations.len() as u128;
            println!("Average RTT: {}", duration);
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
