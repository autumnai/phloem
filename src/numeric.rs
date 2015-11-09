//! Provides the trait for a valid [Blob] input
//! [blob]: ../blob/index.html
//!
//! Needs to be implemented by all (primitive) types that are allowed for Blobs' data and diff
//! value. Supported by default are the `f32` and the `f64` type.

/// Provides the trait for a valid [Blob] input
/// [blob]: ../blob/index.html
///
/// Needs to be implemented by all (primitive) types that are allowed for Blobs' data and diff
/// value.
pub trait Numeric {}

impl Numeric for f32 {}
impl Numeric for f64 {}
