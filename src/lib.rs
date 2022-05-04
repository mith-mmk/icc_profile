//! # Default Color spaces ranges
//! - RGB        0..255,0..255,0..255 (u8)
//! - YUV(YCbCr) 0..255,0..255,0..255 (u8)
//! - XYZ 0.0..1.0,0.0..1.0,0.0..1.0 (f64)
//! - L*a*b* 0.0-100.0,-127.0..127.0,-127.0..127.0 (f64)
//! - CMYK 0..255,0..255,0..255,0..255 (u8)

pub use crate::iccprofile::*;
pub mod utils;
pub mod iccprofile;
pub mod cms;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
