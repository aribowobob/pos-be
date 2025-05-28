pub mod auth;
pub mod health;
pub mod migration;
pub mod orders;
pub mod products;
pub mod user;

// Re-export all route configuration functions
pub use auth::configure as configure_auth;
pub use health::configure as configure_health;
pub use migration::configure as configure_migration;
pub use orders::configure as configure_orders;
pub use products::configure as configure_products;
pub use user::configure as configure_user;

use actix_web::web;

// Function to configure all routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Health endpoint outside of API scope (no authentication needed)
    cfg.configure(configure_health);
    cfg.configure(configure_auth);
    cfg.configure(configure_migration);

    // All other endpoints under API scope
    cfg.service(
        web::scope("/api")
            .configure(configure_products)
            .configure(configure_orders)
            .configure(configure_user),
    );

    // Configure user routes
    user::configure(cfg);
}
