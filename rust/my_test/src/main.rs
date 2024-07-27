use std::io::{self, Write};
use std::process::Command;
use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
    let mut total_time = Duration::new(0, 0);
    for _ in 0..4 {
        let start = Instant::now();
        let output = Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg("google.com")
            .output()?;
        let end = Instant::now();
        let round_trip_time = end - start;
        total_time += round_trip_time;
    }
    let average_time = total_time / 4;
    println!("Average round-trip time: {:?}", average_time);
    Ok(())
}