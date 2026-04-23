use enigo::{Enigo, KeyboardControllable};
use std::env;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let start_time = Instant::now();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: text_entry <text> [delay_ms]");
        return;
    }
    
    let text = &args[1];
    let delay_ms = if args.len() > 2 {
        match args[2].parse::<u64>() {
            Ok(delay) => delay,
            Err(_) => {
                eprintln!("Invalid delay value. Using default 20ms.");
                20
            }
        }
    } else {
        20
    };
    
    let mut enigo = Enigo::new();
    
    for c in text.chars() {
        enigo.key_sequence(&c.to_string());
        if delay_ms > 0 {
            thread::sleep(Duration::from_millis(delay_ms));
        }
    }
    
    let elapsed = start_time.elapsed();
    println!("Input of '{}' succeeded. Elapsed time: {:?}", text, elapsed);
}
