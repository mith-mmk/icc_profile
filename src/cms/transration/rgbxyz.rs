use std::io::Result;
use std::io::{Error,ErrorKind};
use crate::cms::ColorMatrix3D;

pub enum RGBToXYZCoefficient {
    CieRgb,
    SrgbD65,
    SrgbC,
    AdobeRgb,
    NtscRgb,
    Other(ColorMatrix3D)
}

impl RGBToXYZCoefficient {
    pub fn get(&self) -> ColorMatrix3D {
        match self {
            RGBToXYZCoefficient::CieRgb => {
                ColorMatrix3D::cie_rgb_to_xyz()
            },
            RGBToXYZCoefficient::SrgbD65 => {
                ColorMatrix3D::d65_rgb_to_xyz()
            },
            RGBToXYZCoefficient::SrgbC => {
                ColorMatrix3D::c_rgb_to_xyz()
            },
            RGBToXYZCoefficient::AdobeRgb => {
                ColorMatrix3D::adobe_rgb_to_xyz()
            },
            RGBToXYZCoefficient::NtscRgb => {
                ColorMatrix3D::ntsc_rgb_to_xyz()
            },
            RGBToXYZCoefficient::Other(matrix) => {
                matrix.clone()
            },
        }
    }
}

pub fn rgb_to_xyz(r:u8,g:u8,b:u8) -> (f64,f64,f64) {
    let matrix = RGBToXYZCoefficient::SrgbD65.get();
    matrix.convert_3d_u8_f64(r, g, b)
}

pub fn rgb_to_xyz_from_f64(r:f64,g:f64,b:f64) -> (f64,f64,f64) {
    let matrix = RGBToXYZCoefficient::SrgbD65.get();
    matrix.convert_3d(r, g, b)
}


pub fn rgb_to_xyz_entries_f64 (buf:&[u8],entries: usize,mode: &RGBToXYZCoefficient) -> Result<Vec<f64>> {
    if buf.len() < entries * 3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let matrix = mode.get();
    let mut buffer = Vec::with_capacity(entries * 3);

    for i in 0..entries {
        let ptr = i * 3;
        let r = buf[ptr];
        let g = buf[ptr + 1];
        let b = buf[ptr + 2];

        let (x,y,z) = matrix.convert_3d_u8_f64(r, g, b);

        buffer.push(x);
        buffer.push(y);
        buffer.push(z);
    }

    Ok(buffer)
}

pub fn rgba_to_xyz_entries_f64 (buf:&[u8],entries: usize,mode: &RGBToXYZCoefficient) -> Result<Vec<f64>> {
    if buf.len() < entries * 4 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let matrix = mode.get();
    let mut buffer = Vec::with_capacity(entries * 3);

    for i in 0..entries {
        let ptr = i * 4;
        let r = buf[ptr];
        let g = buf[ptr + 1];
        let b = buf[ptr + 2];

        let (x,y,z) = matrix.convert_3d_u8_f64(r, g, b);

        buffer.push(x);
        buffer.push(y);
        buffer.push(z);
    }

    Ok(buffer)
}


pub fn rgb_to_xyz_entries(buf:&[u8],entries: usize,mode: &RGBToXYZCoefficient) -> Result<Vec<u8>> {
    if buf.len() < entries * 3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let matrix = mode.get();
    let mut buffer = Vec::with_capacity(entries * 3);

    for i in 0..entries {
        let ptr = i * 3;
        let r = buf[ptr];
        let g = buf[ptr + 1];
        let b = buf[ptr + 2];

        let (x,y,z) = matrix.convert_3d_u8(r, g, b);

        buffer.push(x);
        buffer.push(y);
        buffer.push(z);
    }

    Ok(buffer)
}

pub fn rgba_to_xyz_entries (buf:&[u8],entries: usize,mode: &RGBToXYZCoefficient) -> Result<Vec<u8>> {
    if buf.len() < entries * 4 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let matrix = mode.get();
    let mut buffer = Vec::with_capacity(entries * 3);

    for i in 0..entries {
        let ptr = i * 4;
        let r = buf[ptr];
        let g = buf[ptr + 1];
        let b = buf[ptr + 2];

        let (x,y,z) = matrix.convert_3d_u8(r, g, b);

        buffer.push(x);
        buffer.push(y);
        buffer.push(z);
    }

    Ok(buffer)
}