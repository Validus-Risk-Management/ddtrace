//! Trace and layer builders to export traces to the Datadog agent.
//!
//! This module contains a function that builds a tracer with an exporter
//! to send traces to the Datadog agent in batches over gRPC.
//!
//! It also contains a convenience function to build a layer with the tracer.
use crate::error::Error;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler, SdkTracerProvider, Tracer};
use opentelemetry_sdk::Resource;
use std::time::Duration;
use tracing::Subscriber;
use tracing_opentelemetry::{OpenTelemetryLayer, PreSampledTracer};
use tracing_subscriber::registry::LookupSpan;

pub struct ProviderGuard {
    tracer_provider: SdkTracerProvider,
}

impl Drop for ProviderGuard {
    fn drop(&mut self) {
        let _ = self.tracer_provider.force_flush();
        let _ = self.tracer_provider.shutdown();
    }
}

pub fn build_tracer_provider(service_name: String) -> Result<SdkTracerProvider, Error> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_timeout(Duration::from_secs(3))
        .build()?;
    let resource = Resource::builder().with_service_name(service_name).build();

    let provider = SdkTracerProvider::builder()
        .with_sampler(Sampler::AlwaysOn)
        .with_resource(resource)
        .with_id_generator(RandomIdGenerator::default())
        .with_batch_exporter(exporter)
        .build();

    Ok(provider)
}

pub fn build_layer<S>(
    service_name: String,
) -> Result<(OpenTelemetryLayer<S, Tracer>, ProviderGuard), Error>
where
    Tracer: opentelemetry::trace::Tracer + PreSampledTracer + 'static,
    S: Subscriber + for<'span> LookupSpan<'span>,
{
    let tracer_provider = build_tracer_provider(service_name)?;
    let layer = tracing_opentelemetry::layer().with_tracer(tracer_provider.tracer(""));
    let guard = ProviderGuard { tracer_provider };
    Ok((layer, guard))
}
