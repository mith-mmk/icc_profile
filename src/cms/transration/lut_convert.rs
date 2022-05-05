use crate::iccprofile::ICCNumber;
use crate::{Mft1,Mft2};

pub fn d4_to_d3_lut16(c1:u8,c2:u8,c3:u8,c4:u8,lut:&Mft2) -> (f64,f64,f64) {
    let grid_points = lut.number_of_clut_grid_points as usize;

    let delta= 65535 / (lut.input_table_enteries as usize) ;
    let c1 = (c1 as usize * 256 + c1 as usize) / lut.input_table_enteries as usize;
    let c1_delta= (c1 as usize * 256 + c1 as usize) % lut.input_table_enteries as usize;
    let c2 = (c2 as usize * 256 + c2 as usize) / lut.input_table_enteries as usize;
    let c2_delta= (c2 as usize * 256 + c2 as usize) % lut.input_table_enteries as usize;
    let c3 = (c3 as usize * 256 + c3 as usize) / lut.input_table_enteries as usize;
    let c3_delta= (c3 as usize * 256 + c3 as usize) % lut.input_table_enteries as usize;
    let c4 = (c4 as usize * 256 + c4 as usize) / lut.input_table_enteries as usize;
    let c4_delta= (c4 as usize * 256 + c4 as usize) % lut.input_table_enteries as usize;

    let c11 = lut.input_table [c1 as usize];
    let c21 = lut.input_table [c2 as usize + lut.input_table_enteries as usize];
    let c31 = lut.input_table [c3 as usize + lut.input_table_enteries as usize * 2];
    let c41 = lut.input_table [c4 as usize + lut.input_table_enteries as usize * 3];

    let c12 = if c1 < lut.input_table_enteries as usize - 1 {lut.input_table [c1 as usize + 1]} else {c11};
    let c22 = if c2 < lut.input_table_enteries as usize - 1 {lut.input_table [c2 as usize + 1 + lut.input_table_enteries as usize]} else {c21};
    let c32 = if c3 < lut.input_table_enteries as usize - 1 {lut.input_table [c3 as usize + 1 + lut.input_table_enteries as usize * 2]} else {c31};
    let c42 = if c4 < lut.input_table_enteries as usize - 1 {lut.input_table [c4 as usize + 1 + lut.input_table_enteries as usize * 3]} else {c41};

    let c1_delta= c1_delta as f64 / delta as f64;
    let c2_delta= c2_delta as f64 / delta as f64;
    let c3_delta= c3_delta as f64 / delta as f64;
    let c4_delta= c4_delta as f64 / delta as f64;

    let c1 = c11 as isize + ((c12 as isize - c11 as isize) as f64 * c1_delta) as isize;
    let c2 = c21 as isize + ((c22 as isize - c21 as isize) as f64 * c2_delta) as isize;
    let c3 = c31 as isize + ((c32 as isize - c31 as isize) as f64 * c3_delta) as isize;
    let c4 = c41 as isize + ((c42 as isize - c41 as isize) as f64 * c4_delta) as isize;

    let c1_grid = ((c1 as f64 / 65536.0) * grid_points as f64).floor() as usize;
    let c2_grid = ((c2 as f64 / 65536.0) * grid_points as f64).floor() as usize;
    let c3_grid = ((c3 as f64 / 65536.0) * grid_points as f64).floor() as usize;
    let c4_grid = ((c4 as f64 / 65536.0) * grid_points as f64).floor() as usize;

    let c1_grid_delta= ((c1 as f64 / 65536.0) * grid_points as f64) - c1_grid as f64;
    let c2_grid_delta= ((c2 as f64 / 65536.0) * grid_points as f64) - c2_grid as f64;
    let c3_grid_delta= ((c3 as f64 / 65536.0) * grid_points as f64) - c3_grid as f64;
    let c4_grid_delta= ((c4 as f64 / 65536.0) * grid_points as f64) - c4_grid as f64;
    let grid_delta= c1_grid_delta* c2_grid_delta* c3_grid_delta* c4_grid_delta;

    let grid = c1_grid * grid_points.pow(3) + c2_grid * grid_points.pow(2)  + c3_grid * grid_points + c4_grid;
    let grid = grid * lut.output_channels as usize;
    let c1_grid2 = if c1_grid < grid_points - 1 {c1_grid + 1} else {c1_grid}; 
    let c2_grid2 = if c2_grid < grid_points - 1 {c2_grid + 1} else {c2_grid}; 
    let c3_grid2 = if c3_grid < grid_points - 1 {c3_grid + 1} else {c3_grid}; 
    let c4_grid2 = if c4_grid < grid_points - 1 {c4_grid + 1} else {c4_grid}; 
    let grid2 = c1_grid2 * grid_points.pow(3) + c2_grid2 * grid_points.pow(2)  + c3_grid2 * grid_points + c4_grid2;
    let grid2 = grid2 * lut.output_channels as usize;
 
    let d1 = lut.clut_values[grid]   as f64 * (1.0  -grid_delta) + lut.clut_values[grid2]   as f64 * grid_delta;
    let d2 = lut.clut_values[grid+1] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+1] as f64 * grid_delta;
    let d3 = lut.clut_values[grid+2] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+2] as f64 * grid_delta;
    
    let dev = 65535.0 / (lut.output_table_enteries as f64 -1.0);

    let od1 = (d1 / dev) as usize;
    let od2 = (d2 / dev) as usize;
    let od3 = (d3 / dev) as usize;

    let od1_delta= d1 / dev - od1 as f64;
    let od2_delta= d2 / dev - od2 as f64;
    let od3_delta= d3 / dev - od3 as f64;

    let d11 = lut.output_table [od1 as usize];
    let d21 = lut.output_table [od2 as usize + lut.output_table_enteries as usize];
    let d31 = lut.output_table [od3 as usize + lut.output_table_enteries as usize * 2];
    let d12 = if d11 < lut.output_table_enteries - 1 {lut.output_table[od1 as usize + 1]} else {d11};
    let d22 = if d21 < lut.output_table_enteries - 1 {lut.output_table[od2 as usize + 1 + lut.output_table_enteries as usize]} else {d21};
    let d32 = if d31 < lut.output_table_enteries - 1 {lut.output_table[od3 as usize + 1 + lut.output_table_enteries as usize * 2]} else {d31};

    let d1 = d11 as f64 * (1.0 - od1_delta) + d12 as f64 * od1_delta;
    let d2 = d21 as f64 * (1.0 - od2_delta) + d22 as f64 * od2_delta;
    let d3 = d31 as f64 * (1.0 - od3_delta) + d32 as f64 * od3_delta;

    (d1,d2,d3)
}


pub fn d4_to_d3_lut8(c:u8,m:u8,y:u8,k:u8,lut:&Mft1) -> (u8,u8,u8) {
    let grid_points = lut.number_of_clut_grid_points as usize;

    let c = lut.input_table [c as usize];
    let m = lut.input_table [m as usize + 256];
    let y = lut.input_table [y as usize + 256 * 2];
    let k = lut.input_table [k as usize + 256 * 3];

    let c_grid = ((y as f64 / 255.0) * grid_points as f64).floor() as usize;
    let m_grid = ((m as f64 / 255.0) * grid_points as f64).floor() as usize;
    let y_grid = ((c as f64 / 255.0) * grid_points as f64).floor() as usize;
    let k_grid = ((k as f64 / 255.0) * grid_points as f64).floor() as usize;
    let c_grid_delta = ((y as f64 / 255.0) * grid_points as f64) - c_grid as f64;
    let m_grid_delta = ((m as f64 / 255.0) * grid_points as f64) - m_grid as f64;
    let y_grid_delta = ((c as f64 / 255.0) * grid_points as f64) - y_grid as f64;
    let k_grid_delta = ((k as f64 / 255.0) * grid_points as f64) - k_grid as f64;
    let grid_delta = c_grid_delta * m_grid_delta * y_grid_delta * k_grid_delta;

    let grid = c_grid * grid_points.pow(3) + m_grid * grid_points.pow(2)  + y_grid * grid_points + k_grid;
    let grid = grid * lut.output_channels as usize;
    let c_grid2 = if c_grid < grid_points - 1 {c_grid + 1} else {c_grid}; 
    let m_grid2 = if m_grid < grid_points - 1 {m_grid + 1} else {m_grid}; 
    let y_grid2 = if y_grid < grid_points - 1 {y_grid + 1} else {y_grid}; 
    let k_grid2 = if k_grid < grid_points - 1 {k_grid + 1} else {k_grid}; 
    let grid2 = c_grid2 * grid_points.pow(3) + m_grid2 * grid_points.pow(2)  + y_grid2 * grid_points + k_grid2;
    let grid2 = grid2 * lut.output_channels as usize;
 
    let d1 = lut.clut_values[grid]   as f64 * (1.0  -grid_delta) + lut.clut_values[grid2]   as f64 * grid_delta;
    let d2 = lut.clut_values[grid+1] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+1] as f64 * grid_delta;
    let d3 = lut.clut_values[grid+2] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+2] as f64 * grid_delta;
    let d1 = lut.output_table[(d1 as usize).clamp(0,255)];
    let d2 = lut.output_table[(d2 as usize).clamp(0,255) + 256];
    let d3 = lut.output_table[(d3 as usize).clamp(0,255) + 512];

    (d1,d2,d3)
}


pub fn d3_to_d4_lut16(c1:u8,c2:u8,c3:u8,lut:&Mft2) -> (f64,f64,f64,f64) {
    let c1 = c1 as f64;
    let c2 = c2 as f64;
    let c3 = c3 as f64;

    let e = &lut.e_params;

    let c1_1 = e[0].as_f64() * c1 + e[1].as_f64() * c2 + e[2].as_f64() * c3;
    let c2_1 = e[3].as_f64() * c1 + e[4].as_f64() * c2 + e[5].as_f64() * c3;
    let c3_1 = e[6].as_f64() * c1 + e[7].as_f64() * c2 + e[8].as_f64() * c3;

    let (c1,c2,c3) = (c1_1,c2_1,c3_1);


    let grid_points = lut.number_of_clut_grid_points as usize;

    let delta= 65535 / (lut.input_table_enteries as usize) ;
    let c1 = (c1 as usize * 256 + c1 as usize) / lut.input_table_enteries as usize;
    let c1_delta= (c1 as usize * 256 + c1 as usize) % lut.input_table_enteries as usize;
    let c2 = (c2 as usize * 256 + c2 as usize) / lut.input_table_enteries as usize;
    let c2_delta= (c2 as usize * 256 + c2 as usize) % lut.input_table_enteries as usize;
    let c3 = (c3 as usize * 256 + c3 as usize) / lut.input_table_enteries as usize;
    let c3_delta= (c3 as usize * 256 + c3 as usize) % lut.input_table_enteries as usize;

    let c11 = lut.input_table [c1 as usize];
    let c21 = lut.input_table [c2 as usize + lut.input_table_enteries as usize];
    let c31 = lut.input_table [c3 as usize + lut.input_table_enteries as usize * 2];

    let c12 = if c1 < lut.input_table_enteries as usize - 1 {lut.input_table [c1 as usize + 1]} else {c11};
    let c22 = if c2 < lut.input_table_enteries as usize - 1 {lut.input_table [c2 as usize + 1 + lut.input_table_enteries as usize]} else {c21};
    let c32 = if c3 < lut.input_table_enteries as usize - 1 {lut.input_table [c3 as usize + 1 + lut.input_table_enteries as usize * 2]} else {c31};

    let c1_delta= c1_delta as f64 / delta as f64;
    let c2_delta= c2_delta as f64 / delta as f64;
    let c3_delta= c3_delta as f64 / delta as f64;

    let c1 = c11 as isize + ((c12 as isize - c11 as isize) as f64 * c1_delta) as isize;
    let c2 = c21 as isize + ((c22 as isize - c21 as isize) as f64 * c2_delta) as isize;
    let c3 = c31 as isize + ((c32 as isize - c31 as isize) as f64 * c3_delta) as isize;

    let c1_grid = ((c1 as f64 / 65536.0) * grid_points as f64).floor() as usize;
    let c2_grid = ((c2 as f64 / 65536.0) * grid_points as f64).floor() as usize;
    let c3_grid = ((c3 as f64 / 65536.0) * grid_points as f64).floor() as usize;

    let c1_grid_delta= ((c1 as f64 / 65536.0) * grid_points as f64) - c1_grid as f64;
    let c2_grid_delta= ((c2 as f64 / 65536.0) * grid_points as f64) - c2_grid as f64;
    let c3_grid_delta= ((c3 as f64 / 65536.0) * grid_points as f64) - c3_grid as f64;
    let grid_delta= c1_grid_delta* c2_grid_delta* c3_grid_delta;

    let grid = c1_grid * grid_points.pow(2) + c2_grid + c3_grid;
    let grid = grid * lut.output_channels as usize;
    let c1_grid2 = if c1_grid < grid_points - 1 {c1_grid + 1} else {c1_grid}; 
    let c2_grid2 = if c2_grid < grid_points - 1 {c2_grid + 1} else {c2_grid}; 
    let c3_grid2 = if c3_grid < grid_points - 1 {c3_grid + 1} else {c3_grid}; 
    let grid2 = c1_grid2 * grid_points.pow(2) + c2_grid2 * grid_points + c3_grid2;
    let grid2 = grid2 * lut.output_channels as usize;
 
    let d1 = lut.clut_values[grid]   as f64 * (1.0  -grid_delta) + lut.clut_values[grid2]   as f64 * grid_delta;
    let d2 = lut.clut_values[grid+1] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+1] as f64 * grid_delta;
    let d3 = lut.clut_values[grid+2] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+2] as f64 * grid_delta;
    let d4 = lut.clut_values[grid+3] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+3] as f64 * grid_delta;
    
    let dev = 65535.0 / (lut.output_table_enteries as f64 -1.0);

    let od1 = (d1 / dev) as usize;
    let od2 = (d2 / dev) as usize;
    let od3 = (d3 / dev) as usize;
    let od4 = (d4 / dev) as usize;

    let od1_delta= d1 / dev - od1 as f64;
    let od2_delta= d2 / dev - od2 as f64;
    let od3_delta= d3 / dev - od3 as f64;
    let od4_delta= d4 / dev - od4 as f64;

    let d11 = lut.output_table [od1 as usize];
    let d21 = lut.output_table [od2 as usize + lut.output_table_enteries as usize];
    let d31 = lut.output_table [od3 as usize + lut.output_table_enteries as usize * 2];
    let d41 = lut.output_table [od3 as usize + lut.output_table_enteries as usize * 3];

    let d12 = if d11 < lut.output_table_enteries - 1 {lut.output_table[od1 as usize + 1]} else {d11};
    let d22 = if d21 < lut.output_table_enteries - 1 {lut.output_table[od2 as usize + 1 + lut.output_table_enteries as usize]} else {d21};
    let d32 = if d31 < lut.output_table_enteries - 1 {lut.output_table[od3 as usize + 1 + lut.output_table_enteries as usize * 2]} else {d31};
    let d42 = if d41 < lut.output_table_enteries - 1 {lut.output_table[od4 as usize + 1 + lut.output_table_enteries as usize * 3]} else {d41};

    let d1 = d11 as f64 * (1.0 - od1_delta) + d12 as f64 * od1_delta;
    let d2 = d21 as f64 * (1.0 - od2_delta) + d22 as f64 * od2_delta;
    let d3 = d31 as f64 * (1.0 - od3_delta) + d32 as f64 * od3_delta;
    let d4 = d41 as f64 * (1.0 - od4_delta) + d42 as f64 * od4_delta;

    (d1,d2,d3,d4)
}

pub fn d3_to_d3_lut16(c1:u8,c2:u8,c3:u8,lut:&Mft2) -> (f64,f64,f64) {
    let c1 = c1 as f64;
    let c2 = c2 as f64;
    let c3 = c3 as f64;

    let e = &lut.e_params;

    let c1_1 = e[0].as_f64() * c1 + e[1].as_f64() * c2 + e[2].as_f64() * c3;
    let c2_1 = e[3].as_f64() * c1 + e[4].as_f64() * c2 + e[5].as_f64() * c3;
    let c3_1 = e[6].as_f64() * c1 + e[7].as_f64() * c2 + e[8].as_f64() * c3;

    let (c1,c2,c3) = (c1_1,c2_1,c3_1);


    let grid_points = lut.number_of_clut_grid_points as usize;

    let delta= 65535 / (lut.input_table_enteries as usize) ;
    let c1 = (c1 as usize * 256 + c1 as usize) / lut.input_table_enteries as usize;
    let c1_delta= (c1 as usize * 256 + c1 as usize) % lut.input_table_enteries as usize;
    let c2 = (c2 as usize * 256 + c2 as usize) / lut.input_table_enteries as usize;
    let c2_delta= (c2 as usize * 256 + c2 as usize) % lut.input_table_enteries as usize;
    let c3 = (c3 as usize * 256 + c3 as usize) / lut.input_table_enteries as usize;
    let c3_delta= (c3 as usize * 256 + c3 as usize) % lut.input_table_enteries as usize;

    let c11 = lut.input_table [c1 as usize];
    let c21 = lut.input_table [c2 as usize + lut.input_table_enteries as usize];
    let c31 = lut.input_table [c3 as usize + lut.input_table_enteries as usize * 2];

    let c12 = if c1 < lut.input_table_enteries as usize - 1 {lut.input_table [c1 as usize + 1]} else {c11};
    let c22 = if c2 < lut.input_table_enteries as usize - 1 {lut.input_table [c2 as usize + 1 + lut.input_table_enteries as usize]} else {c21};
    let c32 = if c3 < lut.input_table_enteries as usize - 1 {lut.input_table [c3 as usize + 1 + lut.input_table_enteries as usize * 2]} else {c31};

    let c1_delta= c1_delta as f64 / delta as f64;
    let c2_delta= c2_delta as f64 / delta as f64;
    let c3_delta= c3_delta as f64 / delta as f64;

    let c1 = c11 as isize + ((c12 as isize - c11 as isize) as f64 * c1_delta) as isize;
    let c2 = c21 as isize + ((c22 as isize - c21 as isize) as f64 * c2_delta) as isize;
    let c3 = c31 as isize + ((c32 as isize - c31 as isize) as f64 * c3_delta) as isize;

    let c1_grid = ((c1 as f64 / 65536.0) * grid_points as f64).floor() as usize;
    let c2_grid = ((c2 as f64 / 65536.0) * grid_points as f64).floor() as usize;
    let c3_grid = ((c3 as f64 / 65536.0) * grid_points as f64).floor() as usize;

    let c1_grid_delta= ((c1 as f64 / 65536.0) * grid_points as f64) - c1_grid as f64;
    let c2_grid_delta= ((c2 as f64 / 65536.0) * grid_points as f64) - c2_grid as f64;
    let c3_grid_delta= ((c3 as f64 / 65536.0) * grid_points as f64) - c3_grid as f64;
    let grid_delta= c1_grid_delta* c2_grid_delta* c3_grid_delta;

    let grid = c1_grid * grid_points.pow(2) + c2_grid + c3_grid;
    let grid = grid * lut.output_channels as usize;
    let c1_grid2 = if c1_grid < grid_points - 1 {c1_grid + 1} else {c1_grid}; 
    let c2_grid2 = if c2_grid < grid_points - 1 {c2_grid + 1} else {c2_grid}; 
    let c3_grid2 = if c3_grid < grid_points - 1 {c3_grid + 1} else {c3_grid}; 
    let grid2 = c1_grid2 * grid_points.pow(2) + c2_grid2 * grid_points + c3_grid2;
    let grid2 = grid2 * lut.output_channels as usize;
 
    let d1 = lut.clut_values[grid]   as f64 * (1.0  -grid_delta) + lut.clut_values[grid2]   as f64 * grid_delta;
    let d2 = lut.clut_values[grid+1] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+1] as f64 * grid_delta;
    let d3 = lut.clut_values[grid+2] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+2] as f64 * grid_delta;
    
    let dev = 65535.0 / (lut.output_table_enteries as f64 -1.0);

    let od1 = (d1 / dev) as usize;
    let od2 = (d2 / dev) as usize;
    let od3 = (d3 / dev) as usize;
 
    let od1_delta= d1 / dev - od1 as f64;
    let od2_delta= d2 / dev - od2 as f64;
    let od3_delta= d3 / dev - od3 as f64;

    let d11 = lut.output_table [od1 as usize];
    let d21 = lut.output_table [od2 as usize + lut.output_table_enteries as usize];
    let d31 = lut.output_table [od3 as usize + lut.output_table_enteries as usize * 2];

    let d12 = if d11 < lut.output_table_enteries - 1 {lut.output_table[od1 as usize + 1]} else {d11};
    let d22 = if d21 < lut.output_table_enteries - 1 {lut.output_table[od2 as usize + 1 + lut.output_table_enteries as usize]} else {d21};
    let d32 = if d31 < lut.output_table_enteries - 1 {lut.output_table[od3 as usize + 1 + lut.output_table_enteries as usize * 2]} else {d31};

    let d1 = d11 as f64 * (1.0 - od1_delta) + d12 as f64 * od1_delta;
    let d2 = d21 as f64 * (1.0 - od2_delta) + d22 as f64 * od2_delta;
    let d3 = d31 as f64 * (1.0 - od3_delta) + d32 as f64 * od3_delta;

    (d1,d2,d3)
}

pub fn d3_to_d1_lut16(c1:u8,c2:u8,c3:u8,lut:&Mft2) -> f64 {
    let c1 = c1 as f64;
    let c2 = c2 as f64;
    let c3 = c3 as f64;

    let e = &lut.e_params;

    let c1_1 = e[0].as_f64() * c1 + e[1].as_f64() * c2 + e[2].as_f64() * c3;
    let c2_1 = e[3].as_f64() * c1 + e[4].as_f64() * c2 + e[5].as_f64() * c3;
    let c3_1 = e[6].as_f64() * c1 + e[7].as_f64() * c2 + e[8].as_f64() * c3;

    let (c1,c2,c3) = (c1_1,c2_1,c3_1);


    let grid_points = lut.number_of_clut_grid_points as usize;

    let delta= 65535 / (lut.input_table_enteries as usize) ;
    let c1 = (c1 as usize * 256 + c1 as usize) / lut.input_table_enteries as usize;
    let c1_delta= (c1 as usize * 256 + c1 as usize) % lut.input_table_enteries as usize;
    let c2 = (c2 as usize * 256 + c2 as usize) / lut.input_table_enteries as usize;
    let c2_delta= (c2 as usize * 256 + c2 as usize) % lut.input_table_enteries as usize;
    let c3 = (c3 as usize * 256 + c3 as usize) / lut.input_table_enteries as usize;
    let c3_delta= (c3 as usize * 256 + c3 as usize) % lut.input_table_enteries as usize;

    let c11 = lut.input_table [c1 as usize];
    let c21 = lut.input_table [c2 as usize + lut.input_table_enteries as usize];
    let c31 = lut.input_table [c3 as usize + lut.input_table_enteries as usize * 2];

    let c12 = if c1 < lut.input_table_enteries as usize - 1 {lut.input_table [c1 as usize + 1]} else {c11};
    let c22 = if c2 < lut.input_table_enteries as usize - 1 {lut.input_table [c2 as usize + 1 + lut.input_table_enteries as usize]} else {c21};
    let c32 = if c3 < lut.input_table_enteries as usize - 1 {lut.input_table [c3 as usize + 1 + lut.input_table_enteries as usize * 2]} else {c31};

    let c1_delta= c1_delta as f64 / delta as f64;
    let c2_delta= c2_delta as f64 / delta as f64;
    let c3_delta= c3_delta as f64 / delta as f64;

    let c1 = c11 as isize + ((c12 as isize - c11 as isize) as f64 * c1_delta) as isize;
    let c2 = c21 as isize + ((c22 as isize - c21 as isize) as f64 * c2_delta) as isize;
    let c3 = c31 as isize + ((c32 as isize - c31 as isize) as f64 * c3_delta) as isize;

    let c1_grid = ((c1 as f64 / 65536.0) * grid_points as f64).floor() as usize;
    let c2_grid = ((c2 as f64 / 65536.0) * grid_points as f64).floor() as usize;
    let c3_grid = ((c3 as f64 / 65536.0) * grid_points as f64).floor() as usize;

    let c1_grid_delta= ((c1 as f64 / 65536.0) * grid_points as f64) - c1_grid as f64;
    let c2_grid_delta= ((c2 as f64 / 65536.0) * grid_points as f64) - c2_grid as f64;
    let c3_grid_delta= ((c3 as f64 / 65536.0) * grid_points as f64) - c3_grid as f64;
    let grid_delta= c1_grid_delta* c2_grid_delta* c3_grid_delta;

    let grid = c1_grid * grid_points.pow(2) + c2_grid + c3_grid;
    let grid = grid * lut.output_channels as usize;
    let c1_grid2 = if c1_grid < grid_points - 1 {c1_grid + 1} else {c1_grid}; 
    let c2_grid2 = if c2_grid < grid_points - 1 {c2_grid + 1} else {c2_grid}; 
    let c3_grid2 = if c3_grid < grid_points - 1 {c3_grid + 1} else {c3_grid}; 
    let grid2 = c1_grid2 * grid_points.pow(2) + c2_grid2 * grid_points + c3_grid2;
    let grid2 = grid2 * lut.output_channels as usize;
 
    let d1 = lut.clut_values[grid]   as f64 * (1.0  -grid_delta) + lut.clut_values[grid2]   as f64 * grid_delta;
    
    let dev = 65535.0 / (lut.output_table_enteries as f64 -1.0);

    let od1 = (d1 / dev) as usize;

    let od1_delta= d1 / dev - od1 as f64;

    let d11 = lut.output_table [od1 as usize];

    let d12 = if d11 < lut.output_table_enteries - 1 {lut.output_table[od1 as usize + 1]} else {d11};

    let d1 = d11 as f64 * (1.0 - od1_delta) + d12 as f64 * od1_delta;

    d1
}

pub fn d3_to_d4_lut8(c1:u8,c2:u8,c3:u8,lut:&Mft1) -> (u8,u8,u8,u8) {
    let grid_points = lut.number_of_clut_grid_points as usize;
    let e = &lut.e_params;

    let c1_1 = e[0].as_f64() * c1 as f64 + e[1].as_f64() * c2 as f64 + e[2].as_f64() * c3 as f64;
    let c2_1 = e[3].as_f64() * c1 as f64 + e[4].as_f64() * c2 as f64 + e[5].as_f64() * c3 as f64;
    let c3_1 = e[6].as_f64() * c1 as f64 + e[7].as_f64() * c2 as f64 + e[8].as_f64() * c3 as f64;

    let c1 = c1_1 as usize;
    let c2 = c2_1 as usize;
    let c3 = c3_1 as usize;

    let c1 = lut.input_table [c1 as usize];
    let c2 = lut.input_table [c2 as usize + 256];
    let c3 = lut.input_table [c3 as usize + 256 * 2];

    let c1_grid = ((c1 as f64 / 255.0) * grid_points as f64).floor() as usize;
    let c2_grid = ((c2 as f64 / 255.0) * grid_points as f64).floor() as usize;
    let c3_grid = ((c3 as f64 / 255.0) * grid_points as f64).floor() as usize;
    let c1_grid_delta = ((c1 as f64 / 255.0) * grid_points as f64) - c1_grid as f64;
    let c2_grid_delta = ((c2 as f64 / 255.0) * grid_points as f64) - c2_grid as f64;
    let c3_grid_delta = ((c3 as f64 / 255.0) * grid_points as f64) - c3_grid as f64;
    let grid_delta = c1_grid_delta * c2_grid_delta * c3_grid_delta;

    let grid = c1_grid * grid_points.pow(2) + c2_grid * grid_points + c3_grid;
    let grid = grid * lut.output_channels as usize;
    let c1_grid2 = if c1_grid < grid_points - 1 {c1_grid + 1} else {c1_grid}; 
    let c2_grid2 = if c2_grid < grid_points - 1 {c2_grid + 1} else {c2_grid}; 
    let c3_grid2 = if c3_grid < grid_points - 1 {c3_grid + 1} else {c3_grid}; 
    let grid2 = c1_grid2 * grid_points.pow(2)  + c2_grid2 * grid_points + c3_grid2;
    let grid2 = grid2 * lut.output_channels as usize;
 
    let d1 = lut.clut_values[grid]   as f64 * (1.0  -grid_delta) + lut.clut_values[grid2]   as f64 * grid_delta;
    let d2 = lut.clut_values[grid+1] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+1] as f64 * grid_delta;
    let d3 = lut.clut_values[grid+2] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+2] as f64 * grid_delta;
    let d4 = lut.clut_values[grid+3] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+3] as f64 * grid_delta;

    let d1 = lut.output_table[(d1 as usize).clamp(0,255)];
    let d2 = lut.output_table[(d2 as usize).clamp(0,255) + 256];
    let d3 = lut.output_table[(d3 as usize).clamp(0,255) + 512];
    let d4 = lut.output_table[(d4 as usize).clamp(0,255) + 768];

    (d1,d2,d3,d4)
}

pub fn d3_to_d3_lut8(c1:u8,c2:u8,c3:u8,lut:&Mft1) -> (u8,u8,u8) {
    let grid_points = lut.number_of_clut_grid_points as usize;
    let e = &lut.e_params;

    let c1_1 = e[0].as_f64() * c1 as f64 + e[1].as_f64() * c2 as f64 + e[2].as_f64() * c3 as f64;
    let c2_1 = e[3].as_f64() * c1 as f64 + e[4].as_f64() * c2 as f64 + e[5].as_f64() * c3 as f64;
    let c3_1 = e[6].as_f64() * c1 as f64 + e[7].as_f64() * c2 as f64 + e[8].as_f64() * c3 as f64;

    let c1 = c1_1 as usize;
    let c2 = c2_1 as usize;
    let c3 = c3_1 as usize;

    let c1 = lut.input_table [c1 as usize];
    let c2 = lut.input_table [c2 as usize + 256];
    let c3 = lut.input_table [c3 as usize + 256 * 2];

    let c1_grid = ((c1 as f64 / 255.0) * grid_points as f64).floor() as usize;
    let c2_grid = ((c2 as f64 / 255.0) * grid_points as f64).floor() as usize;
    let c3_grid = ((c3 as f64 / 255.0) * grid_points as f64).floor() as usize;
    let c1_grid_delta = ((c1 as f64 / 255.0) * grid_points as f64) - c1_grid as f64;
    let c2_grid_delta = ((c2 as f64 / 255.0) * grid_points as f64) - c2_grid as f64;
    let c3_grid_delta = ((c3 as f64 / 255.0) * grid_points as f64) - c3_grid as f64;
    let grid_delta = c1_grid_delta * c2_grid_delta * c3_grid_delta;

    let grid = c1_grid * grid_points.pow(2) + c2_grid * grid_points + c3_grid;
    let grid = grid * lut.output_channels as usize;
    let c1_grid2 = if c1_grid < grid_points - 1 {c1_grid + 1} else {c1_grid}; 
    let c2_grid2 = if c2_grid < grid_points - 1 {c2_grid + 1} else {c2_grid}; 
    let c3_grid2 = if c3_grid < grid_points - 1 {c3_grid + 1} else {c3_grid}; 
    let grid2 = c1_grid2 * grid_points.pow(2)  + c2_grid2 * grid_points + c3_grid2;
    let grid2 = grid2 * lut.output_channels as usize;
 
    let d1 = lut.clut_values[grid]   as f64 * (1.0  -grid_delta) + lut.clut_values[grid2]   as f64 * grid_delta;
    let d2 = lut.clut_values[grid+1] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+1] as f64 * grid_delta;
    let d3 = lut.clut_values[grid+2] as f64 * (1.0  -grid_delta) + lut.clut_values[grid2+2] as f64 * grid_delta;

    let d1 = lut.output_table[(d1 as usize).clamp(0,255)];
    let d2 = lut.output_table[(d2 as usize).clamp(0,255) + 256];
    let d3 = lut.output_table[(d3 as usize).clamp(0,255) + 512];

    (d1,d2,d3)
}

pub fn d3_to_d1_lut8(c1:u8,c2:u8,c3:u8,lut:&Mft1) -> u8 {
    let grid_points = lut.number_of_clut_grid_points as usize;
    let e = &lut.e_params;

    let c1_1 = e[0].as_f64() * c1 as f64 + e[1].as_f64() * c2 as f64 + e[2].as_f64() * c3 as f64;
    let c2_1 = e[3].as_f64() * c1 as f64 + e[4].as_f64() * c2 as f64 + e[5].as_f64() * c3 as f64;
    let c3_1 = e[6].as_f64() * c1 as f64 + e[7].as_f64() * c2 as f64 + e[8].as_f64() * c3 as f64;

    let c1 = c1_1 as usize;
    let c2 = c2_1 as usize;
    let c3 = c3_1 as usize;

    let c1 = lut.input_table [c1 as usize];
    let c2 = lut.input_table [c2 as usize + 256];
    let c3 = lut.input_table [c3 as usize + 256 * 2];

    let c1_grid = ((c1 as f64 / 255.0) * grid_points as f64).floor() as usize;
    let c2_grid = ((c2 as f64 / 255.0) * grid_points as f64).floor() as usize;
    let c3_grid = ((c3 as f64 / 255.0) * grid_points as f64).floor() as usize;
    let c1_grid_delta = ((c1 as f64 / 255.0) * grid_points as f64) - c1_grid as f64;
    let c2_grid_delta = ((c2 as f64 / 255.0) * grid_points as f64) - c2_grid as f64;
    let c3_grid_delta = ((c3 as f64 / 255.0) * grid_points as f64) - c3_grid as f64;
    let grid_delta = c1_grid_delta * c2_grid_delta * c3_grid_delta;

    let grid = c1_grid * grid_points.pow(2) + c2_grid * grid_points + c3_grid;
    let grid = grid * lut.output_channels as usize;
    let c1_grid2 = if c1_grid < grid_points - 1 {c1_grid + 1} else {c1_grid}; 
    let c2_grid2 = if c2_grid < grid_points - 1 {c2_grid + 1} else {c2_grid}; 
    let c3_grid2 = if c3_grid < grid_points - 1 {c3_grid + 1} else {c3_grid}; 
    let grid2 = c1_grid2 * grid_points.pow(2)  + c2_grid2 * grid_points + c3_grid2;
    let grid2 = grid2 * lut.output_channels as usize;
 
    let d1 = lut.clut_values[grid]   as f64 * (1.0  -grid_delta) + lut.clut_values[grid2]   as f64 * grid_delta;

    
    let d1 = lut.output_table[(d1 as usize).clamp(0,255)];

    d1
}