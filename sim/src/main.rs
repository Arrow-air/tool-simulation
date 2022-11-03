//! Simulation Tool

use chrono::{Duration, Utc};
use clap::Parser;
use hyper::{Body, Error, Response};
use sim_types::cfg_types::Config;
use sim_types::eel_types::{Eel, EelEventType};
mod customer;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long)]
    input: String,
}

async fn action(event: &EelEventType) -> Result<Response<Body>, Error> {
    // Will add Weather and Civil Authority Events
    match event {
        EelEventType::CustomerEvent(s) => customer::action(s).await,
    }
}

async fn config_route(config: Config) -> Result<(), ()> {
    println!("Detected config file.");

    let sim_start_time = config.timestamp_start;
    let sim_end_time = sim_start_time + Duration::seconds(config.duration_s.into());
    let real_time_start = Utc::now();

    loop {
        let real_elapsed_time = Utc::now() - real_time_start;

        // TODO Implement actions

        if sim_start_time + real_elapsed_time >= sim_end_time {
            break;
        }
    }

    println!("Done!");
    Ok(())
}

async fn eel_route(eel: Eel) -> Result<(), ()> {
    println!("Detected EEL file.");
    if eel.events.is_empty() {
        eprintln!("No events parsed from EEL file.");
    }

    let sim_time_start = eel.events[0].timestamp;
    let real_time_start = Utc::now();
    println!("Sim Time Start: {:?}", sim_time_start);

    // Start from first event
    let mut event_iter = eel.events.iter();
    let mut next = event_iter.next();

    loop {
        match next {
            None => break,
            Some(e) => {
                let elapsed_time = Utc::now() - real_time_start;
                let sim_time = sim_time_start + elapsed_time;

                if e.timestamp > sim_time {
                    continue;
                }

                println!("{}: {:?}", e.timestamp, e.event);
                let result = action(&e.event).await;
                println!("{:?}", result);
                next = event_iter.next();
            }
        }
    }

    println!("End of EEL file! Simulation over.");

    Ok(())
}

/// Start a simulation from a file.
///
/// Examples:
/// ```
/// cargo run -p sim -- -i samples/cfg.yaml
/// cargo run -p sim -- -i samples/eel.json
/// ```
#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();

    let fname = args.input;
    match Eel::from_filename(&fname) {
        Ok(eel) => eel_route(eel).await,
        Err(_) => match Config::from_filename(&fname) {
            Ok(config) => config_route(config).await,
            Err(_) => {
                eprintln!("Could not parse input as an EEL or Config file.");
                Err(())
            }
        },
    }
}
