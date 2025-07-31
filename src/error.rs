#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ExporterBuildError(#[from] opentelemetry_otlp::ExporterBuildError),
}
