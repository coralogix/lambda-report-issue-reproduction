extern crate core;

use lambda_extension::service_fn;
use lambda_extension::{Extension, LambdaEvent, LambdaTelemetry, LogBuffering, SharedService};
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Error> {
    configure_tracing();

    info!("Starting extension");

    let lambda_extension = Extension::new()
        .with_events_processor(service_fn(handle_lambda_event))
        .with_telemetry_buffering(LogBuffering {
            timeout_ms: 25,
            max_bytes: 262144,
            max_items: 1000,
        })
        .with_telemetry_port_number(12345)
        .with_telemetry_processor(SharedService::new(service_fn(handle_telemetry)));

    lambda_extension.run().await?;

    info!("Stopping extension");

    Ok(())
}

fn configure_tracing() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_ansi(false) // These logs will be ingested by CloudWatch => no ANSI support
        .with_target(false) // we disable the default target which is the full module name and instead write a short "coralogix: "
        .without_time() // CloudWatch provides it's own timestamp
        .init();
}

async fn handle_lambda_event(event: LambdaEvent) -> Result<(), Error> {
    info!("Handling lambda event {:?}", event.next);
    Ok(())
}

async fn handle_telemetry(telemetry: Vec<LambdaTelemetry>) -> Result<(), Error> {
    info!("Handling telemetry event: {:?}", telemetry);
    Ok(())
}
