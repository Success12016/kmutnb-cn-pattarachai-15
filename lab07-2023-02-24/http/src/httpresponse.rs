use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}
use http::{Request, Response, StatusCode};

fn not_found(_req: Request<()>) -> http::Result<Response<()>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(())
}