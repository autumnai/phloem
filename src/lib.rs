#![feature(plugin)]
#![plugin(clippy)]
#![allow(dead_code)]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code, unused_import_braces, unused_qualifications)]

//! Phloem exposes a universal/numeric Blob for machine learning purposes.
//! This might be used with the Rust Machine Learning Framework
//! [leaf](https://github.com/autumnai/leaf)
pub use blob::Blob;

pub mod blob;
