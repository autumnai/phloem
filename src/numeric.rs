/// The Numeric Trait
///
/// Gets implemented by all primitive types that are allowed for `Blob`s' data Vec value.
pub trait Numeric {}

impl Numeric for f32 {}
impl Numeric for f64 {}
