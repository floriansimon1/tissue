mod lax_bool;

pub mod phase;
pub mod lazy_value;

pub use lax_bool::parse_lax_bool;

#[cfg(test)] mod phase_tests;
#[cfg(test)] mod lazy_value_tests;
