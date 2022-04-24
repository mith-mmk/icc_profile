use std::collections::HashMap;
use bin_rs::io::*;
use crate::iccprofile::Data::*;

pub fn icc_profile_decode(data :&Vec<u8>) -> DecodedICCProfile {
    let icc_profile = ICCProfile::new(data);
    let mut decoded: HashMap<String,Data> = HashMap::new();
    let header_size = 128;
    let mut ptr = header_size;
    let tags = read_u32_be(&icc_profile.data,ptr);
    ptr +=  4;
    for _ in 0..tags {
        let tag_name = read_string(&icc_profile.data,ptr,4);
        ptr +=  4;
        let tag_offset = read_u32_be(&icc_profile.data,ptr) as usize;
        ptr +=  4;
        let tag_length = read_u32_be(&icc_profile.data,ptr) as usize;
        ptr +=  4;
        let (_,val) = Data::parse(&icc_profile.data[tag_offset..],tag_length);
        decoded.insert(tag_name,val);
    }
    DecodedICCProfile {
        length : icc_profile.length, 
        cmmid : icc_profile.cmmid,
        version :icc_profile.version,
        device_class :icc_profile.device_class,
        color_space : icc_profile.color_space,
        pcs : icc_profile.pcs,
        create_date: icc_profile.create_date.clone(),
        magicnumber_ascp: icc_profile.magicnumber_ascp,
        platform: icc_profile.platform,
        flags: icc_profile.flags,
        manufacturer: icc_profile.manufacturer,
        model: icc_profile.model,
        attributes: icc_profile.attributes,
        rendering_intent: icc_profile.rendering_intent,
        illuminate :icc_profile.illuminate.clone(),
        creator: icc_profile.creator,
        profile_id: icc_profile.profile_id,
        tags: decoded,
    }
}

#[derive(Debug)]
pub struct DecodedICCProfile {
    pub length : u32,
    pub cmmid : u32,
    pub version :u32,
    pub device_class :u32,
    pub color_space : u32,
    pub pcs : u32,
    pub create_date: String,
    pub magicnumber_ascp: u32,
    pub platform: u32,
    pub flags: u32,
    pub manufacturer: u32,
    pub model: u32,
    pub attributes: u64,
    pub rendering_intent: u32,
    pub illuminate :[u32;3],
    pub creator: u32,
    pub profile_id: u128,
    pub tags: HashMap<String,Data>,
}
impl DecodedICCProfile {
    pub fn new(buffer :&Vec<u8>) -> Self {
        icc_profile_decode(buffer)
    }

}

#[derive(Debug)]
pub struct ICCProfile {
    pub length : u32,
    pub cmmid : u32,
    pub version :u32,
    pub device_class :u32,
    pub color_space : u32,
    pub pcs : u32,
    pub create_date: String,
    pub magicnumber_ascp: u32,
    pub platform: u32,
    pub flags: u32,
    pub manufacturer: u32,
    pub model: u32,
    pub attributes: u64,
    pub rendering_intent: u32,
    pub illuminate :[u32;3],
    pub creator: u32,
    pub profile_id: u128,
    pub reserved :Vec<u8>,  // 28byte,
    pub data: Vec<u8>   // left data
}

impl ICCProfile {    
    pub fn new(buffer :&Vec<u8>) -> Self {
        let mut ptr = 0;
        let length = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let cmmid = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let version = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let device_class = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let color_space = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let pcs = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let year = read_u16_be(&buffer,ptr);
        ptr = ptr + 2;
        let month = read_u16_be(&buffer,ptr);
        ptr = ptr + 2;
        let day = read_u16_be(&buffer,ptr);
        ptr = ptr + 2;
        let hour = read_u16_be(&buffer,ptr);
        ptr = ptr + 2;
        let minute = read_u16_be(&buffer,ptr);
        ptr = ptr + 2;
        let second = read_u16_be(&buffer,ptr);
        ptr = ptr + 2;
        let magicnumber_ascp = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let platform = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let flags = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let manufacturer = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let model = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let attributes = read_u64_be(&buffer,ptr);
        ptr = ptr + 8;
        let rendering_intent = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let mut illuminate = [0_u32;3];
        illuminate[0] = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        illuminate[1] = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        illuminate[2] = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let creator = read_u32_be(&buffer,ptr);
        ptr = ptr + 4;
        let profile_id = read_u128_be(&buffer, ptr);
//        ptr += 28;  // padding data

        let create_date = format!("{:>4}/{:>2}/{:>2} {:>02}:{:>02}:{:>02}",
            year,month,day,hour,minute,second);
        Self {
            length: length,
            cmmid : cmmid,
            version: version,
            device_class: device_class,
            color_space: color_space,
            pcs: pcs,
            create_date: create_date.clone(),
            magicnumber_ascp: magicnumber_ascp,
            platform: platform,
            flags: flags,
            manufacturer: manufacturer,
            model: model,
            attributes: attributes,
            rendering_intent: rendering_intent,
            illuminate: illuminate.clone(),
            creator: creator,
            profile_id: profile_id,
            reserved: Vec::new(),
            data : buffer.to_vec(),
        }
    }
}




trait IICNumber {
    fn as_f32(&self) -> f32;
    fn as_f64(&self) -> f64;
    fn int(&self) -> i32;
    fn decimal(&self) -> u32;
}

#[derive(Debug)]
pub struct S15Fixed16Number {
    integer: i16,
    decimal:u16,
}

impl IICNumber for S15Fixed16Number {
    fn as_f32(&self) -> f32 { self.integer as f32 + self.decimal as f32 / u16::MAX as f32 }
    fn as_f64(&self) -> f64 { self.integer as f64 + self.decimal as f64 / u16::MAX as f64 }
    fn int(&self) -> i32 { self.integer as i32 }
    fn decimal(&self) -> u32 { self.decimal as u32}
}

#[derive(Debug)]
pub struct U16Fixed16Number {
    integer:u16,
    decimal:u16,
}

impl IICNumber for U16Fixed16Number {
    fn as_f32(&self) -> f32 { self.integer as f32 + self.decimal as f32 / u16::MAX as f32 }
    fn as_f64(&self) -> f64 { self.integer as f64 + self.decimal as f64 / u16::MAX as f64 }
    fn int(&self) -> i32 { self.integer as i32 }
    fn decimal(&self) -> u32 { self.decimal as u32}
}

#[derive(Debug)]
pub struct U1Fixed15Number {
    decimal:u16,
}

impl IICNumber for U1Fixed15Number {
    fn as_f32(&self) -> f32 { self.decimal as f32 / i16::MAX as f32 }
    fn as_f64(&self) -> f64 { self.decimal as f64 / i16::MAX as f64 }
    fn int(&self) -> i32 { 0 }
    fn decimal(&self) -> u32 { self.decimal as u32}
}

#[derive(Debug)]
pub struct U8Fixed8Number {
    integer:u8,
    decimal:u8,
}

impl IICNumber for U8Fixed8Number {
    fn as_f32(&self) -> f32 { self.integer as f32 + self.decimal as f32 / u8::MAX as f32 }
    fn as_f64(&self) -> f64 { self.integer as f64 + self.decimal as f64 / u8::MAX as f64  }
    fn int(&self) -> i32 { self.integer as i32 }
    fn decimal(&self) -> u32 { self.decimal as u32}
}

#[derive(Debug)]
pub struct XYZNumber {
    pub x:S15Fixed16Number,
    pub y:S15Fixed16Number,
    pub z:S15Fixed16Number
}

#[derive(Debug)]
pub struct Mft1 {
    pub input_channels :u8,
    pub output_channels:u8,
    pub number_of_clut_grid_points:u8,
    pub e_params:Vec<S15Fixed16Number>,
    pub input_table: Vec<u8>,
    pub clut_values: Vec<u8>,
    pub output_table: Vec<u8>,
}

#[derive(Debug)]
pub struct Mft2 {
    pub input_channels :u8,
    pub output_channels:u8,
    pub number_of_clut_grid_points:u8,
    pub e_params:Vec<S15Fixed16Number>,
    pub input_table_enteries: u16,
    pub output_table_enteries: u16,
    pub input_table: Vec<u16>,
    pub clut_values: Vec<u16>,
    pub output_table: Vec<u16>, 
}

#[derive(Debug)]
pub enum Clut {
    UInt8(Vec<u8>),
    UInt16(Vec<u16>),
}

#[derive(Debug)]
pub struct MbaClut {
    grid_points: Vec<u8>,   // max 16
    precision: u8,
    clut_data: Clut,
}


#[derive(Debug)]
pub struct Mba {
    pub input_channels :u8,
    pub output_channels:u8,
    pub b_curves: Vec<ParametricCurve>,
    pub matrix: Vec<S15Fixed16Number>,
    pub m_curves: Vec<ParametricCurve>,
    pub clut: MbaClut,
    pub a_curves: Vec<ParametricCurve>,

}

#[derive(Debug)]
pub struct ResponseCurveSet16 {
    pub number_of_channels: u16,
    pub count_of_measirement_types: u16,
    pub response_curve_structures: Vec<CurveStructure>,
}

#[derive(Debug)]
pub struct Response16Number{
    encoding_the_interval: u16,
    reserved: u16,
    measurement_value:S15Fixed16Number
}

#[derive(Debug)]
pub struct CurveStructure {
    signature: u32,
    for_each_channel:Vec<u32>,
    patch_with_the_maximum_colorant_value:Vec<XYZNumber>,
    response_arrays:Vec<Response16Number>,
}

#[derive(Debug)]
pub struct ParametricCurve {
    funtion_type:u16,
    vals:Vec<S15Fixed16Number>,
}

impl ParametricCurve {
    pub fn len(&self) -> usize {
        self.vals.len() * 4 + 2
    }
}


#[derive(Debug)]
pub enum Data {
    ASCII(String),
    DataTimeNumber(u32,u32,u32,u32,u32,u32),
    Float32Number(f32),
    PositionNumber(Box<[u8]>),
    S15Fixed16Number(S15Fixed16Number),
    S15Fixed16NumberArray(Vec<S15Fixed16Number>),
    ParametricCurve(ParametricCurve),
    U16Fixed16Number(U16Fixed16Number),
    U16Fixed16NumberArray(Vec<U16Fixed16Number>),
    Response16Number(Response16Number),
    U1Fixed15Number(U1Fixed15Number),
    U8Fixed8Number(U8Fixed8Number),
    UInt16Number(u16),
    UInt16NumberArray(Vec<u16>),
    UInt32Number(u32),
    UInt32NumberArray(Vec<u32>),
    UInt64Number(u64),
    UInt64NumberArray(Vec<u64>),
    UInt8Number(u8),
    XYZNumber(XYZNumber),
    XYZNumberArray(Vec<XYZNumber>),
    ChromaticityType(u16,u16,Vec<(U16Fixed16Number,U16Fixed16Number)>),
    MultiLocalizedUnicode(u32,u32,String,String,String),
    ViewCondtions(XYZNumber,XYZNumber,u32),
    Measurement(u32,XYZNumber,u32,U16Fixed16Number,u32),
    Curve(Vec<u16>),
    Lut8(Mft1),
    Lut16(Mft2),
    LutBtoA(Mba),
    ResponseCurveSet16(ResponseCurveSet16),
    None,
}

impl Data {

    pub fn parse(data: &[u8],length:usize) -> (String,Data) {
        let data_type = Self::read_data_type(data,0);
        (data_type.clone(),Self::get(&data_type,data,length))
    }

    fn read_parmetic_curve(data:&[u8]) -> ParametricCurve {
        let mut ptr = 8;
        let funtion_type = read_u16_be(data,ptr);
        ptr += 4;
        let mut vals :Vec<S15Fixed16Number> = vec![];
        
        let num = match funtion_type {
            0 => {
                1
            },
            1 => {
                3
            },
            2 => {
                4
            },
            3 => {
                5
            },
            4 => {
                7
            },
            _ => {  // Error
                0
            },
        };

        for _ in 0..num {
            vals.push(S15Fixed16Number{
                integer: read_i16_be(data, ptr),
                decimal: read_u16_be(data, ptr+2)
            });
            ptr += 4;
        }
        ParametricCurve{funtion_type,vals}
    }

    pub fn get(data_type:&str,data: &[u8],length:usize) -> Data {
        let len = length - 8;
        let mut ptr = 8;
        match data_type {
            "para" => {
                ParametricCurve(Self::read_parmetic_curve(data))
            },
            "sig " => {
                let string = read_ascii_string(data, ptr, 4);
                ASCII(string)
            }
            "XYZ " | "XYZ" => {
                let mut xyzs :Vec<XYZNumber> = vec![];
                while  ptr < length {
                    let xyz = Self::xyz_number(data, ptr);
                    xyzs.push(xyz);
                    ptr += 12;
                }
                XYZNumberArray(xyzs)
            },
            "sf32" => { //s16Fixed16ArrayType
                let mut vals :Vec<S15Fixed16Number> = vec![];
                while  ptr < length {
                    vals.push(S15Fixed16Number{
                        integer: read_i16_be(data, ptr),
                        decimal: read_u16_be(data, ptr+2)
                    });
                    ptr += 4;
                }
                S15Fixed16NumberArray(vals)
            },
            "uf32" => { //U16Fixed16ArrayType
                let mut vals :Vec<U16Fixed16Number> = vec![];
                while  ptr < length {
                    vals.push(U16Fixed16Number{
                        integer: read_u16_be(data, ptr),
                        decimal: read_u16_be(data, ptr+2)
                    });
                    ptr += 4;
                }
                U16Fixed16NumberArray(vals)
            },
            "ui16" => { 
                let mut vals= vec![];
                while  ptr < length {
                    vals.push(read_u16_be(data, ptr));
                    ptr += 2;
                }
                UInt16NumberArray(vals)
            },
            "ui32" => { 
                let mut vals= vec![];
                while  ptr < length {
                    vals.push(read_u32_be(data, ptr));
                    ptr += 4;
                }
                UInt32NumberArray(vals)
            },
            "ui64" => { 
                let mut vals= vec![];
                while  ptr < length {
                    vals.push(read_u64_be(data, ptr));
                    ptr += 8;
                }
                UInt64NumberArray(vals)
            },
            "text"=> {
                let string = read_ascii_string(data, ptr,len);
                Self::ASCII(string)
            },
            "desc" => {
                let string = read_ascii_string(data, ptr+4,len-4);
                Self::ASCII(string)
            }, 
            "chrm" => {
                let device_number = read_u16_be(data,ptr);
                let encoded_value = read_u16_be(data,ptr+2);
                ptr += 4;
                let mut vals :Vec<(U16Fixed16Number,U16Fixed16Number)> = vec![];
                while  ptr < length {
                    vals.push((
                        U16Fixed16Number{
                            integer: read_u16_be(data, ptr),
                            decimal: read_u16_be(data, ptr+2)
                        },
                        U16Fixed16Number{
                            integer: read_u16_be(data, ptr+2),
                            decimal: read_u16_be(data, ptr+4)
                        }));
                    ptr += 8;
                }
                ChromaticityType(device_number,encoded_value,vals)
            },
            "mluc" |"vued" => {
                let number_of_names = read_u32_be(data,ptr);
                ptr +=4;
                let name_recode_size = read_u32_be(data,ptr);
                ptr +=4;
                let first_name_language_code = read_ascii_string(data,ptr,2);
                ptr +=2;
                let first_name_country_code = read_ascii_string(data,ptr,2);
                ptr +=2;
                let lang = first_name_language_code + "." + &first_name_country_code;
                let name_length = read_u32_be(data,ptr) as usize;
                ptr +=4;
                let name_offset = read_u32_be(data,ptr) as usize;
                ptr +=4;
                let mut len = 0;
                let mut vals = vec![];
                while len < name_length {
                    let val = read_u16_be(data, name_offset + len);
                    if val == 0 {
                        break;
                    }
                    vals.push(val);
                    len += 2;
                }
                let string = String::from_utf16_lossy(&vals);
                let mut vals = vec![];
                while ptr < name_offset {
                    let val = read_u16_be(data, ptr);
                    vals.push(val);
                    ptr += 2;
                }
                let more_string = String::from_utf16_lossy(&vals);
                MultiLocalizedUnicode(number_of_names,name_recode_size,lang,string,more_string)

            },
            "view" => {
                let xyz_ilu = Self::xyz_number(data, ptr);
                ptr += 12;
                let xyz_sur = Self::xyz_number(data, ptr);
                ptr += 12;
                let ilu_type = read_u32_be(data,ptr);
                ViewCondtions(xyz_ilu,xyz_sur,ilu_type)              
            },
            "meas" => {
                let encoded_value = read_u32_be(data, ptr);
                ptr += 4;
                let xyz = Self::xyz_number(data, ptr);
                ptr += 12;
                let measurement_geometry = read_u32_be(data, ptr);
                ptr += 4;
                let measurement_flate = U16Fixed16Number{
                    integer: read_u16_be(data, ptr),
                    decimal: read_u16_be(data, ptr+2)
                };
                ptr += 4;

                let measurement_illuminate = read_u32_be(data, ptr);
                Measurement(encoded_value,xyz,measurement_geometry,measurement_flate,measurement_illuminate)

            },
            "curv" => {
                let mut curv = vec![];
                let count = read_u32_be(data, ptr) as usize;
                ptr += 4;
                for _ in 0..count {
                    curv.push(read_u16_be(data, ptr));
                    ptr += 2;
                }
                Curve(curv)
            },
            "mft1" | "mft2"  => {         
                let input_channels= read_byte(data, ptr);
                ptr +=1;
                let output_channels = read_byte(data, ptr);
                ptr +=1;
                let number_of_clut_grid_points = read_byte(data, ptr);
                ptr +=2; // with skip padding

                let mut e_params:Vec<S15Fixed16Number> = vec![];
                // e00 e01 e02 ... e20 d21 e22
                for _ in 0..9 {
                    let e = S15Fixed16Number {
                        integer: read_i16_be(data, ptr),
                        decimal: read_u16_be(data, ptr+2)
                    };
                    e_params.push(e);
                    ptr += 4;
                }

                let clut_size = ((number_of_clut_grid_points as u32).pow(input_channels as u32) * output_channels as u32) as usize;

                if data_type == "mft1" {
                    let mut input_table = vec![];
                    let mut clut_values = vec![];
                    let mut output_table =vec![];

                    let input_channels_size = input_channels as usize * 256;
                    let output_channels_size = output_channels as usize * 256;

                    for _ in 0..input_channels_size {
                        input_table.push(read_byte(data, ptr));
                        ptr += 1;
                    }

                    for _ in  0..clut_size {
                        clut_values.push(read_byte(data, ptr));
                        ptr += 1;
                    }

                    for _ in  0..output_channels_size {
                        output_table.push(read_byte(data, ptr));
                        ptr += 1;
                    }


                    let mft = Mft1{
                        input_channels,
                        output_channels,
                        number_of_clut_grid_points,
                        e_params,
                        input_table,
                        clut_values,
                        output_table, 

                    };
                    Lut8(mft)
                } else {
                    let mut input_table = vec![];
                    let mut clut_values = vec![];
                    let mut output_table =vec![];

                    let input_table_enteries = read_u16_be(data, ptr);
                    ptr += 2;
                    let output_table_enteries = read_u16_be(data, ptr);
                    ptr += 2;

                    let input_channels_size = input_channels as usize * input_table_enteries as usize;
                    let output_channels_size = output_channels as usize * output_table_enteries as usize;

                    for _ in 0..input_channels_size {
                        input_table.push(read_u16_be(data, ptr));
                        ptr += 2;
                    }

                    for _ in  0..clut_size {
                        clut_values.push(read_u16_be(data, ptr));
                        ptr += 2;
                    }

                    for _ in  0..output_channels_size {
                        output_table.push(read_u16_be(data, ptr));
                        ptr += 2;
                    }

                    let mft = Mft2{
                        input_channels,
                        output_channels,
                        number_of_clut_grid_points,
                        e_params,
                        input_table_enteries,
                        output_table_enteries,
                        input_table,
                        clut_values,
                        output_table, 

                    };
                    Lut16(mft)
                }
            },
            "mBA " => { // no sample
                let input_channels= read_byte(data, ptr);
                ptr +=1;
                let output_channels = read_byte(data, ptr);
                ptr +=1;
                ptr +=2; // with skip padding
                let offset_b_curve = read_u32_be(data, ptr) as usize;
                ptr +=4;
                let offset_matrix = read_u32_be(data, ptr) as usize;
                ptr +=4;
                let offset_m_curve = read_u32_be(data, ptr) as usize;
                ptr +=4;
                let offset_clut = read_u32_be(data, ptr) as usize;
                ptr +=4;
                let offset_a_curve = read_u32_be(data, ptr) as usize;

                let mut b_curves = vec![];
                let mut m_curves = vec![];
                let mut a_curves = vec![];
                let mut matrix = vec![];
                let clut = vec![];

                let mut ptr = offset_b_curve;
                for _ in 0..input_channels {
                    let b_curve = Self::read_parmetic_curve(&data[ptr..]);
                    ptr += b_curve.len();
                    b_curves.push(b_curve);
                }
                let mut ptr = offset_matrix;
                for _ in 0..12 {
                    let e = S15Fixed16Number {
                        integer: read_i16_be(data, ptr),
                        decimal: read_u16_be(data, ptr+2)
                    };
                    matrix.push(e);
                    ptr += 4;
                }
                let mut ptr = offset_m_curve;
                for _ in 0..input_channels {
                    let m_curve = Self::read_parmetic_curve(&data[ptr..]);
                    ptr += m_curve.len();
                    m_curves.push(m_curve);
                }
                let mut ptr = offset_clut;

                let mut grid_points = vec![];
                let mut clut_size = output_channels as usize;
                for _ in 0..16 {
                    let grid_point = read_byte(&data,ptr);
                    clut_size *= grid_point as usize;
                    grid_points.push(grid_point);
                    ptr += 1;
                }
                let precision = read_byte(&data,ptr);
                ptr += 1;
                let clut_data = if precision == 1 {
                    let mut clut_entries = vec![];
                    for _ in 0..clut_size {
                        clut_entries.push(read_byte(data, ptr));
                        ptr += 1;
                    }
                    Clut::UInt8(clut)
                } else {
                    let mut clut_entries = vec![];
                    for _ in 0..clut_size {
                        clut_entries.push(read_u16_be(data, ptr));
                        ptr += 2;
                    }
                    Clut::UInt8(clut)
                };

                let clut = MbaClut {
                    grid_points,
                    precision,
                    clut_data,
                };

                let mut ptr = offset_a_curve;
                for _ in 0..input_channels {
                    let a_curve = Self::read_parmetic_curve(&data[ptr..]);
                    ptr += a_curve.len();
                    a_curves.push(a_curve);
                }
                LutBtoA(Mba{
                    input_channels,
                    output_channels,
                    b_curves,
                    matrix,
                    m_curves,
                    clut,
                    a_curves,
                })


            },
            "rcs2" => {
                let number_of_channels = read_u16_be(data, ptr);
                ptr += 2;
                let count_of_measirement_types = read_u16_be(data, ptr);
                let mut count_relative_offsets = vec![];
                for _ in 0..count_of_measirement_types {
                    count_relative_offsets.push(read_u32_be(data, ptr));
                    ptr += 4;
                }
                let mut response_curve_structures = vec![];
                for offset in count_relative_offsets {
                    let mut offset = offset as usize;
                    let signature = read_u32_be(data,offset);
                    offset += 4;
                    let mut for_each_channel = vec![];
                    for _ in 0..number_of_channels {
                        let val = read_u32_be(data,offset);
                        offset += 4;
                        for_each_channel.push(val);
                    }
                    let mut patch_with_the_maximum_colorant_value = vec![];
                    for _ in 0..number_of_channels {
                        let val = Self::xyz_number(data, offset);
                        offset += 12;
                        patch_with_the_maximum_colorant_value.push(val);                        
                    }
                    let mut response_arrays = vec![];
                    for _ in 0..number_of_channels {
                        let encoding_the_interval = read_u16_be(data, offset);
                        offset += 2;
                        let reserved = read_u16_be(data, offset);
                        offset += 2;
                        let measurement_value = S15Fixed16Number {
                            integer: read_i16_be(data, offset),
                            decimal: read_u16_be(data, offset+2)
                        };
                        offset += 4;
                        response_arrays.push(Response16Number{
                            encoding_the_interval,
                            reserved,
                            measurement_value,
                        });
                    }
                    response_curve_structures.push(CurveStructure{
                        signature,
                        for_each_channel,
                        patch_with_the_maximum_colorant_value,
                        response_arrays,
                    });
                }
                Self::ResponseCurveSet16(ResponseCurveSet16{
                    number_of_channels,
                    count_of_measirement_types,
                    response_curve_structures,
                })
            },

            _ => { // no impl
                Self::None
            }
        }

    }

    pub fn as_string(&self) -> String{
        match &*self {
            DataTimeNumber(year,month,day,hour,minutes,second) => {
                format!("{:4}-{:02}-{:02} {:02}:{:02}:{:02}",
                    year,month,day,hour,minutes,second)
            },
            Float32Number(f) => {
                f.to_string()
            },
            PositionNumber(boxed) => {
                format!("{} bytes..",boxed.len())
            },
            S15Fixed16Number(f) => {
                f.as_f32().to_string()
            },
            S15Fixed16NumberArray(vals) => {
                let mut str = "".to_string();
                for f in vals {
                    str += &f.as_f32().to_string();
                    str += " ";
                }
                str.to_string()
            },
            ParametricCurve(prametic_curve) => {
                let mut str = match prametic_curve.funtion_type {
                    0x000 => {"function Y = X**ganma\n"},
                    0x001 => {"function Y = (aX+b)**ganma (X >= -b/a), Y = 0 (X < -b/a)\n"},
                    0x002 => {"function Y = (aX+b)**ganma + c(X >= -b/a), Y = c (X < -b/a)\n"},
                    0x003 => {"function Y = (aX+b)**ganma (X >= d), Y = cX (X < d)\n"},
                    0x004 => {"function Y = (aX+b)**ganma + e(X >= d), Y = cX + f (X < d)\n"},
                    _ => {"function Unknown"},
                }.to_string();
                for f in &prametic_curve.vals {
                    str += &f.as_f32().to_string();
                    str += " ";
                }
                str.to_string()
            }
            U16Fixed16Number(f) => {
                f.as_f32().to_string()
            },
            Response16Number(v) => {
                format!("{} {} {}",v.encoding_the_interval,v.reserved,v.measurement_value.as_f32())              
            },
            U1Fixed15Number(f) => {
                f.as_f32().to_string()
            },
            U8Fixed8Number(f) => {
                f.as_f32().to_string()
            },
            UInt8Number(n) => {
                n.to_string()
            },
            UInt16Number(n) => {
                n.to_string()
            },
            UInt32Number(n) => {
                n.to_string()
            },
            UInt64Number(n) => {
                n.to_string()
            },
            XYZNumber(xyz) => {
                format!("X:{} Y:{} Z:{}\n",xyz.x.as_f32(),xyz.y.as_f32(),xyz.z.as_f32())
            },
            XYZNumberArray(xyzs) => {
                let mut str = "".to_string();
                for xyz in xyzs {
                    str += &format!("X:{} Y:{} Z:{} ",xyz.x.as_f32(),xyz.y.as_f32(),xyz.z.as_f32())
                }
                str + "\n"
            },
            ChromaticityType(device_number,encoded_value,vals) => {
                let encoded = match encoded_value {
                    0x001 => {"ITU-R BT.709"},
                    0x002 => {"SMPTE RP145-1994"},
                    0x003 => {"EBU Tech.3213-E"},
                    0x004 => {"P22"},
                    _ => {"unknown"},

                };
                let mut str = format!("Number of Device Channels {} {} ",device_number,encoded);
                for (x,y) in vals {
                    str += &format!("x:{} y:{} ",x.as_f32(),y.as_f32());
                }

                str + "\n"
            },
            Measurement(encoded_value,xyz,measurement_geometry,measurement_flate,measurement_illuminate) => {
                let mut str = match encoded_value {
                    0x00000001 => {"Standard Observer: CIE 1931 standard colorimetric observer\n"},
                    0x00000002 => {"Standard Observer: CIE 1964 standard colorimetric observer\n"},
                    _ => {"Standard: Observer unknown\n"},
                }.to_string();
                str += &format!("XYZ tristimulus values X:{} Y:{} Z:{}\n",xyz.x.as_f32(),xyz.y.as_f32(),xyz.z.as_f32(),);
                str += "Measurement geometry ";
                str += match measurement_geometry {
                    0x00000001 => {"0/45 or 45/0\n"},
                    0x00000002 => {"0/d or d/0\n"},
                    _ => {"unknown\n"},
                };
                str += &format!("Measurement flare {}\n",measurement_flate.as_f32());
                str += "Standard Illuminant: ";
                str += match measurement_illuminate {
                    0x00000001 => {"D50\n"},
                    0x00000002 => {"D65\n"},
                    0x00000003 => {"D93\n"},
                    0x00000004 => {"F2\n"},
                    0x00000005 => {"D55\n"},
                    0x00000006 => {"A\n"},
                    0x00000007 => {"Equi-Power (E)\n"},
                    0x00000008 => {"F8\n"},
                    _ => {"unknown\n"},
                };

                str + "\n"
            },
            ResponseCurveSet16(response_curve_structures) => {
                let mut str = "".to_string();
                str += &format!("number_of_channels {} count_of_measirement_types{}\n",
                    response_curve_structures.number_of_channels,
                    response_curve_structures.count_of_measirement_types);
                
                    for curve_structure in &response_curve_structures.response_curve_structures {
                        let sig = read_ascii_string(&curve_structure.signature.to_be_bytes(),0,4);

                        str += &format!("{}\n",sig);
                        
                        for each in &curve_structure.for_each_channel {
                            str += &format!("{} ",each)
                        }
                        str += "\n";

                        for xyz in &curve_structure.patch_with_the_maximum_colorant_value {
                            str += &format!("X:{} Y:{} Z:{} ",xyz.x.as_f32(),xyz.y.as_f32(),xyz.z.as_f32())
                        }
                        str += "\n";

                        for respose in &curve_structure.response_arrays {
                            str += &format!("{:?} ",respose)
                        }
                        str += "\n";
                    }

                str + "\n"
            }

            ASCII(string) => {
                string.to_string()
            },
            None => {
                "None".to_string()
            },
            _ => {
                format!("{:?}",*self)
            }
        }

    }

    pub fn xyz_number(data: &[u8],ptr: usize) -> XYZNumber {
        let cie_x = S15Fixed16Number {
            integer: read_i16_be(&data, ptr),
            decimal: read_u16_be(&data, ptr+2)
        };
        let cie_y = S15Fixed16Number {
            integer: read_i16_be(&data, ptr+4),
            decimal: read_u16_be(&data, ptr+6)
        };
        let cie_z = S15Fixed16Number {
            integer: read_i16_be(&data, ptr+8),
            decimal: read_u16_be(&data, ptr+10)
        };
        XYZNumber{x:cie_x,y:cie_y,z:cie_z}
    }

    pub fn read_data_type(data:&[u8],ptr: usize) -> String {
        let data_type = read_string(data, ptr as usize, 4);
        if data_type.len() == 0 {
            return read_string(data, ptr as usize, 3)
        }
        data_type
    }
}

