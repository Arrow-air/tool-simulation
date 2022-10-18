//! Validate EEL Files

use sim_types::eel_types::*;

/// Validate a file given as a command line argument
fn main() -> Result<(), ()> {
    // Get Fields in Provided File
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an EEL filename as a command line argument.");
        return Err(());
    }

    print!("{}: ", &args[1]);
    match Eel::from_filename(&args[1]) {
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
