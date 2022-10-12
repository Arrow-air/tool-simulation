// use serde_yaml; // 0.8.23
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{Error, ErrorKind};

pub use svc_cargo_client_rest::types as cargo_client_types;

///////////////////////////////////////////////////////////////////////
/// EEL File
///////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub enum EelType {
    /// Events triggered by a customer
    CustomerEvent(CustomerRequest),
    // WeatherEvent
    // AuthorityEvent
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EelEvent {
    /// The type of event
    pub event: EelType,

    /// The timestamp of the event
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Eel {
    pub events: Vec<EelEvent>,
}

///////////////////////////////////////////////////////////////////////
/// CustomerRequest Events
///////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub enum CustomerRequest {
    /// Request for a new cargo flight
    CargoRequest(CargoRequest),
    // Rideshare(RideshareRequest),
    // Charter(CharterRequest),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CargoRequest {
    /// Create a cargo flight
    CargoCreate(cargo_client_types::FlightQuery),

    /// Confirm a cargo flight
    CargoConfirm(cargo_client_types::FlightConfirm),

    /// Cancel a cargo flight
    CargoCancel(cargo_client_types::FlightCancel),
    // Modify(cargo_client_types::ModifyQuery) // R2
}

impl Eel {
    /// Validate an EEL file given a filename
    /// # Arguments
    ///
    /// * `fname` - The name of a sim configuration YAML file
    #[allow(dead_code)]
    pub fn from_filename(fname: &str) -> Result<Self, Error> {
        // Read in File to be parsed
        let input_str = std::fs::read_to_string(fname)?;

        // Get Eel Fields
        // let eel = cfg_types::Eel::default();
        // let yaml = serde_yaml::to_string(&config).unwrap();
        // let cfg_map: BTreeMap<String, String> = serde_yaml::from_str(&yaml).unwrap();

        // Get Fields from Provided File
        match serde_json::from_str::<Eel>(&input_str) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::InvalidInput, e)),
        }
    }
}
