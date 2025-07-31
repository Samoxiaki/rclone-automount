use std::path::{PathBuf};
use std::env;
use tokio::signal::unix::{signal, SignalKind};

pub fn expand_home(path: &str) -> String {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(stripped).to_string_lossy().into_owned();
        }
    }
    path.to_string()
}



pub async fn wait_for_shutdown() {
    let mut sigterm = signal(SignalKind::terminate()).expect("Unable to listen for SIGTERM");
    let mut sigint = signal(SignalKind::interrupt()).expect("Unable to listen for SIGINT");
    let mut sigquit = signal(SignalKind::quit()).expect("Unable to listen for SIGQUIT");

    tokio::select! {
        _ = sigterm.recv() => {
            tracing::info!("Received SIGTERM, shutting down...");
        },
        _ = sigint.recv() => {
            tracing::info!("Received SIGINT (Ctrl+C), shutting down...");
        },
        _ = sigquit.recv() => {
			tracing::info!("Received SIGQUIT, shutting down...");
        },
    }
}
