//! Simulation Tool

mod cfg_types;
mod eel_types;

use cfg_types::*;
use eel_types::*;
use clap::Parser;
use chrono::{Duration, Utc};

#[derive(Debug, clap::ValueEnum, Clone)]
enum Filetype {
   Cfg,
   Eel
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Either "cfg" or "eel"
   #[arg(value_enum)]
   ftype: Filetype,

   /// Number of times to greet
   #[arg(short, long)]
   filename: String,
}

fn config_route(fname: &str) -> Result<(), ()> {
    let result = Config::from_filename(fname);
    if result.is_err() {
        println!("\u{1F525} Invalid config file: {:?}", result.unwrap_err());
        return Err(());
    }

    let config = result.unwrap();
    let sim_start_time = config.timestamp_start;
    let sim_end_time = sim_start_time + Duration::seconds(config.duration_s.into());
    let time_start = Utc::now();

    loop {
        let elapsed_time = Utc::now() - time_start;

        // TODO Implement actions

        if sim_start_time + elapsed_time >= sim_end_time {
            break;
        }
    }

    println!("Done!");
    Ok(())
}

fn eel_route(fname: &str) -> Result<(), ()> {
    let result = Eel::from_filename(fname);
    if result.is_err() {
        println!("\u{1F525} Invalid EEL file: {:?}", result.unwrap_err());
        return Err(());
    }

    //  TODO run from EEL file
    // let eel = result.unwrap();
    // for e in eel.events {
    //     println!("{}", e.timestamp);
    // }

    Ok(())
}

/// Start a simulation from a file.
/// 
/// Examples:
/// ```
/// cargo run -p sim -- --filename samples/cfg.yaml cfg
/// cargo run -p sim -- --filename samples/eel.json eel
/// ```
fn main() -> Result<(), ()> {
    let args = Args::parse();

    let ftype = args.ftype;
    let fname = args.filename;
    match ftype {
        Filetype::Cfg => config_route(&fname),
        Filetype::Eel => eel_route(&fname)
    }
}
