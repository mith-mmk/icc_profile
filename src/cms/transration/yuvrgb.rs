use crate::cms::ColorMatrix3D;
use std::io::Result;
use std::io::{Error,ErrorKind};


pub enum YUVToRGBCoefficient {
    Bt601,
    Pal,
    Bt709,
    Other(ColorMatrix3D)
}

impl YUVToRGBCoefficient {
    pub fn get(&self) -> ColorMatrix3D {
        match self {
            YUVToRGBCoefficient::Bt601 => {
                let crr = 1.402;
                let cbg = -0.34414;
                let crg = -0.71414;
                let cbb = 1.772;
                ColorMatrix3D::from(
                    &[1.0, 0.0, crr,
                     1.0, cbg, crg,
                     1.0, cbb, 1.0]).unwrap()
            },
            YUVToRGBCoefficient::Pal => {
                let crr = 1.1398;
                let cbg = -0.39465;
                let crg = -0.5806;
                let cbb = 2.03211;
                ColorMatrix3D::from(
                    &[1.0, 0.0, crr,
                     1.0, cbg, crg,
                     1.0, cbb, 1.0]).unwrap()
            },
            YUVToRGBCoefficient::Bt709 => {
                let crr = 1.5748;
                let cbg = -0.187324;
                let crg = -0.468124;
                let cbb = 1.8556;
                ColorMatrix3D::from(
                    &[1.0, 0.0, crr,
                     1.0, cbg, crg,
                     1.0, cbb, 1.0]).unwrap()
            },
            YUVToRGBCoefficient::Other(matrix) => {
                matrix.clone()
            }
        }
    }
}


/// (Y,Cb,Cr) -> (R, G, B)
pub fn yuv_to_rgb (y:u8,cb:u8,cr:u8) -> (u8,u8,u8) {
    let matrix = YUVToRGBCoefficient::Bt601.get();
    matrix.convert_3d_u8(y, cb, cr) 
}

pub fn yuv_to_rgb_with_mode (y:u8,cb:u8,cr:u8,mode: &YUVToRGBCoefficient) -> (u8,u8,u8) {
    let matrix = mode.get();
    matrix.convert_3d_u8(y, cb, cr) 
}


pub fn yuv_to_rgb_entries (buf:&[u8],entries: usize,mode: &YUVToRGBCoefficient) -> Result<Vec<u8>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let matrix = mode.get();

    let mut buffer = Vec::with_capacity(entries * 3);

    for i in 0..entries {
        let ptr = i * 3;
        let y  = buf[ptr];
        let cb = buf[ptr + 1];
        let cr = buf[ptr + 2];

        let (r,g,b) = matrix.convert_3d_u8(y, cb, cr);

        buffer.push(r);
        buffer.push(g);
        buffer.push(b);
    }

    Ok(buffer)
}

pub fn yuv_to_rgba_entries (buf:&[u8],entries: usize,mode: &YUVToRGBCoefficient) -> Result<Vec<u8>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let matrix = mode.get();
    let mut buffer = Vec::with_capacity(entries * 4);

    for i in 0..entries {
        let ptr = i * 3;
        let y  = buf[ptr];
        let cb = buf[ptr + 1];
        let cr = buf[ptr + 2];

        let (r,g,b) = matrix.convert_3d_u8(y, cb, cr);

        buffer.push(r);
        buffer.push(g);
        buffer.push(b);
        buffer.push(0xff);
    }

    Ok(buffer)
}