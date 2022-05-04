// La*b* 0-100 -127-127 -127-127
use crate::cms::transration::d3_to_d4_lut8;
use crate::cms::transration::d3_to_d4_lut16;
use crate::{Mft1,Mft2};
use std::io::Result;
use std::io::{Error,ErrorKind};

pub fn lab_to_cmyk_lut16_u8(l:u8,a:u8,b:u8,lut:&Mft2) -> (u8,u8,u8,u8) {

    let (c,m,y,k) = lab_to_cmyk_lut16(l ,a ,b, lut);
    
    let c = (c / 65535.0 * 255.0 + 0.5) as u8;
    let m = (m / 65535.0 * 255.0 + 0.5) as u8;
    let y = (y / 65535.0 * 255.0 + 0.5) as u8;
    let k = (k / 65535.0 * 255.0 + 0.5) as u8;
    (c,m,y,k)
}

pub fn lab_to_cmyk_lut16(l:u8,a:u8,b:u8,lut:&Mft2) -> (f64,f64,f64,f64) {

    let (c,m,y,k) = d3_to_d4_lut16(l,a,b,lut);

    let c = c / 65535.0 * 100.0;
    let m = m / 65535.0 * 100.0;
    let y = y / 65535.0 * 100.0;
    let k = k / 65535.0 * 100.0;
    (c,m,y,k)
}

pub fn lab_f64_to_cmyk_lut16(l:f64,a:f64,b:f64,lut:&Mft2) -> (f64,f64,f64,f64) {
    let l = (l / 100.0 * 255.0) as u8;
    let a = (a + 127.5) as u8;
    let b = (b + 127.5) as u8;
    let (c,m,y,k) = d3_to_d4_lut16(l,a,b,lut);

    let c = c / 65535.0 * 100.0;
    let m = m / 65535.0 * 100.0;
    let y = y / 65535.0 * 100.0;
    let k = k / 65535.0 * 100.0;
    (c,m,y,k)
}


pub fn lab_to_cmyk_lut8(l:u8,a:u8,b:u8,lut:&Mft1) -> (f64,f64,f64,f64) {
    let (c,m,y,k) = d3_to_d4_lut8(l,a,b,lut);

    let c = c as f64 / 255.0 * 100.0;
    let m = m as f64 / 255.0 * 100.0;
    let y = y as f64 / 255.0 * 100.0;
    let k = k as f64 / 255.0 * 100.0;
    (c,m,y,k)
}

pub fn lab_f64_to_cmyk_lut8(l:f64,a:f64,b:f64,lut:&Mft1) -> (f64,f64,f64,f64) {
    let l = (l / 100.0 * 255.0 + 0.5) as u8;
    let a = (a + 127.5) as u8;
    let b = (b + 127.5) as u8;
    lab_to_cmyk_lut8(l,a,b,lut)
}

// no test
pub fn lab_to_cmyk_lut8_u8(l:u8,a:u8,b:u8,lut:&Mft1) -> (u8,u8,u8,u8) {
    d3_to_d4_lut8(l,a,b,lut)
}

pub fn lab_f64_to_cmyk_lut8_u8(l:f64,a:f64,b:f64,lut:&Mft1) -> (u8,u8,u8,u8) {
    let l = (l / 100.0 * 255.0 + 0.5) as u8;
    let a = (a + 127.5) as u8;
    let b = (b + 127.5) as u8;
    d3_to_d4_lut8(l,a,b,lut)
}


pub fn lab_to_cmyk_entries_lut16_u8(buf:&[u8],entries: usize,lut:&Mft2) -> Result<Vec<u8>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let mut buffer = Vec::with_capacity(entries * 4);

    for i in 0..entries {
        let ptr = i * 4;
        let l = buf[ptr];
        let a = buf[ptr + 1];
        let b = buf[ptr + 2];
        let (c,m,y,k) = lab_to_cmyk_lut16_u8(l,a,b,lut);

        buffer.push(c);
        buffer.push(m);
        buffer.push(y);
        buffer.push(k);
    }

    Ok(buffer)
}

pub fn lab_to_cmyk_entries_lut16(buf:&[u8],entries: usize,lut:&Mft2) -> Result<Vec<f64>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let mut buffer = Vec::with_capacity(entries * 4);

    for i in 0..entries {
        let ptr = i * 4;
        let l = buf[ptr];
        let a = buf[ptr + 1];
        let b = buf[ptr + 2];
        let (c,m,y,k) = lab_to_cmyk_lut16(l,a,b,lut);

        buffer.push(c);
        buffer.push(m);
        buffer.push(y);
        buffer.push(k);
    }

    Ok(buffer)
}

pub fn lab_to_cmyk_entries_lut8(buf:&[u8],entries: usize,lut:&Mft1) -> Result<Vec<f64>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let mut buffer = Vec::with_capacity(entries * 4);

    for i in 0..entries {
        let ptr = i * 4;
        let l = buf[ptr];
        let a = buf[ptr + 1];
        let b = buf[ptr + 2];
        let (c,m,y,k) = lab_to_cmyk_lut8(l,a,b,lut);

        buffer.push(c);
        buffer.push(m);
        buffer.push(y);
        buffer.push(k);
    }

    Ok(buffer)
}


pub fn lab_to_cmyk_entries_lut8_u8(buf:&[u8],entries: usize,lut:&Mft1) -> Result<Vec<u8>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let mut buffer = Vec::with_capacity(entries * 4);

    for i in 0..entries {
        let ptr = i * 4;
        let l = buf[ptr];
        let a = buf[ptr + 1];
        let b = buf[ptr + 2];
        let (c,m,y,k) = lab_to_cmyk_lut8_u8(l,a,b,lut);

        buffer.push(c);
        buffer.push(m);
        buffer.push(y);
        buffer.push(k);
    }

    Ok(buffer)
}

pub fn lab_f64_to_cmyk_entries_lut8(buf:&[f64],entries: usize,lut:&Mft1) -> Result<Vec<f64>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let mut buffer = Vec::with_capacity(entries * 4);

    for i in 0..entries {
        let ptr = i * 4;
        let l = buf[ptr];
        let a = buf[ptr + 1];
        let b = buf[ptr + 2];
        let l = (l / 100.0 * 255.0 + 0.5) as u8;
        let a = (a + 127.5) as u8;
        let b = (b + 127.5) as u8;
        let (c,m,y,k) = lab_to_cmyk_lut8(l,a,b,lut);

        buffer.push(c);
        buffer.push(m);
        buffer.push(y);
        buffer.push(k);
    }

    Ok(buffer)
}


pub fn lab_f64_to_cmyk_entries_lut8_u8(buf:&[f64],entries: usize,lut:&Mft1) -> Result<Vec<u8>> {
    if buf.len() < entries *3 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let mut buffer = Vec::with_capacity(entries * 4);

    for i in 0..entries {
        let ptr = i * 4;
        let l = buf[ptr];
        let a = buf[ptr + 1];
        let b = buf[ptr + 2];
        let l = (l / 100.0 * 255.0 + 0.5) as u8;
        let a = (a + 127.5) as u8;
        let b = (b + 127.5) as u8;
        let (c,m,y,k) = lab_to_cmyk_lut8_u8(l,a,b,lut);

        buffer.push(c);
        buffer.push(m);
        buffer.push(y);
        buffer.push(k);
    }

    Ok(buffer)
}

