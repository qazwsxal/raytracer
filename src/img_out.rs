use image::{RgbImage, ImageResult};
use ndarray::Array3;


pub fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

pub fn array_save(arr: Array3<u8>, filepath: String) ->  ImageResult<()>{

    let image = array_to_image(arr);
    image.save(filepath)

}