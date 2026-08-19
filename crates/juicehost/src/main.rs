use sentry::types::Dsn;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;

use config::Config;

mod config;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new().with(sentry_tracing::layer());
    tracing::subscriber::set_global_default(subscriber)?;

    let config = Config::try_load()?;
    setup_sentry(config.sentry());

    Ok(())
}

#[tracing::instrument(skip_all)]
fn setup_sentry(config: &config::sentry::SentryConfig) {
    if let Some(dsn) = config.dsn().filter(|dsn| !dsn.is_empty()) {
        // a guard to prevent panicing when constructing client options
        //
        // I wonder if there's a way for serde to verify this stuff?
        if let Err(err) = dsn.parse::<Dsn>() {
            warn!("Failed to parse Dsn from config; skipping sentry initialization: {err}");
            return;
        }

        let client_options = sentry::ClientOptions::default()
            .dsn(dsn)
            .maybe_release(sentry::release_name!())
            .environment(config.environment().to_owned())
            .traces_sample_rate(config.trace_sample_rate())
            .send_default_pii(false);

        // forget is used here to make the sentry client guard static
        std::mem::forget(sentry::init(client_options));
    } else {
        info!("No sentry dsn provided; skipping sentry initialization");
    }
}
