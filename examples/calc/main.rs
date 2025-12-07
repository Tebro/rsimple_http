use crate::calc::handle_calculate;
use http::Method;
use http::Response;
use http::StatusCode;
use rsimple_http::http::start_server;

mod calc;

fn main() {
    let address = "127.0.0.1:7878";

    println!("Starting on {address}");
    // TODO handle startup and listening errors
    let _ = start_server(address, |req| match req.path.as_str() {
        "/" => Response::builder()
            .status(StatusCode::OK)
            .body("Hello world".to_string())
            .unwrap(),
        "/calc" => match req.method {
            Method::POST => match handle_calculate(&req.body) {
                Ok(result) => Response::builder()
                    .status(StatusCode::OK)
                    .body(format!("{}", result).to_string())
                    .unwrap(),
                Err(e) => Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("Error: {}", e).to_string())
                    .unwrap(),
            },
            _ => Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body("".to_string())
                .unwrap(),
        },
        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("".to_string())
            .unwrap(),
    });
}
