//! Validate EEL Files
use clap::Parser;
use sim_types::eel_types::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long)]
    input: String,
}

/// Validate a file given as a command line argument
fn main() -> Result<(), ()> {
    // Get Fields in Provided File
    let args = Args::parse();

    let fname = args.input;
    match Eel::from_filename(&fname) {
        Ok(_) => {
            println!("\u{1F370} Valid EEL File");
            Ok(())
        }
        Err(e) => {
            println!("\u{1F525} Invalid EEL File: {:?}", e);
            Err(())
        }
    }
}
