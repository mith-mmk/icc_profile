// icc profile reader
use crate::cms::transration::d4_to_d3_lut8;
use crate::cms::transration::d4_to_d3_lut16;
pub use crate::iccprofile::*;
use std::io::Result;
use std::io::{Error,ErrorKind};


// La*b* 0-100 -127-127 -127-127
pub fn cmyk_to_lab_lut16_u8(c:u8,m:u8,y:u8,k:u8,lut:&Mft2) -> (u8,u8,u8) {
    let (l,a,b) = cmyk_to_lab_lut16(c ,m ,y , k,lut);
    let l = ((l * 255.0 / 100.0) as i16 ).clamp(0,255) as u8;
    let a = ((a +128.0) as i16).clamp(0,255) as u8;
    let b = ((b +128.0) as i16).clamp(0,255) as u8;
    (l,a,b)
}

// YMCK -> Lab conversion 0-65536
pub fn cmyk_to_lab_lut16(c:u8,m:u8,y:u8,k:u8,lut:&Mft2) -> (f64,f64,f64) {

    let (l,a,b) = d4_to_d3_lut16(c,m,y,k,lut);

    let l = l * 100.0 / 65535.0;
    let a = a * 255.0 / 65535.0 - 127.5;
    let b = b * 255.0 / 65535.0 - 127.5;
    (l,a,b)
}

// La*b* 0-100 -127-127 -127-127
pub fn cmyk_to_lab_lut8(c:u8,m:u8,y:u8,k:u8,lut:&Mft1) -> (f64,f64,f64) {
    let (l,a,b) = d4_to_d3_lut8(c,m,y,k,lut);
    let l = l as f64 / 255.0 * 100.0;
    let a = a as f64 - 127.0;
    let b = b as f64 - 127.0;
    
    (l,a,b)
}

// no test
pub fn cmyk_to_lab_lut8_u8(c:u8,m:u8,y:u8,k:u8,lut:&Mft1) -> (u8,u8,u8) {

    let (l,a,b) = d4_to_d3_lut8(c,m,y,k,lut);
    (l,a,b)
}

pub fn cmyk_to_lab_entries_lut16(buf:&[u8],entries: usize,lut:&Mft2) -> Result<Vec<f64>> {
    if buf.len() < entries *4 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let mut buffer = Vec::with_capacity(entries * 3);

    for i in 0..entries {
        let ptr = i * 4;
        let c = buf[ptr];
        let m = buf[ptr + 1];
        let y = buf[ptr + 2];
        let k = buf[ptr + 3];
        let (l,a,b);
        (l,a,b) = cmyk_to_lab_lut16(c,m,y,k,lut);

        buffer.push(l);
        buffer.push(a);
        buffer.push(b);


    }

    Ok(buffer)
}

pub fn cmyk_to_lab_entries_lut8(buf:&[u8],entries: usize,lut:&Mft1) -> Result<Vec<f64>> {
    if buf.len() < entries *4 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let mut buffer = Vec::with_capacity(entries * 3);

    for i in 0..entries {
        let ptr = i * 4;
        let c = buf[ptr];
        let m = buf[ptr + 1];
        let y = buf[ptr + 2];
        let k = buf[ptr + 3];
        let (l,a,b);
        (l,a,b) = cmyk_to_lab_lut8(c,m,y,k,lut);

        buffer.push(l);
        buffer.push(a);
        buffer.push(b);


    }

    Ok(buffer)
}