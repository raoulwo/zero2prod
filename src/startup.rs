use crate::routes;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    // `web::Data` is used to wrap `pool` in an `Arc`
    // (atomic reference-counter pointer). We need to
    // do so because `pool` can't be shared across
    // threads. Instead we *move* a **clone** of the
    // pointer to the worker.
    let pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            // The crate `tracing-actix-web` is a stand-in replacement for
            // the default actix-web logger which is based on `tracing` instead
            // of `log`.
            .wrap(TracingLogger::default()) // Middlewares are added using `wrap`
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(pool.clone())
        // `app_data` can be used to register information as
        // part of the application state.
    })
    .listen(listener)?
    .run();

    Ok(server)
}
