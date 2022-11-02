// use serde_yaml; // 0.8.23
use serde::{Deserialize, Serialize};
pub use svc_cargo_client_rest::types as cargo_client_types;

///////////////////////////////////////////////////////////////////////
/// CustomerRequest Events
///////////////////////////////////////////////////////////////////////

/// Events triggered by customer actions
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CustomerEvent {
    /// Request for a new cargo flight
    CargoRequest(CargoRequest),
    // Rideshare(RideshareRequest),
    // Charter(CharterRequest),
}

/// Customer requests for cargo flights
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CargoRequest {
    /// Create a cargo flight
    CargoCreate(cargo_client_types::FlightQuery),

    /// Confirm a cargo flight
    CargoConfirm(cargo_client_types::FlightConfirm),

    /// Cancel a cargo flight
    CargoCancel(cargo_client_types::FlightCancel),

    // Modify(cargo_client_types::ModifyQuery) // R2
}
