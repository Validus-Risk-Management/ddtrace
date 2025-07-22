//! Tonic utilities.
//!
//! This module re-exposes the middleware layers provided by the
//! [`tonic-tracing-opentelemetry`] project.
//!
//! [`tonic-tracing-opentelemetry`]: https://github.com/davidB/tracing-opentelemetry-instrumentation-sdk

pub use tonic_tracing_opentelemetry::middleware::server::{Filter, OtelGrpcLayer};
