
use crate::cms::transration::*;
use crate::Mft2;

pub fn cmyk_to_rgb(y:u8,m:u8,c:u8,k:u8) -> (u8,u8,u8) {
    let r = c + k - 255;
    let g = m + k - 255;
    let b = y + k - 255;
    (r,g,b)
}

pub fn cmyk_to_rgb_lut16(c:u8,m:u8,y:u8,k:u8,lut:&Mft2,wp:&WhitePoint) -> (u8,u8,u8) {
    let (l,a,b) = cmyk_to_lab_lut16(y,m,c,k,lut);
    let (x,y,z) = lab_to_xyz(l,a,b,wp);
    let (r,g,b) = xyz_to_rgb(x,y,z);

    (r,g,b)
}

pub fn cmyk_to_rgb_lut8(c:u8,m:u8,y:u8,k:u8,lut:&Mft1,wp:&WhitePoint) -> (u8,u8,u8) {
    let (l,a,b) = cmyk_to_lab_lut8(y,m,c,k,lut);
    let (x,y,z) = lab_to_xyz(l,a,b,wp);
    let (r,g,b) = xyz_to_rgb(x,y,z);

    (r,g,b)
}


pub fn cmyk_to_rgb_from_profile(c:u8,m:u8,y:u8,k:u8,decoded:&DecodedICCProfile) -> (u8,u8,u8) {
    if decoded.color_space == 0x434d594b {  // CMYK
        let lut = decoded.tags.get("A2B0");
        let wp = WhitePoint::from_profile(decoded);
        if let Some(lut) = lut {
            match lut {
                Data::Lut16(lut16) => {
                    return cmyk_to_rgb_lut16(c,m,y,k,lut16,&wp)
                },
                Data::Lut8(lut8) => {
                    return cmyk_to_rgb_lut8(c,m,y,k,lut8,&wp)

                },
                _ => {
                }
            }
        }
    }
    // not has profile
    cmyk_to_rgb(c, m, y, k)
}