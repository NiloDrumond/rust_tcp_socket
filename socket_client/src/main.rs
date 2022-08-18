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
                now.elapsed().as_millis() as i32
            });
            let mean_duration: i32 = durations.iter().sum::<i32>() / durations.len() as i32;
            let variance: f32 = durations
                .iter()
                .map(|value| {
                    let diff: i32 = mean_duration - value;
                    diff * diff
                })
                .sum::<i32>() as f32
                / durations.len() as f32;
            let std_deviation = variance.sqrt();
            println!("Average RTT: {}", mean_duration);
            println!("Standard Deviation: {}", std_deviation);
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
