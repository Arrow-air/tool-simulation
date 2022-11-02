mod customer_events;
pub use customer_events::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json;

///////////////////////////////////////////////////////////////////////
/// EEL File
///////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub enum EelEventType {
    /// Events triggered by a customer
    CustomerEvent(CustomerEvent),
    // WeatherEvent
    // AuthorityEvent
}

/// External Event
#[derive(Debug, Serialize, Deserialize)]
pub struct EelEvent {
    /// The type of event
    pub event: EelEventType,

    /// The timestamp of the event
    pub timestamp: NaiveDateTime,
}

/// External Event Log
#[derive(Debug, Serialize, Deserialize)]
pub struct Eel {

    /// A list of external events
    pub events: Vec<EelEvent>,
}

impl Eel {
    /// Validate an EEL file given a filename
    /// # Arguments
    ///
    /// * `fname` - The name of a sim configuration YAML file
    pub fn from_filename(fname: &str) -> Result<Self, serde_json::Error> {
        // Read in File to be parsed
        let input_str = match std::fs::read_to_string(fname) {
            Err(_) => "".to_string(),
            Ok(s) => s
        };

        serde_json::from_str::<Eel>(&input_str)
    }
}
