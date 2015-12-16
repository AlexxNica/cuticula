extern crate collenchyma as co;
extern crate cuticula;

#[cfg(test)]
mod transformer_spec {
    use co::backend::{Backend, BackendConfig};
    use co::framework::IFramework;
    use co::frameworks::Native;

    use cuticula::{Set, Transformer, Image};
    use cuticula::image::{Crop};
    use cuticula::transformer::TransformerError;
    use std::path::Path;

    fn native_backend() -> Backend<Native> {
        let framework = Native::new();
        let hardwares = framework.hardwares();
        let backend_config = BackendConfig::new(framework, hardwares);
        Backend::new(backend_config).unwrap()
    }

    fn expected_result() -> Vec<f32> {
        vec![255.0, 255.0, 255.0, 255.0, 255.0, 255.0, 255.0, 255.0, 255.0, 0.0, 0.0, 0.0]
    }

    #[test]
    fn transform_returns_a_valid_result() {
        let path = Path::new("tests/assets/test_image.png");
        let img = Image::from_path(&path);
        match img.transform(vec![2, 2, 3]) {
            Ok(_) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn transform_returns_a_blob_with_data() {
        let path = Path::new("tests/assets/test_image.png");
        let img = Image::from_path(&path);
        match img.transform(vec![2, 2, 3]) {
            Ok(mut blob) => {
                let native = native_backend();
                let blob_data = blob.get_mut(native.device()).unwrap().as_mut_native().unwrap().as_mut_slice::<f32>();
                assert_eq!(expected_result(), blob_data);
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn transform_writs_correctly_to_blob_data() {
        let path = Path::new("tests/assets/test_image.png");
        let img = Image::from_path(&path);
        for i in 0..2 {
            match img.transform(vec![2, 2, 3]) {
                Ok(mut blob) => {
                    let native = native_backend();
                    let blob_data = blob.get_mut(native.device()).unwrap().as_mut_native().unwrap().as_mut_slice::<f32>();
                    if i > 1 {
                        assert_eq!(expected_result(), blob_data);
                    }
                },
                _ => assert!(false)
            }
        }
    }

    #[test]
    fn transform_returns_an_error_when_different_shape() {
        let path = Path::new("tests/assets/test_image.png");
        let img = Image::from_path(&path);
        match img.transform(vec![3, 3, 3]) {
            Err(TransformerError::InvalidShape) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn transform_returns_a_valid_result_with_modifiers() {
        let path = Path::new("tests/assets/test_image.png");
        let img = Image::from_path(&path);
        let crop = Crop { x: 0, y: 0, width: 1, height: 1 };
        match img.set(crop).transform(vec![1, 1, 3]) {
            Ok(_) => assert!(true),
            _ => assert!(false)
        }
    }
}
