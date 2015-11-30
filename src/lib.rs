//! Phloem provides a Blob - a wrapper for universal/numeric data, that can be synchronized between
//! the CPU and the GPU. Used for machine learning purposes at the Rust Deep Learning Framework
//! [Leaf][leaf].
//! [leaf]: https://github.com/autumnai/leaf
//!
//! ## Overview
//!
//! Mathematically speaking, a blob is an N-dimensional array stored in a C-contiguous fashion.
//! Ploems' Blob uses [Collenchyma][collenchyma] as the backend abstraction.
//! The advantages of Phloems' Blob is a unified memory interface for data that hides the
//! computational and mental overhead of mixed CPU/GPU operations by synchronizing from the host
//! (CPU) to the device (GPU) as needed.
//!
//! As Phloems' Blob was developed for use in machine learning scenarios, the Blob is opinionated
//! about its application. For more information about how exactly, see the Blob module.
//!
//! [collenchyma] https://github.com/autumnai/collenchyma
#![cfg_attr(lint, feature(plugin))]
#![cfg_attr(lint, plugin(clippy))]
#![feature(clone_from_slice)]
#![allow(dead_code)]
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code, unused_import_braces, unused_qualifications)]

extern crate collenchyma as co;

pub use blob::Blob;

pub mod blob;
