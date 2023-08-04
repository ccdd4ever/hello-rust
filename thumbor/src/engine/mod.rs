mod photon;

pub use photon::Photon;
use crate::pb::Spec;
use image::ImageOutputFormat;

//Engine trait,未来可以替换图片处理引擎
pub trait Engine {
    fn apply(&mut self, specs: &[Spec]);
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

//添加spec实现的trait
pub trait SpecTransform<T> {
    fn transform(&mut self, op: T);
}
