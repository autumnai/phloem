//! Phloem provides a Blob - a wrapper for universal/numeric data, that can be synchronized between
//! the CPU and the GPU. Used for machine learning purposes at the Rust Deep Learning Framework
//! [Leaf][leaf].
//! [leaf]: https://github.com/autumnai/leaf
//!
//! ## Overview
//!
//! Mathematically speaking, a blob is an N-dimensional array stored in a C-contiguous fashion.
//! Ploems' Blob uses [Arrayfire][arrayfire] as the backend abstraction, which currently restricts
//! the Blob to be a 4-dimensional array at most. Time will show if this is a severe
//! limitation.<br/>
//! The advantages of Phloems' Blob is a unified memory interface for data that hides the
//! computational and mental overhead of mixed CPU/GPU operations by synchronizing from the host
//! (CPU) to the device (GPU) as needed.
//!
//! As Phloems' Blob was developed for use in machine learning scenarios, the Blob is opinionated
//! about its application. For more information about how exactly, see the Blob module.
//!
//! [arrayfire]: https://github.com/arrayfire/arrayfire
#![feature(plugin)]
#![plugin(clippy)]
#![allow(dead_code)]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code, unused_import_braces, unused_qualifications)]

pub use blob::Blob;
pub use numeric::Numeric;

pub mod blob;
pub mod numeric;
