// use serde_yaml; // 0.8.23
use serde::{Deserialize, Serialize};
use hyper::{Body, Client, Method, Request, Response};
use hyper::client::HttpConnector;
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

async fn req_helper(
            client: &Client<HttpConnector>,
            endpoint: String,
            msg: String,
            method: Method
        ) -> Result<Response<Body>, ()> {
    let req = Request::builder()
        .method(method)
        .uri(endpoint)
        .header("content-type", "application/json")
        .body(Body::from(msg));
    
    match req {
        Err(_) => {
            eprintln!("ERROR: Unable to form request.");
            Err(())
        },
        Ok(req) => {
            match client.request(req).await {
                Ok(r) => Ok(r),
                Err(e) => {
                    eprintln!("ERROR: Failed to request: {:?}", e);
                    Err(())
                }
            }
        }
    }
}

async fn cargo(event: &CargoRequest) -> Result<Response<Body>, ()> {
    let url = format!("{}/cargo", get_server_addr());
    let client = Client::builder()
        .pool_idle_timeout(std::time::Duration::from_secs(10))
        .build_http();

    match event {
        CargoRequest::Vertiports(s) => {
            let endpoint = format!("{url}/vertiports");
            match serde_json::to_string(&s) {
                Err(_) => {
                    eprintln!("ERROR: Failed to serialize to string: {:?}.", s);
                    Err(())
                },
                Ok(msg) => {
                    req_helper(&client, endpoint, msg, Method::POST).await
                }
            }
        }
        CargoRequest::Query(s) => {
            let endpoint = format!("{url}/query");
            match serde_json::to_string(&s) {
                Err(_) => {
                    eprintln!("ERROR: Failed to serialize to string: {:?}.", s);
                    Err(())
                },
                Ok(msg) => {
                    req_helper(&client, endpoint, msg, Method::POST).await
                }
            }
        }
        CargoRequest::Cancel(s) => {
            let endpoint = format!("{url}/cancel");
            match serde_json::to_string(&s) {
                Err(_) => {
                    eprintln!("ERROR: Failed to serialize to string: {:?}.", s);
                    Err(())
                },
                Ok(msg) => {
                    req_helper(&client, endpoint, msg, Method::DELETE).await
                }
            }
        }
        CargoRequest::Confirm(s) => {
            let endpoint = format!("{url}/confirm");
            match serde_json::to_string(&s) {
                Err(_) => {
                    eprintln!("ERROR: Failed to serialize to string: {:?}.", s);
                    Err(())
                },
                Ok(msg) => {
                    req_helper(&client, endpoint, msg, Method::PUT).await
                }
            }
        }
    }
}

/// Mimics a REST message from an external client
pub async fn action(event: &CustomerEvent) -> Result<Response<Body>, ()> {
    match event {
        CustomerEvent::CargoRequest(s) => cargo(s).await,
    }
}
