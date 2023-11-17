use crate::calc::handle_calculate;
use rsimple_http::http::request::Method;
use rsimple_http::http::response::Response;
use rsimple_http::http::response::ResponseCode;
use rsimple_http::http::start_server;

mod calc;

fn main() {
    let address = "127.0.0.1:7878";

    println!("Starting on {address}");
    // TODO handle startup and listening errors
    let _ = start_server(address, |req| match req.path.as_str() {
        "/" => Response::ok("Hello World!".to_string()),
        "/calc" => match req.method {
            Method::POST => match handle_calculate(&req.body) {
                Ok(result) => Response::ok(format!("{}", result).to_string()),
                Err(e) => Response::with_code(
                    ResponseCode::InternalServerError,
                    format!("Error: {}", e).to_string(),
                ),
            },
            _ => Response::with_code(
                ResponseCode::MethodNotSupported,
                "method not supported".to_string(),
            ),
        },
        _ => Response::with_code(ResponseCode::NotFound, "not found".to_string()),
    });
}
