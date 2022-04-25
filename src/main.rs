use crate::utils::decoded_print;
pub use crate::iccprofile::*;
pub mod utils;
pub mod iccprofile;

use std::env;

// YMCK -> RGB conversion
fn clut_calc16(lut:&Mft2,y:u8,m:u8,c:u8,k:u8) -> (u8,u8,u8) {
    let grid_points = lut.number_of_clut_grid_points as usize;

    let delta = 65535 / (lut.input_table_enteries as usize) ;
    let y = (y as usize * 256 + y as usize) / lut.input_table_enteries as usize;
    let y_delta = (y as usize * 256 + y as usize) % lut.input_table_enteries as usize;
    let m = (m as usize * 256 + m as usize) / lut.input_table_enteries as usize;
    let m_delta = (m as usize * 256 + m as usize) % lut.input_table_enteries as usize;
    let c = (c as usize * 256 + c as usize) / lut.input_table_enteries as usize;
    let c_delta = (c as usize * 256 + c as usize) % lut.input_table_enteries as usize;
    let k = (k as usize * 256 + k as usize) / lut.input_table_enteries as usize;
    let k_delta= (k as usize * 256 + k as usize) % lut.input_table_enteries as usize;

    let y1 = lut.input_table [y as usize];
    let m1 = lut.input_table [m as usize + lut.input_table_enteries as usize];
    let c1 = lut.input_table [c as usize + lut.input_table_enteries as usize * 2];
    let k1 = lut.input_table [k as usize + lut.input_table_enteries as usize * 3];
//    println!("modified {} {} {} {}",y1,m1,c1,k1);
    let y2 = if y < lut.input_table_enteries as usize - 1 {lut.input_table [y as usize + 1]} else {y1};
    let m2 = if m < lut.input_table_enteries as usize - 1 {lut.input_table [m as usize + 1 + lut.input_table_enteries as usize]} else {m1};
    let c2 = if c < lut.input_table_enteries as usize - 1 {lut.input_table [c as usize + 1 + lut.input_table_enteries as usize * 2]} else {c1};
    let k2 = if k < lut.input_table_enteries as usize - 1 {lut.input_table [k as usize + 1 + lut.input_table_enteries as usize * 3]} else {k1};
//    println!("modified2 {} {} {} {}",y2,m2,c2,k2);
//    println!("delta {} {} {} {} {}",y_delta,m_delta,c_delta,k_delta,delta);
    let y_delta = y_delta as f32 / delta as f32;
    let m_delta = m_delta as f32 / delta as f32;
    let c_delta = c_delta as f32 / delta as f32;
    let k_delta = k_delta as f32 / delta as f32;
//    println!("delta {} {} {} {}",y_delta,m_delta,c_delta,k_delta);
    let y = y1 as isize + ((y2 as isize - y1 as isize) as f32 * y_delta) as isize;
    let m = m1 as isize + ((m2 as isize - m1 as isize) as f32 * m_delta) as isize;
    let c = c1 as isize + ((c2 as isize - c1 as isize) as f32 * c_delta) as isize;
    let k = k1 as isize + ((k2 as isize - k1 as isize) as f32 * k_delta) as isize;
//    println!("modified {} {} {} {}",y,m,c,k);

    let y_grid = ((y as f32 / 65536.0) * grid_points as f32).floor() as usize;
    let m_grid = ((m as f32 / 65536.0) * grid_points as f32).floor() as usize;
    let c_grid = ((c as f32 / 65536.0) * grid_points as f32).floor() as usize;
    let k_grid = ((k as f32 / 65536.0) * grid_points as f32).floor() as usize;
    let y_grid_delta = ((y as f32 / 65536.0) * grid_points as f32) - y_grid as f32;
    let m_grid_delta = ((m as f32 / 65536.0) * grid_points as f32) - m_grid as f32;
    let c_grid_delta = ((c as f32 / 65536.0) * grid_points as f32) - c_grid as f32;
    let k_grid_delta = ((k as f32 / 65536.0) * grid_points as f32) - k_grid as f32;
    let grid_delta = y_grid_delta * m_grid_delta * c_grid_delta * k_grid_delta;

    let grid = y_grid * grid_points.pow(3) + m_grid * grid_points.pow(2)  + c_grid * grid_points + k_grid;
    let grid = grid * lut.output_channels as usize;
    let y_grid2 = if y_grid < grid_points - 1 {y_grid + 1} else {y_grid}; 
    let m_grid2 = if m_grid < grid_points - 1 {m_grid + 1} else {m_grid}; 
    let c_grid2 = if c_grid < grid_points - 1 {c_grid + 1} else {c_grid}; 
    let k_grid2 = if k_grid < grid_points - 1 {k_grid + 1} else {k_grid}; 
    let grid2 = y_grid2 * grid_points.pow(3) + m_grid2 * grid_points.pow(2)  + c_grid2 * grid_points + k_grid2;
    let grid2 = grid2 * lut.output_channels as usize;
 
    let cy = lut.clut_values[grid]   as f32 * (1.0  -grid_delta) + lut.clut_values[grid2]   as f32 * grid_delta;
    let cb = lut.clut_values[grid+1] as f32 * (1.0  -grid_delta) + lut.clut_values[grid2+1] as f32 * grid_delta;
    let cr = lut.clut_values[grid+2] as f32 * (1.0  -grid_delta) + lut.clut_values[grid2+2] as f32 * grid_delta;
    
    let dev = 65536.0 / (lut.output_table_enteries as f32 -1.0);

    let ocy = (cy / dev) as usize;
    let ocb = (cb / dev) as usize;
    let ocr = (cr / dev) as usize;

    let ocy_delta = cy / dev - ocy as f32;
    let ocb_delta = cb / dev - ocb as f32;
    let ocr_delta = cr / dev - ocr as f32;

    let cy1 = lut.output_table [ocy as usize] as usize;
    let cb1 = lut.output_table [ocb as usize + lut.output_table_enteries as usize] as usize;
    let cr1 = lut.output_table [ocr as usize + lut.output_table_enteries as usize * 2] as usize;
    let cy2 = if cy1 < lut.output_table_enteries as usize - 1 {lut.output_table[ocy as usize + 1]} else {y1};
    let cb2 = if cb1 < lut.output_table_enteries as usize - 1 {lut.output_table[ocb as usize + 1 + lut.output_table_enteries as usize]} else {m1};
    let cr2 = if cr1 < lut.output_table_enteries as usize - 1 {lut.output_table[ocr as usize + 1 + lut.output_table_enteries as usize * 2]} else {c1};

    let cy = cy1 as f32 * (1.0 - ocy_delta) + cy2 as f32 * ocy_delta;
    let cb = cb1 as f32 * (1.0 - ocb_delta) + cb2 as f32 * ocb_delta;
    let cr = cr1 as f32 * (1.0 - ocr_delta) + cr2 as f32 * ocr_delta;

    let cy = cy / 256.0;
    let cr = cr / 256.0;
    let cb = cb / 256.0;


    let crr = 1.402;
    let cbg = - 0.34414;
    let crg = - 0.71414;
    let cbb = 1.77;


    let red  = cy + 16.0 + (crr * (cr - 128.0));
    let green= cy + 16.0 + (cbg * (cb - 128.0) + crg * (cr - 128.0));
    let blue = cy + 16.0 + (cbb * (cb - 128.0));

    let red = (red as u16).clamp(0,255) as u8;
    let green = (green as u16).clamp(0,255) as u8;
    let blue = (blue as u16).clamp(0,255) as u8;


    (red,green,blue)
}

fn clut_calc8(lut:&Mft1,y:u8,m:u8,c:u8,k:u8) -> (u8,u8,u8) {
    let grid_points = lut.number_of_clut_grid_points as usize;
    println!("clut size {} {}",grid_points.pow(4),lut.clut_values.len()/3) ;
    println!("input {} {} {} {}",y,m,c,k);

    let y = lut.input_table [y as usize];
    let m = lut.input_table [m as usize + 256];
    let c = lut.input_table [c as usize + 256 * 2];
    let k = lut.input_table [k as usize + 256 * 3];

    let y_grid = ((y as f32 / 256.0) * grid_points as f32).floor() as usize;
    let m_grid = ((m as f32 / 256.0) * grid_points as f32).floor() as usize;
    let c_grid = ((c as f32 / 256.0) * grid_points as f32).floor() as usize;
    let k_grid = ((k as f32 / 256.0) * grid_points as f32).floor() as usize;
    let y_grid_delta = ((y as f32 / 256.0) * grid_points as f32) - y_grid as f32;
    let m_grid_delta = ((m as f32 / 256.0) * grid_points as f32) - m_grid as f32;
    let c_grid_delta = ((c as f32 / 256.0) * grid_points as f32) - c_grid as f32;
    let k_grid_delta = ((k as f32 / 256.0) * grid_points as f32) - k_grid as f32;
    let grid_delta = y_grid_delta * m_grid_delta * c_grid_delta * k_grid_delta;

    let grid = y_grid * grid_points.pow(3) + m_grid * grid_points.pow(2)  + c_grid * grid_points + k_grid;
    let grid = grid * lut.output_channels as usize;
    let y_grid2 = if y_grid < grid_points - 1 {y_grid + 1} else {y_grid}; 
    let m_grid2 = if m_grid < grid_points - 1 {m_grid + 1} else {m_grid}; 
    let c_grid2 = if c_grid < grid_points - 1 {c_grid + 1} else {c_grid}; 
    let k_grid2 = if k_grid < grid_points - 1 {k_grid + 1} else {k_grid}; 
    let grid2 = y_grid2 * grid_points.pow(3) + m_grid2 * grid_points.pow(2)  + c_grid2 * grid_points + k_grid2;
    let grid2 = grid2 * lut.output_channels as usize;
 
    let cy = lut.clut_values[grid]   as f32 * (1.0  -grid_delta) + lut.clut_values[grid2]   as f32 * grid_delta;
    let cr = lut.clut_values[grid+1] as f32 * (1.0  -grid_delta) + lut.clut_values[grid2+1] as f32 * grid_delta;
    let cb = lut.clut_values[grid+2] as f32 * (1.0  -grid_delta) + lut.clut_values[grid2+2] as f32 * grid_delta;
    let cy = lut.output_table[(cy as usize).clamp(0,255)] as f32;
    let cr = lut.output_table[(cr as usize + 256).clamp(0,255)] as f32;
    let cb = lut.output_table[(cb as usize + 512).clamp(0,255)] as f32;

    let crr = 1.402;
    let cbg = - 0.34414;
    let crg = - 0.71414;
    let cbb = 1.77;


    let red  = cy + 16.0 + (crr * (cr - 128.0));
    let green= cy + 16.0 + (cbg * (cb - 128.0) + crg * (cr - 128.0));
    let blue = cy + 16.0 + (cbb * (cb - 128.0));

    let red = (red as u16).clamp(0,255) as u8;
    let green = (green as u16).clamp(0,255) as u8;
    let blue = (blue as u16).clamp(0,255) as u8;

    (red,green,blue)
}

pub fn main() -> std::io::Result<()> {
    let mut is_fast = true;
    for argument in env::args() {
        if is_fast {
            is_fast = false;
            continue
        }
        println!("{}",argument);
        let icc_profile = crate::utils::load(argument)?;
        let decoded = DecodedICCProfile::new(&icc_profile.data)?;
        println!("{}",decoded_print(&decoded, 0)?);
/*        
        let (y,m,c,k) = (0,0,0,0);
        let mut converter = &Data::None;
        if decoded.color_space == 0x434d594b { //YMCK
            let lut = decoded.tags.get("A2B0");
            if let Some(lut2) = lut {
                match lut2 {
                    Data::Lut8(lut) => {
                        converter = lut2;
                        let (r,g,b) = clut_calc8(lut,y,m,c,k);
                    },
                    Data::Lut16(lut) => {
                        converter = lut2;
                        let (red,green,blue) = clut_calc16(lut,y,m,c,k);
                    },
                    _ => {
    
                    }
    
                }
            }

        }
*/
    }
    Ok(())
}