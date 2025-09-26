pub mod v1;

pub use v1::landing;
pub use v1::landing::events;
pub use v1::landing::testimonials;
pub use v1::landing::events::events_public_routes;
pub use v1::landing::events::events_protected_routes;
pub use v1::landing::testimonials::testimonials_public_routes;
pub use v1::landing::testimonials::testimonials_protected_routes;
