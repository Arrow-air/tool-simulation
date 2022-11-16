//! Simulation Tool

use chrono::{Duration, NaiveDateTime, Utc};
use clap::Parser;
use hyper::{Body, Response};
use rand::seq::SliceRandom;
use sim_types::cfg_types::{customer_agent::Customer, Config};
use sim_types::eel_types::{customer_events, Eel, EelEventType};

pub use svc_storage_client_grpc::client::{vertiport_rpc_client::VertiportRpcClient, SearchFilter};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long)]
    input: String,
}

async fn action(event: &EelEventType) -> Result<Response<Body>, ()> {
    // Will add Weather and Civil Authority Events
    match event {
        EelEventType::CustomerEvent(s) => customer_events::action(s).await,
    }
}

async fn config_route(config: Config) -> Result<(), ()> {
    println!("Detected config file.");

    // Initialize
    let sim_start_time: NaiveDateTime = config.timestamp_start;
    let sim_end_time = sim_start_time + Duration::seconds(config.duration_s.into());
    let real_time_start = Utc::now();

    // Initialize Customers
    let n_customers = config.n_customers;
    let mut customers: Vec<Customer> = vec![];
    for _ in 0..n_customers {
        let customer_type = config.customer_types.choose(&mut rand::thread_rng());
        if customer_type.is_none() {
            eprintln!("ERROR: Could not choose a customer type.");
            return Err(());
        }

        let c = Customer::generate(customer_type.unwrap(), sim_start_time);

        customers.push(c);
    }

    println!("Starting simulation.");
    loop {
        let real_elapsed_time = Utc::now() - real_time_start;

        if sim_start_time + real_elapsed_time >= sim_end_time {
            break;
        }

        // TODO Time Delays and Kickoff Times
        for x in &mut customers {
            x.next().await;
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
    println!("Sim Time Start: {:?}\n", sim_time_start);

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

                println!(
                    "EVENT @ {}\n{}",
                    e.timestamp,
                    serde_json::to_string_pretty(&e.event).unwrap()
                );
                let result = action(&e.event).await;
                println!("RESPONSE\n{:?}\n", result);
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
    if let Ok(eel) = Eel::from_filename(&fname) {
        eel_route(eel).await
    } else if let Ok(config) = Config::from_filename(&fname) {
        config_route(config).await
    } else {
        eprintln!("Could not parse input as an EEL or Config file.");
        Err(())
    }
}
