#[cfg(feature = "macros")]
pub mod macros;

#[cfg(feature = "string")]
pub mod string;

#[cfg(feature = "string")]
pub use string::CharAt;

#[cfg(feature = "macros")]
pub use crate::ternary as tern;
