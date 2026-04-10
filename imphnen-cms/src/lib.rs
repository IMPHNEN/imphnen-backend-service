pub mod events;
pub mod roadmap;
pub mod testimonials;
pub mod qr;

pub use events::{events_protected_routes, events_public_routes};
pub use roadmap::{roadmap_protected_routes, roadmap_public_routes};
pub use testimonials::{testimonials_protected_routes, testimonials_public_routes};
pub use qr::qr_router;
