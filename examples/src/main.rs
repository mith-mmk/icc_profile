// icc profile reader
use icc_profile::cms::transration::*;
use icc_profile::utils::decoded_print;
pub use icc_profile::iccprofile::*;

use std::env;

pub fn main() -> std::io::Result<()> {
    let mut is_fast = true;
    for argument in env::args() {
        if is_fast {
            is_fast = false;
            continue
        }
        println!("{}",argument);
        let icc_profile = icc_profile::utils::load(argument)?;
        let decoded = DecodedICCProfile::new(&icc_profile.data)?;
        println!("{}",decoded_print(&decoded, 0)?);
        let mut count = 0;
        for r in 0..=255 { 
            for g in 0..=255 { 
                for b in 0..=255 { 
                    let (x,y,z) = rgb_to_xyz(r,g,b);        
                    let (l,la,lb) = xyz_to_lab(x,y,z);
                    let (x1,y1,z1) = lab_to_xyz(l,la,lb);
                    let (r1,g1,b1) = xyz_to_rgb(x1,y1,z1);
                    if (r == r1) && (g == g1) && (b == b1) {

                    } else {
                        if count % 100 == 0 {
                            println!("RGB {} {} {} inverce {} {} {}",r,g,b,r1,g1,b1);
                            println!("XYZ {} {} {}",x,y,z);
                            println!("Lab {} {} {}",l,la,lb);
                            println!("xyz {} {} {}",x1,y1,z1);
                        }
                        count += 1;
                    }
                }
            }
        }
        println!("error count {}",count);
    }
    Ok(())
}