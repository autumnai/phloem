extern crate phloem;

#[cfg(test)]
mod blod_spec {

    use phloem::Blob;

    #[test]
    fn construction_basic() {
        let blob: Blob = Blob::new();
        assert_eq!(blob.cpu_data().capacity(), 0);
    }

    #[test]
    fn construction_of_shape() {
        let shape = vec![2, 3, 2];
        let blob: Blob = Blob::of_shape(shape);
        assert_eq!(12, blob.cpu_data().capacity());
    }

    #[test]
    fn shape_string() {
        let shape = vec![2, 3, 2];
        let blob: Blob = Blob::of_shape(shape);
        assert_eq!("2 3 2 (3)", blob.shape_string());
    }
}
