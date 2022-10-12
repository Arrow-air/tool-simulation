//! Validate Configuration Files
//! Test

mod cfg_types {
    include!("../../../sim/src/cfg_types.rs");
}

/// Validate a file given as a command line argument
fn main() -> Result<(), ()> {
    // Get Fields in Provided File
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a config filename as a command line argument.");
        return Err(());
    }

    print!("{}: ", &args[1]);
    match cfg_types::Config::from_filename(&args[1]) {
        Ok(_) => {
            println!("\u{1F370} Valid config file");
            Ok(())
        }
        Err(e) => {
            println!("\u{1F525} Invalid config file: {:?}", e);
            Err(())
        }
    }
}
