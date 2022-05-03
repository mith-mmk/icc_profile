use std::io::Result;
use std::io::{Error,ErrorKind};
use crate::cms::ColorMatrix3D;

pub enum XYZtoRGBCoefficient {
    CieRgb,
    SrgbD65,
    SrgbC,
    AdobeRgb,
    NtscRgb,
    Other(ColorMatrix3D)
}

impl XYZtoRGBCoefficient {
    pub fn get(&self) -> ColorMatrix3D {
        match self {
            XYZtoRGBCoefficient::CieRgb => {
                ColorMatrix3D::cie_rgb_to_rgb()
            },
            XYZtoRGBCoefficient::SrgbD65 => {
                ColorMatrix3D::d65_rgb_to_rgb()
            },
            XYZtoRGBCoefficient::SrgbC => {
                ColorMatrix3D::c_rgb_to_rgb()
            },
            XYZtoRGBCoefficient::AdobeRgb => {
                ColorMatrix3D::adobe_rgb_to_rgb()
            },
            XYZtoRGBCoefficient::NtscRgb => {
                ColorMatrix3D::ntsc_rgb_to_rgb()
            },
            XYZtoRGBCoefficient::Other(matrix) => {
                matrix.clone()
            },
        }
    }
}

pub fn xyz_to_rgb(x:f32,y:f32,z:f32) -> (u8,u8,u8) {
    let matrix = XYZtoRGBCoefficient::SrgbD65.get();
    let (r,g,b) = matrix.convert_3d_f32_u8(x, y, z);
    (r,g,b)
}


pub fn xyz_to_rgb_entries (buf:&[u8],entries: usize,mode: &XYZtoRGBCoefficient) -> Result<Vec<u8>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let matrix = mode.get();
    let index = (entries / 3) as usize;
    let mut buffer = Vec::with_capacity(index * 3);

    for i in 0..entries {
        let ptr = i * 3;
        let x = buf[ptr];
        let y = buf[ptr + 1];
        let z = buf[ptr + 2];

        let (r,g,b) = matrix.convert_3d_u8(x, y, z);

        buffer.push(r);
        buffer.push(g);
        buffer.push(b);

    }

    Ok(buffer)
}

pub fn xyz_to_rgba_entries (buf:&[u8],entries: usize,mode: &XYZtoRGBCoefficient) -> Result<Vec<u8>> {
    if buf.len() < entries * 4 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let matrix = mode.get();
    let index = (entries / 4) as usize;
    let mut buffer = Vec::with_capacity(index * 3);

    for i in 0..entries {
        let ptr = i * 4;
        let x = buf[ptr];
        let y = buf[ptr + 1];
        let z = buf[ptr + 2];

        let (r,g,b) = matrix.convert_3d_u8(x, y, z);

        buffer.push(r);
        buffer.push(g);
        buffer.push(b);
        buffer.push(0xff);
    }


    Ok(buffer)
}