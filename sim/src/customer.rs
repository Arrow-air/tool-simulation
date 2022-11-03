use sim_types::eel_types::{CargoRequest, CustomerEvent};

use hyper::{Body, Client, Error, Method, Request, Response};

fn get_server_addr() -> String {
    "http://0.0.0.0:8000".into()
}

async fn cargo(event: &CargoRequest) -> Result<Response<Body>, Error> {
    let url = format!("{}/cargo", get_server_addr());
    let client = Client::new();

    match event {
        CargoRequest::CargoCreate(s) => {
            let endpoint = format!("{url}/create");
            let req = Request::builder()
                .method(Method::POST)
                .uri(endpoint)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&s).unwrap()))
                .unwrap();

            client.request(req).await
        }
        CargoRequest::CargoCancel(s) => {
            let endpoint = format!("{url}/cancel");
            let req = Request::builder()
                .method(Method::DELETE)
                .uri(endpoint)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&s).unwrap()))
                .unwrap();

            client.request(req).await
        }
        CargoRequest::CargoConfirm(s) => {
            let endpoint = format!("{url}/confirm");
            let req = Request::builder()
                .method(Method::POST)
                .uri(endpoint)
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&s).unwrap()))
                .unwrap();

            client.request(req).await
        }
    }
}

pub async fn action(event: &CustomerEvent) -> Result<Response<Body>, Error> {
    match event {
        CustomerEvent::CargoRequest(s) => cargo(s).await,
    }
}
