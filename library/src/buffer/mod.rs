pub mod from_buffer;
pub mod into_buffer;

// Re-exports to ease the use of those traits.
pub use from_buffer::FromBuffer;
pub use into_buffer::IntoBuffer;