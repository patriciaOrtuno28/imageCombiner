mod args;
use args::Args;
use image::GenericImageView;
use image::{DynamicImage, ImageFormat, io::Reader, imageops::FilterType::Triangle};
use std::io::BufReader;
use std::fs::File;

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = height * width * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage { width, height, data: buffer, name}
    }
    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        } else {
            self.data = data;
            Ok(())
        }
    }
}

#[allow(unused_variables)]
fn main() -> Result<(), ImageDataErrors> {

    let args = Args::new();

    let (img1, img_format1) = find_image_from_path(args.img1);
    let (img2, img_format2) = find_image_from_path(args.img2);

    if img_format1 != img_format2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (img1, img2) = standardise_size(img1, img2);
    let mut output = FloatingImage::new(img1.width(), img1.height(), args.output);
    let combined_data = combine_images(img1, img2);
    output.set_data(combined_data)?; // ? -> unwrap value and propagate into current function

    image::save_buffer_with_format(
        output.name, 
        &output.data, 
        output.width, 
        output.height, 
        image::ColorType::Rgba8, 
        img_format1
    ).unwrap();

    Ok(())

    // println!("{:?}", args);
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}

fn get_smallest_dim(dim1: (u32, u32), dim2: (u32, u32)) -> (u32, u32) {
    let pix1 = dim1.0 * dim1.1;
    let pix2 = dim2.0 * dim2.1;
    return if pix1 < pix2 {dim1} else {dim2};
}

fn standardise_size(img1: DynamicImage, img2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dim(img1.dimensions(), img2.dimensions());
    if img2.dimensions() == (width, height) {
        let img1_resized = img1.resize_exact(width, height, Triangle);
        return (img1_resized, img2);
    } else {
        let img2_resized = img2.resize_exact(width, height, Triangle);
        return (img1, img2_resized);
    }
}

fn combine_images(img1: DynamicImage, img2: DynamicImage) -> Vec<u8> {
    let vec1 = img1.to_rgba8().into_vec();
    let vec2 = img2.to_rgba8().into_vec();

    alternate_pixels(vec1, vec2)
}

fn alternate_pixels(vec1: Vec<u8>, vec2: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec1.len()];

    let mut i = 0;
    while i < vec1.len() {
        if i % 8 == 0 {
            combined_data.splice(i..=i+3, set_rgba(&vec1, i, i+3));
        } else {
            combined_data.splice(i..=i+3, set_rgba(&vec2, i, i+3));
        } 
        i += 4;
    }
    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val = match vec.get(i){
            Some (d) => *d,
            None => panic!("Index out of bounds.")
        };
        rgba.push(val);
    }
    rgba
}
