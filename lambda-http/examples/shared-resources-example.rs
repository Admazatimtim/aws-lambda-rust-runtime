use lambda_http::{service_fn, Error, IntoResponse, Request, RequestExt, Response};

struct SharedClient {
    name: &'static str,
}

impl SharedClient {
    fn response(&self, req_id: String, first_name: &str) -> String {
        format!("{}: Client ({}) invoked by {}.", req_id, self.name, first_name)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create the "client" and a reference to it, so that we can pass this into the handler closure below.
    let shared_client = SharedClient {
        name: "random_client_name_1",
    };
    let shared_client_ref = &shared_client;

    // Define a closure here that makes use of the shared client.
    let handler_func_closure = move |event: Request| async move {
        Ok(match event.query_string_parameters().first("first_name") {
            Some(first_name) => shared_client_ref
                .response(event.lambda_context().request_id, first_name)
                .into_response(),
            _ => Response::builder()
                .status(400)
                .body("Empty first name".into())
                .expect("failed to render response"),
        })
    };

    // Pass the closure to the runtime here.
    lambda_http::run(service_fn(handler_func_closure)).await?;
    Ok(())
}
