use rodio::{Decoder, OutputStream, Sink};
use std::io::{self, Write, Cursor};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Do you want to enter BPM (beats per minute) or BPS (beats per second)? ");
    print!("Enter 'm' for BPM or 's' for BPS: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let bps: f64;
    if input.trim().to_lowercase() == "m" {
        println!("Enter BPM: ");
        let mut bpm_input = String::new();
        io::stdin().read_line(&mut bpm_input).unwrap();
        let bpm: f64 = bpm_input.trim().parse().expect("Invalid number");
        bps = bpm / 60.0;
    } else if input.trim().to_lowercase() == "s" {
        println!("Enter BPS: ");
        let mut bps_input = String::new();
        io::stdin().read_line(&mut bps_input).unwrap();
        bps = bps_input.trim().parse().expect("Invalid number");
    } else {
        eprintln!("Invalid input. Please enter 'm' for BPM or 's' for BPS.");
        return;
    }

    let interval_ms = (1000.0 / bps) as u64;

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    loop {
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Include the asset directly in the binary
        let bytes: &[u8] = include_bytes!("../assets/click.wav");
        let source = Decoder::new(Cursor::new(bytes)).unwrap();

        sink.append(source);
        sink.detach(); // Play asynchronously

        thread::sleep(Duration::from_millis(interval_ms));
    }
}
