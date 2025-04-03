use std::net::SocketAddr;

use axum::{Router, routing::get};
use directory::{
    app_state::AppState,
    config::Config,
    routes::{health::ping_pong, school_directory_r::school_data, teachers_r::teacher_data},
};
use tower_http::{
    cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration from Env.
    let app_config = envy::from_env::<Config>()?;

    let app_state = AppState::build(&app_config).await?;

    // TODO: #6 Create background tasks to cache data.

    let app = Router::new()
        // Health
        .route("/health/ping", get(ping_pong))
        .route("/teacher/(id)", get(teacher_data))
        .route("/school/(id)", get(school_data))
        // CORS
        .layer(
            CorsLayer::new()
                .allow_headers(AllowHeaders::any())
                .allow_origin(AllowOrigin::any())
                .allow_methods(AllowMethods::any()),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::debug!(
        "Listening on address: {}",
        listener
            .local_addr()
            .expect("Unable to print listening address")
    );

    axum::serve(listener, app).await?;
    Ok(())
}
