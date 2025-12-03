use tracing_subscriber::EnvFilter;
use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::Response,
};
use tracing::info;
use http_body_util::BodyExt;


pub fn init_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("info".parse().unwrap())
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .with_ansi(true)
        .compact()
        .init();
}



pub async fn log_request_response(
    req: Request,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    let (parts, body) = req.into_parts();
    let body_bytes = body.collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8_lossy(&body_bytes);
    
    info!("→ {} {} | Body: {}", method, uri, body_str);
    
    let req = Request::from_parts(parts.clone(), Body::from(body_bytes));
    let response = next.run(req).await;
    
    let status = response.status();
    let (res_parts, res_body) = response.into_parts();
    let res_body_bytes = res_body.collect().await.unwrap().to_bytes();
    
    info!("← {} {} | Status: {}", method, uri, status);
    
    Response::from_parts(res_parts, Body::from(res_body_bytes))
}
