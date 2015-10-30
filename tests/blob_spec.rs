extern crate phloem;

#[cfg(test)]
mod blod_spec {

    use phloem::Blob;

    #[test]
    fn it_works_for_f32() {
        let _: Blob<f32> = Blob::new();
    }

    #[test]
    fn it_works_for_f64() {
        let _: Blob<f64> = Blob::new();
    }

    #[test]
    fn it_works_basic_construction() {
        let blob: Blob<f32> = Blob::new();
        assert_eq!(blob.cpu_data().capacity(), 0);
    }

    #[test]
    fn it_works_of_shape_construction() {
        let shape = vec![2, 3, 2];
        let blob: Blob<f32> = Blob::of_shape(shape);
        assert_eq!(12, blob.cpu_data().capacity());
    }

    #[test]
    fn it_works_shape_string() {
        let shape = vec![2, 3, 2];
        let blob: Blob<f32> = Blob::of_shape(shape);
        assert_eq!("2 3 2 (3)", blob.shape_string());
    }

    #[test]
    fn correct_len() {
        let shape = vec![2, 3, 2];
        let mut blob: Blob<f32> = Blob::of_shape(shape);
        assert_eq!(0, blob.len());

        blob.mutable_cpu_data().push(0f32);
        assert_eq!(1, blob.len());
    }

    #[test]
    fn correct_capacity() {
        let shape = vec![2, 2, 2, 2];
        let blob: Blob<f32> = Blob::of_shape(shape);
        assert_eq!(16, blob.capacity());
    }

    #[test]
    fn correct_shape() {
        let shape = vec![2, 3, 4];
        let blob: Blob<f32> = Blob::of_shape(shape);
        assert_eq!(vec![2, 3, 4], blob.shape());
    }
}
