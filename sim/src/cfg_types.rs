// Types used for Configuration Files
//

// use serde_yaml; // 0.8.23
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::{Error, ErrorKind};

/// Configuration File Fields
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Date and Time for the Simulation to Start
    pub timestamp_start: NaiveDateTime,

    // Duration of simulation
    pub duration_s: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            timestamp_start: NaiveDate::from_ymd(2021, 12, 31).and_hms(23, 59, 59),
            duration_s: 10,
        }
    }
}

impl Config {
    /// Validate an EEL file given a filename
    /// # Arguments
    ///
    /// * `fname` - The name of a sim configuration YAML file
    #[allow(dead_code)]
    pub fn from_filename(fname: &str) -> Result<Self, Error> {
        // Read in File to be parsed
        let input_str = std::fs::read_to_string(fname)?;

        // Get Config Fields
        let config = Config::default();
        let yaml = serde_yaml::to_string(&config).unwrap();
        let cfg_map: BTreeMap<String, String> = serde_yaml::from_str(&yaml).unwrap();

        // Get Fields from Provided File
        let map: BTreeMap<String, String> = serde_yaml::from_str(&input_str).unwrap();
        for (key, _) in map.iter() {
            if !cfg_map.contains_key(key) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("field name '{key}'"),
                ));
            }
        }

        match serde_yaml::from_str::<Config>(&input_str) {
            Ok(e) => Ok(e),
            Err(e) => Err(Error::new(ErrorKind::InvalidInput, e)),
        }
    }
}
