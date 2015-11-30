//! Provides the backend-agnostic data container.
//!
//! This is the main module of Phloem and probably the reason, why you are using or interested in
//! this library.
//!
//! ## Overview
//!
//! The [main overview][overview] of the library gives a overview, to which I will just link here.
//! [overview]: ../index.html
//!
//! ## Opinions
//!
//! As Phloems' Blob is hihgly opinionated for the use in a machine learning scenario, it has some
//! restrictions and designs decisions. The following two are the most important.
//!
//! * __diff__:<br/>
//! Beside the fields data and shape, the Blob also has a field diff, which stores the gradients
//! that the network might compute over the data stored in the data field.
//!
//! * __Numeric__:<br/>
//! As only numeric values can be processed by a Network, the type of the data that the Blob can
//! store is restricted to the type Float, which is implemented for `f32` and `f64` Append
//! reexported from the `num` crate.
use co::shared_memory::SharedMemory;
use co::framework::IFramework;
use co::device::{IDevice, DeviceType};
use co::backend::{Backend, BackendConfig};
use co::frameworks::native::*;
use co::libraries::blas::*;
use co::libraries::numeric_helpers::{Float, cast};

#[derive(Debug)]
/// Backend-agnostic data container.
pub struct Blob<T: Float> {
    data: SharedMemory<T>,
    diff: SharedMemory<T>,
    shape: Vec<usize>,
}

impl <T> Blob<T> where T: Float {
    /// Creates a new Blob in host Memory.
    pub fn new() -> Blob<T> {
        Blob::of_shape(None, &[0])
    }

    /// Creates a new Blob with specified `shape` on `device`.
    ///
    /// If `device` is `None`, Blob will be created in host Memory.
    pub fn of_shape(device: Option<&DeviceType>, shape: &[usize]) -> Blob<T> {
        let ntv = Native::new();
        let cpu_device = ntv.new_device(ntv.hardwares()).unwrap();
        let dev: &DeviceType;
        match device {
            Some(provided_device) => dev = provided_device,
            None => { dev = &cpu_device }
        }

        let capacity = Blob::<T>::shape_capacity(shape);
        Blob {
            data: SharedMemory::new(dev, capacity),
            diff: SharedMemory::new(dev, capacity),
            shape: shape.to_vec(),
        }
    }

    /// Reshapes the Blob to the new `shape` given.
    ///
    /// Results in recreation of the Blob if the capacity
    /// of the new shape doesn't match the one of the old shape.</br>
    /// **This destroys all data inside the blob**
    ///
    /// If the capacity of the old and new shape match, no data will be destroyed.
    pub fn reshape(&mut self, new_shape: &[usize]) {
        let new_capacity = Self::shape_capacity(new_shape);
        if new_capacity != self.data.capacity() {
            *self = Self::of_shape(None, new_shape);
        }
    }

    /// Helper to get total capacity of shape.
    fn shape_capacity(shape: &[usize]) -> usize {
        let mut capacity = 1;
        for dimension in shape {
            capacity *= *dimension;
        }
        capacity
    }

    /// Returns the shape of the Blob.
    pub fn shape(&self) -> Vec<usize> {
        self.shape.clone()
    }

    /// Returns a String representation of the Blobs' `shape`.
    ///
    /// The first numbers represent the size of the dimension.
    /// The last number in brackets defines the dimensionality of the Blob.
    pub fn shape_string(&self) -> String {
        let mut string: String = "".to_owned();
        for dim in self.shape.clone() {
            string.push_str(&format!("{} ", &dim.to_string()));
        }
        string.push_str(&format!("({})", self.shape.len().to_string()));
        string
    }

    /// Returns the allocated capacity of the Blob.
    pub fn capacity(&self) -> usize {
        Self::shape_capacity(&self.shape)
    }

    /// Returns a reference to the data of the Blob.
    pub fn data(&self) -> &SharedMemory<T> {
        &self.data
    }

    /// Returns a mutable reference to the data of the Blob.
    pub fn mut_data(&mut self) -> &mut SharedMemory<T> {
        &mut self.data
    }

    /// Returns a reference to the diff of the Blob.
    pub fn diff(&self) -> &SharedMemory<T> {
        &self.diff
    }

    /// Returns a mutable reference to the diff of the Blob.
    pub fn mut_diff(&mut self) -> &mut SharedMemory<T> {
        &mut self.diff
    }

    /// Apply the diff to the data.
    ///
    /// In machine learnig this is used when the blob represents weights in a network,
    /// and the comuted gradients should be applied to weights.
    ///
    /// If no `backend` is provided, comutations are executed on the host.
    pub fn apply_diff(&mut self) {
        let framework = Native::new();
        let hardwares = framework.hardwares();
        let backend_config = BackendConfig::new(framework, hardwares);
        let backend = Backend::new(backend_config).unwrap();

        let shared_a = &mut SharedMemory::<T>::new(backend.device(), 1);
        let local_a = [cast::<f32, T>(-1f32).unwrap()];
        let a = shared_a.get_mut(backend.device()).unwrap().as_mut_native().unwrap();
        a.as_mut_slice().clone_from_slice(&local_a);

        unimplemented!();
        // backend.axpy(shared_a, self.mut_diff(), self.mut_data());

        // backend.axpy(
        // leaf_cpu_axpy(&-1f32,
        //               weight_blob.read().unwrap().cpu_diff(),
        //               weight_blob.write().unwrap().mutable_cpu_data());
    }
}
