pub mod yuvrgb;
pub use yuvrgb::*;
pub mod rgbyuv;
pub use rgbyuv::*;
pub mod xyzrgb;
pub use xyzrgb::*;
//pub mod lut_convert;
//pub use lut_convert::*;
pub mod xyzlab;
pub use xyzlab::*;
pub mod labxyz;
pub use labxyz::*;
pub mod rgbxyz;
pub use rgbxyz::*;
pub mod cmyklab;
pub use cmyklab::*;
pub mod cmykrgb;
pub use cmykrgb::*;


pub struct WhitePoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl WhitePoint {
    pub fn new(x:f32,y:f32,z:f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    pub fn get(&self) -> &Self {
        &self
    }

    pub fn a() -> Self {
        Self{x:1.0985, y:1.0000, z:0.3558}
    }

    pub fn d50() -> Self {
        Self{x:0.9568, y:1.0000, z:0.9214}
    }

    pub fn d55() -> Self {
        Self{x:0.9642, y:1.0000, z:0.8251}
    }

    pub fn d65() -> Self {
        Self{x:0.9504, y:1.0000, z:1.0888}
    }

    pub fn from(xyz:&XYZNumber) -> Self {
        Self{x:xyz.x.as_f32()
            ,y:xyz.y.as_f32()
            ,z:xyz.z.as_f32()
        }
    }

    pub fn icc() -> Self {
        Self{x:0.9642, y:1.000, z:0.8249}
    }

    pub fn from_profile(decoded:&DecodedICCProfile) -> Self {
        let white_point = decoded.tags.get("wtpt");

        if let Some(white_point) = white_point {
            if let Data::XYZNumber(xyz) = white_point {
                return Self::from(xyz)
            }
        }
        Self::icc()
    }

}