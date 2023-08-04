use std::fmt::format;
use std::io::Cursor;
use bytes::Bytes;
use image::imageops::resize;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use lazy_static::lazy_static;
use photon_rs::{effects, filters, multiple, PhotonImage};
use photon_rs::{native::open_image_from_bytes, transform};
use photon_rs::filters::filter;
use crate::engine::{Engine, SpecTransform};
use crate::pb::{Contrast, Crop, filter, FlipH, FlipV, Resize, resize, Spec, Watermark, Filter, spec};
use crate::pb::resize::ResizeType;
use crate::pb::spec::Data;

lazy_static! {
    static ref WATERMARK:PhotonImage={
        let data=include_bytes!("../../mhd.png");
        let wartermark=open_image_from_bytes(data).unwrap();
        transform::resize(&wartermark,64,64,transform::SamplingFilter::Nearest)
    };
}

pub struct Photon(PhotonImage);

impl SpecTransform<&Watermark> for Photon {
    fn transform(&mut self, op: &Watermark) {
        multiple::watermark(&mut self.0, &WATERMARK, op.x, op.y);
    }
}


impl SpecTransform<&Resize> for Photon {
    fn transform(&mut self, op: &Resize) {
        let img = match resize::ResizeType::from_i32(op.rtype).unwrap() {
            ResizeType::Norma => {
                transform::resize(&mut self.0, op.width, op.height,
                                  resize::SampleFilter::from_i32(op.filter).unwrap().into())
            }
            ResizeType::SeamCarve => {
                transform::seam_carve(&mut self.0, op.width, op.height)
            }
        };
    }
}

impl SpecTransform<&Contrast> for Photon {
    fn transform(&mut self, op: &Contrast) {
        effects::adjust_contrast(&mut self.0, op.contrast);
    }
}

impl SpecTransform<&Filter> for Photon {
    fn transform(&mut self, op: &Filter) {
        match filter::Filter::from_i32(op.filter) {
            None => {}
            Some(filter::Filter::Unspecified) => {}
            Some(f) => {
                filters::filter(&mut self.0, f.to_str().unwrap())
            }
        }
    }
}

impl SpecTransform<&FlipV> for Photon {
    fn transform(&mut self, op: &FlipV) {
        transform::flipv(&mut self.0)
    }
}

impl SpecTransform<&FlipH> for Photon {
    fn transform(&mut self, op: &FlipH) {
        transform::fliph(&mut self.0)
    }
}

impl SpecTransform<&Crop> for Photon {
    fn transform(&mut self, op: &Crop) {
        let image = transform::crop(&mut self.0, op.x1, op.y1, op.x2, op.y2);
        self.0 = image;
    }
}

impl Engine for Photon {
    fn apply(&mut self, specs: &[Spec]) {
        for spec in specs.iter() {
            match spec.data {
                None => {}
                Some(spec::Data::Crop(ref v)) => { self.transform(v) }
                Some(spec::Data::Contrast(ref v)) => { self.transform(v) }
                Some(spec::Data::FlipV(ref v)) => { self.transform(v) }
                Some(spec::Data::FlipH(ref v)) => { self.transform(v) }
                Some(spec::Data::Watermark(ref v)) => { self.transform(v) }
                Some(spec::Data::Resize(ref v)) => { self.transform(v) }
                Some(spec::Data::Filter(ref v)) => { self.transform(v) }
            }
        }
    }

    fn generate(self, format: ImageOutputFormat) -> Vec<u8> {
        image_to_buf(self.0, format)
    }
}

fn image_to_buf(img: PhotonImage, format: ImageOutputFormat) -> Vec<u8> {
    let raw_pixels = img.get_raw_pixels();
    let width = img.get_width();
    let height = img.get_height();

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dyn_image = DynamicImage::ImageRgba8(img_buffer);

    let mut buffer = Vec::with_capacity(32768);
    dyn_image.write_to(&mut Cursor::new(&mut buffer), format).unwrap();
    buffer
}

impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(open_image_from_bytes(&value)?))
    }
}


