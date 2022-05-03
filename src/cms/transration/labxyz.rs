use std::io::{Error,ErrorKind};
use std::io::Result;
use super::WhitePoint;

pub fn lab_to_xyz(l:f32,a:f32,b:f32,wp: &WhitePoint) -> (f32,f32,f32) {
    let th: f32 = 6.0/29.0;
    let th_3 = th.powi(3);

    let fy = (l +16.0) / 116.0;
    let fx =  fy + (a /500.0);
    let fz =  fy - (b /200.0);

    let x =  if fy > th {fx.powi(3) * wp.x} else {th_3 * (116.0 * fx - 16.0) * wp.x };
    let y =  if fy > th {fy.powi(3) * wp.y} else {th_3 * (116.0 * fy - 16.0) * wp.y };
    let z =  if fy > th {fz.powi(3) * wp.z} else {th_3 * (116.0 * fz - 16.0) * wp.z };

    (x,y,z)
}


// xyz =0.0-1.0
pub fn lab_to_xyz_entries_f32(buf:&[f32],entries: usize,wp: &WhitePoint)  -> Result<Vec<f32>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let index = (entries / 3) as usize;
    let mut buffer = Vec::with_capacity(index * 3);

    for i in 0..entries {
        let ptr = i * 3;
        let l = buf[ptr];
        let a = buf[ptr + 1];
        let b = buf[ptr + 2];
        let (x,y,z) = lab_to_xyz(l,a,b,wp);

        buffer.push(x);
        buffer.push(y);
        buffer.push(z);
    }

    Ok(buffer)

}

// lab 0-255 / xyz = 0-255

pub fn lab_to_xyz_entries(buf:&[u8],entries: usize,wp: &WhitePoint)  -> Result<Vec<u8>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let index = (entries / 3) as usize;
    let mut buffer = Vec::with_capacity(index * 3);

    for i in 0..entries {
        let ptr = i * 3;
        let l = buf[ptr]     as f32 / 255.0 * 100.0;
        let a = buf[ptr + 1] as f32 - 128.0;
        let b = buf[ptr + 2] as f32 - 128.0;
        let (x,y,z) = lab_to_xyz(l,a,b,wp);

        let x = ((x * 255.0) as i16).clamp(0,255) as u8;
        let y = ((y * 255.0) as i16).clamp(0,255) as u8;
        let z = ((z * 255.0) as i16).clamp(0,255) as u8;

        buffer.push(x);
        buffer.push(y);
        buffer.push(z);
    }

    Ok(buffer)
}

