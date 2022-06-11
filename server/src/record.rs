//! Tracing utilities
// use crate::errors::FaceRetrievalError;
use tonic::{Request, Response, Status};

pub trait Record {
    fn record(self) -> Self;
}

impl<R> Record for Request<R>
where
    R: std::fmt::Debug,
{
    /// On Request: record request in active span
    fn record(self) -> Self {
        // Get current span
        let span = tracing::Span::current();
        span.record("request", &tracing::field::debug(self.get_ref()));
        tracing::info!("Received request");
        self
    }
}

impl<R> Record for Result<R, Status>
where
    R: std::fmt::Debug,
{
    // Record response and logs
    fn record(self) -> Self {
        match &self {
            Ok(response) => {
                tracing::Span::current().record("response", &tracing::field::debug(response));
                tracing::info!("Success");
            }
            Err(err) => {
                tracing::error!("{}", err);
            }
        }
        self
    }
}
