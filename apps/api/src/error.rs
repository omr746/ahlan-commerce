
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use rootcause::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use catalog::CatalogError;

#[derive(Debug,thiserror::Error)]
enum AppErrorKind{
  #[error("{0}")]
  Validation(String),
  #[error("product handle '{0}' is already in use ")]
  DuplicateProductHandle(String),
  #[error("{0} was not found")]
    NotFound(String),
  #[error("a required dependency is unavailable")]
  DependencyUnavailable,
  #[error("the server failed unexpectedly")]
  Internal,

}

#[derive(Debug)]
pub struct AppError{
 kind:AppErrorKind,
 request_id:Uuid,
 cause:Option<Report>,   
}


impl AppError{
pub fn validation(message: impl Into<String>, request_id: Uuid) -> Self {
        Self {
            kind: AppErrorKind::Validation(message.into()),
            request_id,
            cause: None,
        }
    }
    pub fn duplicate_handle(message: impl Into<String>, request_id: Uuid) -> Self {
        Self {
            kind: AppErrorKind::DuplicateProductHandle(message.into()),
            request_id,
            cause: None,
        }
    }
     pub fn not_found(message: impl Into<String>, request_id: Uuid) -> Self {
        Self {
            kind: AppErrorKind::NotFound(message.into()),
            request_id,
            cause: None,
        }
    }
    pub fn dependency_unavailable(request_id: Uuid, cause: Report) -> Self {
        Self {
            kind: AppErrorKind::DependencyUnavailable,
            request_id,
            cause: Some(cause),
        }
    }

    pub fn internal(request_id: Uuid, cause: Report) -> Self {
        Self {
            kind: AppErrorKind::Internal,
            request_id,
            cause: Some(cause),
        }
    }
    pub fn from_catalog(err:CatalogError,request_id:Uuid)->Self{
      match err{
            CatalogError::DuplicateHandle(handle) => Self::duplicate_handle(handle, request_id),
            CatalogError::NotFound(id) => Self::not_found(format!("product {id}"), request_id),
      }
    }
       pub fn code(&self) -> &'static str {
        match self.kind {
            AppErrorKind::Validation(_) => "validation_failed",
            AppErrorKind::DuplicateProductHandle(_) => "duplicate_product_handle",
            AppErrorKind::NotFound(_) => "not_found",
            AppErrorKind::DependencyUnavailable => "dependency_unavailable",
            AppErrorKind::Internal => "internal_error",
        }
    }
      pub fn status(&self) -> StatusCode {
        match self.kind {
            AppErrorKind::Validation(_) => StatusCode::BAD_REQUEST,
            AppErrorKind::DuplicateProductHandle(_) => StatusCode::CONFLICT,
            AppErrorKind::NotFound(_) => StatusCode::NOT_FOUND,
            AppErrorKind::DependencyUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            AppErrorKind::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
     pub fn public_message(&self) -> String {
        self.kind.to_string()
    }

    pub fn request_id(&self) -> Uuid {
        self.request_id
    }
    pub fn log(&self){
      if let Some(cause)= &self.cause{
         eprintln!(
                "level=error request_id={} code={} public_message={:?} root_cause=\"{}\"",
                self.request_id,
                self.code(),
                self.public_message(),
                cause,
            );
      }
    }

 pub fn body(&self) -> ErrorEnvelope {
        ErrorEnvelope {
            error: ErrorBody {
                code: self.code().to_string(),
                message: self.public_message(),
                request_id: self.request_id,
            },
        }
    }


}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    pub request_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ErrorEnvelope {
    pub error: ErrorBody,
}

impl IntoResponse for AppError{
    fn into_response(self)->Response{
        self.log();
        (self.status(),Json(self.body())).into_response()
    }
}



