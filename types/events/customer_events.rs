// use serde_yaml; // 0.8.23
use serde::{Deserialize, Serialize};
use hyper::{Body, Client, Method, Request, Response};
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
    /// Request list of vertiports
    Vertiports(cargo_client_types::VertiportsQuery),

    /// Create a cargo flight
    Query(cargo_client_types::FlightQuery),

    /// Confirm a cargo flight
    Confirm(cargo_client_types::FlightConfirm),

    /// Cancel a cargo flight
    Cancel(cargo_client_types::FlightCancel),

    // Modify(cargo_client_types::ModifyQuery) // R2
}

///////////////////////////////////////////////////////////////////////
/// Actions on Customer Events
///////////////////////////////////////////////////////////////////////
fn get_server_addr() -> String {
    "http://0.0.0.0:8000".into()
}

async fn cargo(event: &CargoRequest) -> Result<Response<Body>, ()> {
    let url = format!("{}/cargo", get_server_addr());
    let client = Client::builder()
        .pool_idle_timeout(std::time::Duration::from_secs(10))
        .build_http();

    match event {
        CargoRequest::Vertiports(s) => {
            let endpoint = format!("{url}/vertiports");
            let req = Request::builder()
                .method(Method::POST)
                .uri(endpoint)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&s).unwrap()))
                .unwrap();
            
            match client.request(req).await {
                Ok(r) => Ok(r),
                Err(e) => {
                    println!("Failed to request: {:?}", e);
                    Err(())
                }
            }
        }
        CargoRequest::Query(s) => {
            let endpoint = format!("{url}/query");
            let req = Request::builder()
                .method(Method::POST)
                .uri(endpoint)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&s).unwrap()))
                .unwrap();
            
            match client.request(req).await {
                Ok(r) => Ok(r),
                Err(e) => {
                    println!("Failed to request: {:?}", e);
                    Err(())
                }
            }
        }
        CargoRequest::Cancel(s) => {
            let endpoint = format!("{url}/cancel");
            let req = Request::builder()
                .method(Method::DELETE)
                .uri(endpoint)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&s).unwrap()))
                .unwrap();

            match client.request(req).await {
                Ok(r) => Ok(r),
                _ => Err(())
            }
        }
        CargoRequest::Confirm(s) => {
            let endpoint = format!("{url}/confirm");
            let req = Request::builder()
                .method(Method::PUT)
                .uri(endpoint)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&s).unwrap()))
                .unwrap();

            match client.request(req).await {
                Ok(r) => Ok(r),
                _ => Err(())
            }
        }
    }
}

/// Mimics a REST message from an external client
/// # Arguments
/// * event - The type of event to mimic
/// # Returns
/// Result of the REST request
pub async fn action(event: &CustomerEvent) -> Result<Response<Body>, ()> {
    match event {
        CustomerEvent::CargoRequest(s) => cargo(s).await,
    }
}
