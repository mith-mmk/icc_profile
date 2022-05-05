use crate::cms::ColorMatrix3D;
use std::io::Result;
use std::io::{Error,ErrorKind};

pub enum RGBToYUVCoefficient {
    Bt601,
    Pal,
    Bt709,
    Other(ColorMatrix3D)
}

impl RGBToYUVCoefficient {
    pub fn get(&self) -> ColorMatrix3D {
        match self {
            RGBToYUVCoefficient::Bt601 => {
                ColorMatrix3D::from(
                    &[0.299, 0.587, 0.114,
                    -0.168736, -0.331264, 0.5,
                    0.5, -0.418688, -0.081312]).unwrap()
            },
            RGBToYUVCoefficient::Pal => {
                ColorMatrix3D::from(
                    &[0.299, 0.587, 0.114,
                     -0.14713, -0.28886, 0.436,
                     0.615, -0.51499, -0.10001]).unwrap()     
            },
            RGBToYUVCoefficient::Bt709 => {
                ColorMatrix3D::from(
                    &[0.2126, 0.7152, 0.0722,
                     -0.114572, -0.385428, 0.5,
                     0.5, -0.454153, -0.045847]).unwrap()      
            },
            RGBToYUVCoefficient::Other(matrix) => {
                matrix.clone()
            }

        }
    }

}

/// (R,G,B) -> (Y,Cb,Cr)
pub fn rgb_to_yuv(r:u8,g:u8,b:u8) -> (u8,u8,u8) {
    let matrix = RGBToYUVCoefficient::Bt601.get();
    matrix.convert_3d_u8(r, g, b) 
}

pub fn rgb_to_yuv_with_mode(r:u8,g:u8,b:u8,mode: &RGBToYUVCoefficient) -> (u8,u8,u8) {
    let matrix = mode.get();
    matrix.convert_3d_u8(r, g, b) 
}


pub fn rgb_to_yuv_entries (buf:&[u8],entries: usize,mode: &RGBToYUVCoefficient) -> Result<Vec<u8>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let index = (entries / 3) as usize;
    let mut buffer = Vec::with_capacity(index * 3);
    let matrix = mode.get();

    for i in 0..entries {
        let ptr = i * 3;
        let r = buf[ptr];
        let g = buf[ptr + 1];
        let b = buf[ptr + 2];

        let (y,u,v) =matrix.convert_3d_u8(r, g, b);

        buffer.push(y);
        buffer.push(u);
        buffer.push(v);
    }

    Ok(buffer)
}

pub fn yuv_to_rgba_entries (buf:&[u8],entries: usize,mode: &RGBToYUVCoefficient) -> Result<Vec<u8>> {
    if buf.len() < entries * 4 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let index = (entries / 4) as usize;
    let mut buffer = Vec::with_capacity(index * 3);
    let matrix = mode.get();

    for i in 0..entries {
        let ptr = i * 4;
        let r  = buf[ptr];
        let g = buf[ptr + 1];
        let b = buf[ptr + 2];

        let (y,u,v) =matrix.convert_3d_u8(r, g, b);

        buffer.push(y);
        buffer.push(u);
        buffer.push(v);
    }


    Ok(buffer)
}