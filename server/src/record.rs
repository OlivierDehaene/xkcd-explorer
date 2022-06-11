//! Tracing utilities
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

impl<R> Record for Response<R>
where
    R: std::fmt::Debug,
{
    /// On Success: record response and logs
    fn record(self) -> Self {
        tracing::Span::current().record("response", &tracing::field::debug(self.get_ref()));
        tracing::info!("Success");
        self
    }
}

impl<R> Record for Result<R, Status>
where
    R: std::fmt::Debug,
{
    // Record errors
    fn record(self) -> Self {
        self.map_err(|err| {
            tracing::error!("{}", err);
            err
        })
    }
}
