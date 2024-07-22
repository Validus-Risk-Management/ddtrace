//! Trace and layer builders to export traces to the Datadog agent.
//! It also contains a convenience function to build a layer with the tracer.
pub use opentelemetry::trace::{TraceError, TraceResult};
use opentelemetry_datadog::new_pipeline;
use opentelemetry_sdk::trace::Tracer;
use tracing::Subscriber;
use tracing_opentelemetry::{OpenTelemetryLayer, PreSampledTracer};
use tracing_subscriber::registry::LookupSpan;

pub fn build_tracer(service_name: &str) -> TraceResult<Tracer> {
    new_pipeline()
        .with_service_name(service_name)
        .install_batch(opentelemetry_sdk::runtime::Tokio)
}

pub fn build_layer<S>(service_name: &str) -> TraceResult<OpenTelemetryLayer<S, Tracer>>
where
    Tracer: opentelemetry::trace::Tracer + PreSampledTracer + 'static,
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    let tracer = build_tracer(service_name)?;
    Ok(tracing_opentelemetry::layer().with_tracer(tracer))
}
