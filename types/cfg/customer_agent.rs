use std::time::Duration;
use std::time::SystemTime;
use rand::seq::SliceRandom;
use rand::Rng;
use hyper::{StatusCode, body};
use uuid::Uuid;

use svc_cargo_client_rest::types::{
    VertiportsQuery,
    Vertiport,
    FlightOption,
    FlightConfirm,
    FlightCancel,
    FlightQuery
};

mod customer_events {
    include!("../events/customer_events.rs");
}

use customer_events::{
    CustomerEvent,
    CargoRequest
};

/// Phases of customer activity
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CustomerStatus {
    /// Retrieving list of local vertiports
    Vertiports,

    /// Hasn't checked for available flights yet
    Query,

    /// Has a list of flight options to select
    Confirm,

    /// Debating on cancelling
    Cancel,

    /// No more actions to be taken
    Done
}

// #[derive(Default)]
// pub struct CustomerStatistics {
//     spawn_time: Option<SystemTime>,
//     query_wait: Option<Duration>,
//     confirm_wait: Option<Duration>,
//     cancel_wait: Option<Duration>
// }

/// Customer Details
#[allow(missing_debug_implementations)]
pub struct Customer {
    id: Uuid,
    behavior: Box<dyn CustomerBehavior>,
    status: CustomerStatus,
    vertiport_depart_id: String,
    vertiport_arrive_id: String,
    current_time: SystemTime,
    fp_id: String,
    flights: Vec<FlightOption>,
    retries: i8
}

/// How customers exhibit behaviors
trait CustomerBehavior {
    /// How a customer responds to a list of available flights
    fn confirm(&self, flights: &[FlightOption]) -> Option<String>;

    /// Willingness to wait N seconds
    // fn patience(&self, seconds: Duration) -> bool;

    /// Delay Range
    // fn delay_seconds(&self,) -> (u16, u16);

    /// Probability of cancelling a confirmed flight
    fn cancel_chance(&self) -> f32;
}

/// Greedy customers take the first thing they can get and don't cancel
#[derive(Debug, Clone, Copy)]
pub struct GreedyCustomer;

impl CustomerBehavior for GreedyCustomer {
    fn confirm(&self, flights: &[FlightOption]) -> Option<String> {
        // Just confirm the first available flight, immediately
        flights.get(0).map(|f| f.fp_id.clone())
    }

    fn cancel_chance(&self) -> f32 {
        0.0
    }
}

/// Mistake customers create a booking on accident and need to cancel
#[derive(Debug, Clone, Copy)]
pub struct MistakeCustomer;

impl CustomerBehavior for MistakeCustomer {
    fn confirm(&self, flights: &[FlightOption]) -> Option<String> {
        // Just confirm the first available flight, immediately
        flights.get(0).map(|f| f.fp_id.clone())
    }

    fn cancel_chance(&self) -> f32 {
        // Will cancel
        1.0
    }
}

/// Indecisive customers query but never select anything
#[derive(Debug, Clone, Copy)]
pub struct IndecisiveCustomer;

impl CustomerBehavior for IndecisiveCustomer {
    fn confirm(&self, _flights: &[FlightOption]) -> Option<String> {
        None
    }

    fn cancel_chance(&self) -> f32 {
        // Will cancel
        1.0
    }
}

impl Customer {
    /// Creates a customer, assigns it a behavior and desired itinerary details
    pub fn generate(
        customer_type: &str,
        current_time: chrono::NaiveDateTime
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let uuid = Uuid::new_v4();
        println!("Creating '{}' customer {}", customer_type, uuid);
        let customer: Box<dyn CustomerBehavior> = match customer_type {
            "greedy" => Box::new(GreedyCustomer),
            "mistake" => Box::new(MistakeCustomer),
            "indecisive" => Box::new(IndecisiveCustomer),

            _ => {
                println!("Invalid customer type '{}'; producing greedy customer", customer_type);
                Box::new(GreedyCustomer)
            }
        };

        let time = SystemTime::try_from(
            prost_types::Timestamp {
                seconds: current_time.timestamp(),
                nanos: current_time.timestamp_subsec_nanos() as i32
            }
        );

        match time {
            Ok(t) => { 
                Ok(Customer {
                    id: uuid,
                    behavior: customer,
                    status: CustomerStatus::Vertiports,
                    vertiport_depart_id: "".to_string(),
                    vertiport_arrive_id: "".to_string(),
                    current_time: t,
                    fp_id: "".to_string(),
                    flights: vec!(),
                    retries: 1
                })
            },
            Err(e) => {
                eprintln!("ERROR: Could not parse timestamp.");
                Err(Box::new(e))
            }
        }
    }

    /// Prints a customer ID and log message to stdout
    pub fn log(&self, s: &str) {
        println!("{:?}: {}", self.id, s);
    }

    async fn handle_vertiports(&mut self) -> bool {
        let query = CustomerEvent::CargoRequest(
            CargoRequest::Vertiports(
                // Arbitrary, not currently used
                VertiportsQuery {
                    latitude: 100.,
                    longitude: 100.
                }
            )
        );

        self.log("Attempting to query for vertiports...");

        // Get response
        let act = customer_events::action(&query).await;
        if let Err(e) = act {
            self.log(&format!("Failed to get vertiports: {:?}", e));
            return false;
        }
        let resp = act.unwrap(); // safe
        if resp.status() != StatusCode::OK {
            self.log(&format!("Bad Response: {}", resp.status()));
            return false;
        }

        // Get bytes
        let bytes = body::to_bytes(resp.into_body()).await;
        if bytes.is_err() {
            self.log("Failed to convert response to bytes.");
            return false;
        }
        let bytes = bytes.unwrap(); // safe

        // Get vertiports from bytes
        let res = serde_json::from_slice(&bytes);
        if res.is_err() {
            self.log("Failed to get vertiports from bytes.");
            return false;
        }
        let mut vertiports: Vec<Vertiport> = res.unwrap(); // safe
        if vertiports.len() < 2 {
            self.log(&format!("Not enough vertiports available: {}.", vertiports.len()));
            return false;
        }

        self.log(&format!("Received {} vertiports.", vertiports.len()));

        vertiports = vertiports.choose_multiple(&mut rand::thread_rng(), 2).cloned().collect();
        self.vertiport_depart_id = vertiports[0].id.clone();
        self.vertiport_arrive_id = vertiports[1].id.clone();
        self.log(&format!(
            "Picking two arbitrary vertiports: {} and {}",
            self.vertiport_depart_id,
            self.vertiport_arrive_id
        ));
        self.status = CustomerStatus::Query;
        true
    }

    /// Queries for available routes
    /// # Returns
    /// true if successful
    async fn handle_query(&mut self) -> bool {
        let query = CustomerEvent::CargoRequest(
            CargoRequest::Query(
                FlightQuery {
                    vertiport_depart_id: self.vertiport_depart_id.clone(),
                    vertiport_arrive_id: self.vertiport_arrive_id.clone(),
                    timestamp_depart_min: None,
                    timestamp_depart_max: None,
                    timestamp_arrive_min: Some(self.current_time + Duration::from_secs(60)),
                    timestamp_arrive_max: Some(self.current_time + Duration::from_secs(600)),
                    cargo_weight_kg: 1.0,
                }
            )
        );

        self.log("Attempting to query for flight...");
        let act = customer_events::action(&query).await;
        if let Err(e) = act {
            self.log(&format!("Failed to query: {:?}", e));
            return false;
        }
        let resp = act.unwrap(); // safe
        if resp.status() != StatusCode::OK {
            self.log(&format!("Bad Response: {}", resp.status()));
            return false;
        }

        // Get bytes
        let bytes = body::to_bytes(resp.into_body()).await;
        if bytes.is_err() {
            self.log("Failed to convert response to bytes.");
            return false;
        }
        let bytes = bytes.unwrap(); // safe

        // Get Queries
        let res = serde_json::from_slice(&bytes);
        if res.is_err() {
            self.log("Failed to get query from bytes");
            return false;
        }
        self.flights = res.unwrap(); // safe

        if self.flights.is_empty() {
            self.log("No routes available.");
            return false;
        }

        for f in &self.flights {
            let price: String = match f.base_pricing {
                Some(p) => p.to_string(),
                None => "UNK".to_string()
            };
            self.log(&format!("Option: {} ({:?} USD)", f.fp_id, price));
        }

        self.log(&format!("Received {} flight options.", self.flights.len()));
        self.status = CustomerStatus::Confirm;
        true
    }

    /// Confirms the flight plan that the customer selected
    /// # Returns
    /// true if successful
    async fn handle_confirm(&mut self) -> bool {
        let ret = self.behavior.confirm(&self.flights);
        if ret.is_none() {
            self.log("Did not select a flight.");
            return false;
        }
        let draft_fp_id = ret.unwrap(); // safe
        
        let confirm_query = CustomerEvent::CargoRequest(
            CargoRequest::Confirm(
                FlightConfirm {
                    fp_id: draft_fp_id.clone()
                }
        ));

        self.log(&format!("Confirming draft ID {}...", &draft_fp_id));

        // Make request, get response
        let act = customer_events::action(&confirm_query).await;
        if act.is_err() {
            self.log("Failed to confirm.");
            return false;
        }
        let resp = act.unwrap(); // safe
        if resp.status() != StatusCode::OK {
            self.log(&format!("Bad Response: {}", resp.status()));
            return false;
        }

        // Make bytes from response
        let bytes = body::to_bytes(resp.into_body()).await;
        if bytes.is_err() {
            self.log("Failed to convert response into bytes.");
        }
        let bytes = bytes.unwrap(); // safe

        // Make String from bytes
        let fp_id = String::from_utf8(bytes.to_vec());
        if fp_id.is_err() {
            self.log("Failed to convert bytes to string.")
        }
        let fp_id = fp_id.unwrap(); //safe

        self.log(&format!("Confirmed, assigned plan {}.", fp_id));
        self.fp_id = fp_id;
        self.status = CustomerStatus::Cancel;
        true
    }

    async fn handle_cancel(&mut self) -> bool {
        // To add: Actual probability distribution
        let mut rng = rand::thread_rng();
        if !rng.gen_bool(self.behavior.cancel_chance().into()) {
            self.log("Chose not to cancel.");
            self.status = CustomerStatus::Done;
            return true;
        }
    
        let cancel_query = CustomerEvent::CargoRequest(
            CargoRequest::Cancel(
                FlightCancel {
                    fp_id: self.fp_id.clone()
                }
        ));

        self.log(&format!("Cancelling plan {}", &self.fp_id));
        let act = customer_events::action(&cancel_query).await;
        if let Err(e) = act {
            self.log(&format!("Could not cancel: {:?}", e));
            return false;
        }
        let resp = act.unwrap(); // safe

        if resp.status() != StatusCode::OK {
            self.log(&format!("Bad Response: {}", resp.status()));
            return false;
        }

        self.log("Cancel success!");

        self.status = CustomerStatus::Done;
        true
    }

    /// Prompts the customer to perform an action
    pub async fn next(&mut self) {
        let ret: bool = match self.status {
            CustomerStatus::Done => {
                true
            },
            CustomerStatus::Vertiports => {
                self.handle_vertiports().await
            },
            CustomerStatus::Query => {                
                self.handle_query().await
            },
            CustomerStatus::Confirm => {
                self.handle_confirm().await
            },
            CustomerStatus::Cancel => {
                self.handle_cancel().await
            }
        };

        if !ret {
            self.retries -= 1;
            if self.retries <= 0 {
                self.log("Customer reached max retries, dying.");
                self.status = CustomerStatus::Done;
            }
        }
    }
}
