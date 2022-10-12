//! Generate an EEL File from a Configuration File

mod cfg_types {
    include!("../../../sim/src/cfg_types.rs");
}

mod eel_types {
    include!("../../../sim/src/eel_types.rs");
}

// fn test() {
//     let d = NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 12, 12);
//     let eel = Eel {
//         events: vec!(
//             EelEvent {
//                     event: EelType::CustomerEvent(
//                     CustomerRequest::CargoRequest(
//                     CargoRequest::CargoCreate(
//                         cargo_client_types::FlightQuery::new(
//                             "".to_string(),
//                             "".to_string(), d, d, 0.
//                         )
//                 ))),
//                 timestamp: NaiveDate::from_ymd(2022, 1, 1).and_hms(12, 12, 12)
//             }
//         )
//     };

//     println!("{}", serde_json::to_string(&eel).unwrap());
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() > 1);

    let _file_str = std::fs::read_to_string(&args[1]).unwrap_or_else(|_| "".to_string());

    Ok(())
}
