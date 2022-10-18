// use serde_yaml; // 0.8.23
use serde::{Deserialize, Serialize};
pub use svc_cargo_client_rest::types as cargo_client_types;

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
