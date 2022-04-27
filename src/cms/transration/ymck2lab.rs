// icc profile reader
pub use icc_profile::iccprofile::*;

pub enum YMCKLut{
    Lun16(Mft2),
    Lut8(Mft1),
    // LBtoA,
    // None,
}

// YMCK -> RGB conversion
pub fn ycmk_rgb_lut16(y:u8,m:u8,c:u8,k:u8,lut:&Mft2) -> (u8,u8,u8) {
    let grid_points = lut.number_of_clut_grid_points as usize;

    // e_params may not use,because there are for XYZ color space and 3x3 matrix,but the YCMK color space has four channels,

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

    let y2 = if y < lut.input_table_enteries as usize - 1 {lut.input_table [y as usize + 1]} else {y1};
    let m2 = if m < lut.input_table_enteries as usize - 1 {lut.input_table [m as usize + 1 + lut.input_table_enteries as usize]} else {m1};
    let c2 = if c < lut.input_table_enteries as usize - 1 {lut.input_table [c as usize + 1 + lut.input_table_enteries as usize * 2]} else {c1};
    let k2 = if k < lut.input_table_enteries as usize - 1 {lut.input_table [k as usize + 1 + lut.input_table_enteries as usize * 3]} else {k1};

    let y_delta = y_delta as f32 / delta as f32;
    let m_delta = m_delta as f32 / delta as f32;
    let c_delta = c_delta as f32 / delta as f32;
    let k_delta = k_delta as f32 / delta as f32;

    let y = y1 as isize + ((y2 as isize - y1 as isize) as f32 * y_delta) as isize;
    let m = m1 as isize + ((m2 as isize - m1 as isize) as f32 * m_delta) as isize;
    let c = c1 as isize + ((c2 as isize - c1 as isize) as f32 * c_delta) as isize;
    let k = k1 as isize + ((k2 as isize - k1 as isize) as f32 * k_delta) as isize;

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
 
    let l = lut.clut_values[grid]   as f32 * (1.0  -grid_delta) + lut.clut_values[grid2]   as f32 * grid_delta;
    let a = lut.clut_values[grid+1] as f32 * (1.0  -grid_delta) + lut.clut_values[grid2+1] as f32 * grid_delta;
    let b = lut.clut_values[grid+2] as f32 * (1.0  -grid_delta) + lut.clut_values[grid2+2] as f32 * grid_delta;
    
    let dev = 65536.0 / (lut.output_table_enteries as f32 -1.0);

    let ol = (l / dev) as usize;
    let oa = (a / dev) as usize;
    let ob = (b / dev) as usize;

    let ol_delta = l / dev - ol as f32;
    let oa_delta = a / dev - oa as f32;
    let ob_delta = b / dev - ob as f32;

    let l1 = lut.output_table [ol as usize] as usize;
    let a1 = lut.output_table [oa as usize + lut.output_table_enteries as usize] as usize;
    let b1 = lut.output_table [ob as usize + lut.output_table_enteries as usize * 2] as usize;
    let l2 = if l1 < lut.output_table_enteries as usize - 1 {lut.output_table[ol as usize + 1]} else {l1};
    let a2 = if a1 < lut.output_table_enteries as usize - 1 {lut.output_table[oa as usize + 1 + lut.output_table_enteries as usize]} else {a1};
    let b2 = if b1 < lut.output_table_enteries as usize - 1 {lut.output_table[ob as usize + 1 + lut.output_table_enteries as usize * 2]} else {b1};

    let l = l1 as f32 * (1.0 - ol_delta) + l2 as f32 * ol_delta;
    let a = a1 as f32 * (1.0 - oa_delta) + a2 as f32 * oa_delta;
    let b = b1 as f32 * (1.0 - ob_delta) + b2 as f32 * ob_delta;

    (l,a,b)
}

// no test
pub fn ycmk_rgb_lut8(y:u8,m:u8,c:u8,k:u8,lut:&Mft1) -> (u8,u8,u8) {
    let grid_points = lut.number_of_clut_grid_points as usize;

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
 
    let l = lut.clut_values[grid]   as f32 * (1.0  -grid_delta) + lut.clut_values[grid2]   as f32 * grid_delta;
    let a = lut.clut_values[grid+1] as f32 * (1.0  -grid_delta) + lut.clut_values[grid2+1] as f32 * grid_delta;
    let b = lut.clut_values[grid+2] as f32 * (1.0  -grid_delta) + lut.clut_values[grid2+2] as f32 * grid_delta;
    let l = lut.output_table[(l as usize).clamp(0,255)] as f32;
    let a = lut.output_table[(a as usize + 256).clamp(0,255)] as f32;
    let b = lut.output_table[(b  as usize + 512).clamp(0,255)] as f32;

    (l,a,b)
}

pub fn ymck_to_lab_entries (buf:&[u8],entries: usize,mode: &YMCKLut) -> Result<Vec<u8>> {
    if buf.len() < entries *4 {
        return Err(Error::new(ErrorKind::Other, "Data shotage"))
    }
    let matrix = mode.get();
    let mut buffer = Vec::with_capacity(entries * 3);

    for i in 0..entries {
        let ptr = i * 4;
        let x = buf[ptr];
        let y = buf[ptr + 1];
        let z = buf[ptr + 2];
        match mode {
            Lut16(lut) {
                let (r,g,b) = ycmk_to_lab_lut16(y,m,c,k,lut);
            },
            Lut8(lut) {
                let (r,g,b) = ycmk_to_lab_lut8(y,m,c,k,lut);
            },
            /*
            None {
                let (r,g,b) = ycmk_to_lab(y,m,c,k);

            }*/
        }


        buffer.push(r);
        buffer.push(g);
        buffer.push(b);

    }

    Ok(buffer)
}