//! Axum web host: serves the built frontend from `dist/` and mirrors every
//! AppCore command under `/api`. Single-user, localhost by default.

mod api;
mod error;
mod files;
mod sse;

use std::{path::PathBuf, sync::Arc};

use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use yaya_app_core::{AppCore, AppPaths};

#[derive(Clone)]
pub(crate) struct WebState {
    pub core: AppCore,
    /// Browser downloads land in isolated per-request dirs under here and
    /// are deleted after being streamed out.
    pub web_output_dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = std::env::var_os("YAYA_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            dirs::data_local_dir()
                .unwrap_or_else(std::env::temp_dir)
                .join("YAYA")
        });
    let web_output_dir = data_dir.join("web-downloads");
    tokio::fs::create_dir_all(&web_output_dir).await?;

    let core = AppCore::open(
        AppPaths {
            data_dir: data_dir.clone(),
            default_output_dir: web_output_dir.clone(),
        },
        yaya_provider_bundle::providers(),
    )
    .await?;
    let state = Arc::new(WebState {
        core,
        web_output_dir,
    });

    let app = axum::Router::new()
        .nest("/api", api::router())
        .fallback_service(ServeDir::new("dist").fallback(ServeFile::new("dist/index.html")))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let address = std::env::var("YAYA_WEB_ADDR").unwrap_or_else(|_| "127.0.0.1:9527".into());
    let listener = tokio::net::TcpListener::bind(&address).await?;
    println!("YAYA Web: http://{address}");
    axum::serve(listener, app).await?;
    Ok(())
}
