use std::time::Duration;
use axum::extract::{MatchedPath,Request};
use axum::http::HeaderName;
use axum::response::Response;
use axum::Router;
use tower::ServiceBuilder;

use tower_http::request_id::{
MakeRequestId,
PropagateRequestIdLayer,
RequestId,
SetRequestIdLayer
};
use tower_http::trace::TraceLayer;
use tracing::Span;
use uuid::Uuid;

use crate::error::ErrorCode;


pub fn init_tracing(){
    let filter =tracing_subscriber::EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));
    tracing_subscriber::fmt()
    .with_env_filter(filter)
    .with_target(true)
    .init();
}

#[derive(Clone, Default)]
pub struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid{
    fn make_request_id<B>(&mut self,_request:&Request<B>)->Option<RequestId>{
    let id =Uuid::now_v7().to_string();
    axum::http::HeaderValue::from_str(&id)
    .ok()
    .map(RequestId::new)

    

    }     
}

pub fn current_request_id(raw:&RequestId)->Uuid{
    raw.header_value()
    .to_str()
    .ok()
    .and_then(|s|Uuid::parse_str(s).ok())
    .unwrap_or_else(Uuid::now_v7)
}

pub fn with_request_tracing(router:Router)->Router{
    let request_id_header=HeaderName::from_static("x-request-id");
    router.layer(
      ServiceBuilder::new()
      .layer(SetRequestIdLayer::new(request_id_header.clone(),MakeRequestUuid))
      .layer(
        TraceLayer::new_for_http()
        .make_span_with(make_span)
        .on_response(on_response),
      )
      .layer(PropagateRequestIdLayer::new(request_id_header))

    )   
}


fn make_span(request:&Request)->Span{
   let route=request
   .extensions()
   .get::<MatchedPath>()
   .map(MatchedPath::as_str)
   .unwrap_or_else(|| request.uri().path());
 let request_id=request
   .extensions()
   .get::<RequestId>()
   .map(current_request_id)
   .unwrap_or_else(Uuid::now_v7);
  
  tracing::info_span!(
    "http_request",
    request_id=%request_id,
    method=%request.method(),
    route= %route,
    status=tracing::field::Empty,
    latency_ms=tracing::field::Empty,
    error_code=tracing::field::Empty,
  )
}

fn on_response(response: &Response,latency:Duration,span:&Span){
let status=response.status().as_u16();
let latency_ms=latency.as_secs_f64()*1000.0;
span.record("status",status);
span.record("latency_ms",latency_ms);

 match response.extensions().get::<ErrorCode>() {
        Some(code) => {
            span.record("error_code", code.0);
            tracing::warn!(status, latency_ms, error_code = code.0, "request completed");
        }
        None => {
            tracing::info!(status, latency_ms, "request completed");
        }
    }


}


