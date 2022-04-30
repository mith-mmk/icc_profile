use crate::iccprofile::IICNumber;
use crate::S15Fixed16Number;

pub mod transration;

pub enum ColorEntries {
    Rgb24(Vec<u8>),
    Rgba32(Vec<u8>),
    Lab24(Vec<u8>),
    Lab(Vec<f32>),
    Lab48(Vec<u16>),
    Ymck(Vec<u8>),
    Xyz(Vec<f32>),
    Xyz24(Vec<u8>),
}

impl ColorEntries {
    pub fn to_rgb(&self) -> Option<Vec<u8>> {
        None
    }

    pub fn to_lab(&self) -> Option<Vec<f32>> {
        None
    }

    pub fn to_lab24(&self) -> Option<Vec<u8>> {
        None
    }

    pub fn to_lab48(&self) -> Option<Vec<u16>> {
        None
    }


    pub fn to_xyz(&self) -> Option<Vec<f32>> {
        None
    }

    pub fn to_xyz24(&self) -> Option<Vec<u8>> {
        None
    }

    pub fn to_ymck(&self) -> Option<Vec<f32>> {
        None
    }

}

#[derive(Clone)]
pub struct ColorMatrix3D {
    pub e: [f32;9]
}

#[derive(Clone)]
pub struct ColorMatrix3DWithShift {
    pub e: [f32;12]
}


impl ColorMatrix3D {
    pub fn cie_rgb_to_xyz() -> Self {
        Self {
            e : [0.4898, 0.3101, 0.2001, 0.1769, 0.8124, 0.0107, 0.0000, 0.0100, 0.9903]
        }
    }

    pub fn d65_rgb_to_xyz() -> Self {
        Self {
            e : [0.4124, 0.3576, 0.1805, 0.2126, 0.7152, 0.0722, 0.0193, 0.1192, 0.9505]
        }
    }
    pub fn c_rgb_to_xyz() -> Self {
        Self {
            e : [0.6069, 0.1735, 0.2003, 0.2989, 0.5866, 0.1144, 0.0000, 0.0661, 1.1157]
        }
    }

    pub fn adobe_rgb_to_xyz() -> Self {
        Self {
            e : [0.5778, 0.1825, 0.1902, 0.3070, 0.6170, 0.0761, 0.0181, 0.0695, 1.0015]
        }
    }
    pub fn ntsc_rgb_to_xyz() -> Self {
        Self {
            e : [0.6070, 0.1734, 0.2006, 0.2990, 0.5864, 0.1146, 0.0000, 0.0661, 1.1175]
        }
    }

    pub fn cie_rgb_to_rgb() -> Self {
        Self {
            e : [2.3655, - 0.8971, - 0.4683, -0.5151, 1.4264, 0.0887, 0.0052, - 0.0144, 1.0089]
        }
    }

    pub fn d65_rgb_to_rgb() -> Self {
        Self {
            e : [3.2410, - 1.5374, - 0.4986,  -0.9692, 1.8760,  0.0416, 0.0556, - 0.2040, 1.0507]
        }
    }

    pub fn c_rgb_to_rgb() -> Self {
        Self {
            e : [1.9099, - 0.5324, - 0.2882, -0.9846, 1.9991, -0.0283, 0.0583, - 0.1184, 0.8979]
        }
    }

    pub fn adobe_rgb_to_rgb() -> Self {
        Self {
            e : [2.0416, -0.5650, -0.3447, -1.0199, 1.9171, 0.0481, 0.0340, - 0.1229, 1.0014]
        }
    }

    pub fn ntsc_rgb_to_rgb() -> Self {
        Self {
            e : [1.9097, - 0.5324, - 0.2882, -0.9850, 1.9998, - 0.0283, 0.0582, - 0.1182, 0.8966]
        }
    }

    pub fn from(e:&[f32]) -> Option<Self> {
        if e.len() != 9 {None}
        else {
            Some(Self {
                e: [e[0],e[1],e[2],e[3],e[4],e[5],e[6],e[7],e[8]]
            })
        }
    }

    pub fn from_s15_fixed16_number(e:&[S15Fixed16Number])-> Option<Self> {
        if e.len() != 9 {None}
        else {
            Some(Self {
                e: [e[0].as_f32(),e[1].as_f32(),e[2].as_f32(),
                    e[3].as_f32(),e[4].as_f32(),e[5].as_f32(),
                    e[6].as_f32(),e[7].as_f32(),e[8].as_f32()]
            })
        }
    }

    pub fn invese(matrix:&Self) -> Option<Self> {
        let a11 = matrix.e[0];
        let a12 = matrix.e[1];
        let a13 = matrix.e[2];
        let a21 = matrix.e[3];
        let a22 = matrix.e[4];
        let a23 = matrix.e[5];
        let a31 = matrix.e[6];
        let a32 = matrix.e[7];
        let a33 = matrix.e[8];
        
        let delta =  a11 * a22 * a33 + a12 *a23 * a31 + a13 * a21 * a32
            - a13 * a22 * a31 - a11 * a23 * a32 - a12 * a21 *a33;
        if delta == 0.0 {
            None
        } else {
            Some(Self{
                e:[
                (a22 * a33 - a23 * a32) /delta,
                (a13 * a32 - a12 * a33) /delta,
                (a12 * a23 - a13 * a22) /delta,

                (a23 * a31 - a21 * a33) /delta,
                (a11 * a33 - a13 * a31) /delta,
                (a13 * a21 - a11 * a23) /delta,
                
                (a21 * a32 - a22 * a31) /delta,
                (a12 * a31 - a11 * a32) /delta,
                (a11 * a22 - a12 * a21) /delta
                ]
            })
        }
    }

    pub fn convert_3d_u8(&self,x:u8,y:u8,z:u8) -> (u8,u8,u8) {
        let e = self.e;
        let (x,y,z) = (x as f32,y as f32,z as f32);
        
        let a = x * e[0] + y * e[1] + z * e[2];    
        let b = x * e[3] + y * e[4] + z * e[5];    
        let c = x * e[6] + y * e[7] + z * e[8];    
        
        let a = (a as i16).clamp(0,255) as u8;
        let b = (b as i16).clamp(0,255) as u8;
        let c = (c as i16).clamp(0,255) as u8;
        (a,b,c)
    }

    pub fn convert_3d_u16(&self,x:u16,y:u16,z:u16) -> (u16,u16,u16) {
        let e = self.e;
        let (x,y,z) = (x as f32,y as f32,z as f32);
        
        let a = x * e[0] + y * e[1] + z * e[2];    
        let b = x * e[3] + y * e[4] + z * e[5];    
        let c = x * e[6] + y * e[7] + z * e[8];    
        
        let a = (a as i32).clamp(0,65535) as u16;
        let b = (b as i32).clamp(0,65535) as u16;
        let c = (c as i32).clamp(0,65535) as u16;
        (a,b,c)
    }

}