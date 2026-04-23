use enigo::{Enigo, Keyboard, Settings};
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
    
    let settings = Settings::default();
    let mut enigo = match Enigo::new(&settings) {
        Ok(enigo) => enigo,
        Err(_e) => {
            let elapsed = start_time.elapsed();
            println!("Input of '{}' failed: Failed to initialize keyboard. Elapsed time: {:?}", text, elapsed);
            return;
        }
    };
    
    let mut success = true;
    let mut error_message = String::new();
    for c in text.chars() {
        if let Err(_e) = enigo.text(&c.to_string()) {
            error_message = "Failed to type character".to_string();
            success = false;
            break;
        }
        if delay_ms > 0 {
            thread::sleep(Duration::from_millis(delay_ms));
        }
    }
    
    let elapsed = start_time.elapsed();
    if success {
        println!("Input of '{}' succeeded. Elapsed time: {:?}", text, elapsed);
    } else {
        println!("Input of '{}' failed: {}. Elapsed time: {:?}", text, error_message, elapsed);
    }
}
