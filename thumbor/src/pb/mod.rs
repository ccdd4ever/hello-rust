use base64::{Engine as _, engine, engine::general_purpose};
use photon_rs::transform::SamplingFilter;
use prost::Message;
use std::convert::TryFrom;

mod abi;

pub use abi::*;
use crate::pb::resize::SampleFilter;

//NOTE1: self作为返回值？
// Self is the type of the current object. It may appear either in a trait or an impl,
// but appears most often in trait where it is a stand-in for whatever type will end up implementing the trait (which is unknown when defining the trait):

//NOTE2: impl struct == 类方法
impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self { specs }
    }
}

impl From<&ImageSpec> for String {
    fn from(value: &ImageSpec) -> Self {
        let data = value.encode_to_vec();
        general_purpose::URL_SAFE_NO_PAD.encode(data)
    }
}

impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = general_purpose::URL_SAFE_NO_PAD.decode(value)?;
        Ok(ImageSpec::decode(&data[..])?)
    }
}

//NOTE1: Some using in match Option<T>
//TODO 'static str? 生命周期
impl filter::Filter {
    pub fn to_str(&self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic => { Some("oceanic") }
            filter::Filter::Islands => { Some("islands") }
            filter::Filter::Marine => { Some("marine") }
        }
    }
}

impl From<resize::SampleFilter> for SamplingFilter {
    fn from(value: SampleFilter) -> Self {
        match value {
            SampleFilter::Undefined => { SamplingFilter::Nearest }
            SampleFilter::Nearest => { SamplingFilter::Nearest }
            SampleFilter::Triangle => { SamplingFilter::Triangle }
            SampleFilter::CatmullRom => { SamplingFilter::CatmullRom }
            SampleFilter::Gaussian => { SamplingFilter::Gaussian }
            SampleFilter::Lanczos3 => { SamplingFilter::Lanczos3 }
        }
    }
}

impl Spec {
    pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize({
                Resize {
                    width,
                    height,
                    rtype: resize::ResizeType::SeamCarve as i32,
                    filter: resize::SampleFilter::Undefined as i32,
                }
            })),
        }
    }
    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize {
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: filter as i32,
            })),
        }
    }

    pub fn new_filter(filter: filter::Filter) -> Self {
        Self {
            data: Some(spec::Data::Filter(Filter {
                filter: filter as i32,
            })),
        }
    }

    pub fn new_watermark(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark { x, y }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;
    use std::convert::TryInto;
    use percent_encoding::{NON_ALPHANUMERIC, percent_encode};

    #[test]
    fn encoded_spec_could_be_decoded() {
        let spec1 = Spec::new_resize(600, 600, resize::SampleFilter::CatmullRom);
        let spec2 = Spec::new_filter(filter::Filter::Islands);
        let spec3 = Spec::new_watermark(0, 0);
        let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);
        let s: String = image_spec.borrow().into();
        assert_eq!(image_spec, s.as_str().try_into().unwrap());
        println!("{}\n", s);
        let url_encode = percent_encode("https://images.pexels.com/photos/16450166/pexels-photo-16450166.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1".as_bytes(), NON_ALPHANUMERIC);
        println!("{}", url_encode);
    }
}