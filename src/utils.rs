use bin_rs::io::*;
use crate::Data;
use crate::ICCProfile;
use std::fs::File;
use std::io::prelude::*;

pub fn dump(filename:String,buf:&mut [u8]) -> std::io::Result<()>  {
    let mut file = File::create(filename)?;
    file.write_all(buf)?;
    file.flush()?;
    Ok(())
}

pub fn load(filename:String) -> std::io::Result<ICCProfile> {
    let mut file = File::open(filename)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    Ok(ICCProfile::new(&buf))
}

pub fn icc_profile_header_print(header: &ICCProfile) -> String {
    let mut str = "=========== ICC Profile ===========\n".to_string();
    str += &format!("cmmid {}\n",String::from_utf8_lossy(&header.cmmid.to_be_bytes()));
    str += &format!("version {:08x}\n",&header.version);
    str += &format!("Device Class {}\n",String::from_utf8_lossy(&header.device_class.to_be_bytes()));
    str += &format!("Color Space {}\n",String::from_utf8_lossy(&header.color_space.to_be_bytes()));
    str += &format!("PCS {}\n",String::from_utf8_lossy(&header.pcs.to_be_bytes()));
    str += &format!("DATE {}\n",header.create_date);
    str += &format!("It MUST be 'ascp' {}\n",String::from_utf8_lossy(&header.magicnumber_ascp.to_be_bytes()));
    str += &format!("Platform {}\n",String::from_utf8_lossy(&header.platform.to_be_bytes()));
    str += &format!("flags {}\n",&header.flags);
    str += &format!("Manuacture {}\n",String::from_utf8_lossy(&header.manufacturer.to_be_bytes()));
    str += &format!("Model {:04x}\n",&header.model);
    str += &format!("Attributes {:>064b}\n",&header.attributes);
    str += &format!("Illiuminate X:{} Y:{} Z:{}\n",&header.illuminate[0],&header.illuminate[1],&header.illuminate[2]);
    str += &format!("Creator {}\n",String::from_utf8_lossy(&header.creator.to_be_bytes()));
    str += &format!("Profile ID (MD5 {:016x})\n",&header.profile_id);
    str += &format!("Data length {}bytes\n",&header.data.len());
    str
}


pub fn icc_profile_print(icc_profile :&ICCProfile) -> String {
    let mut str = icc_profile_header_print(&icc_profile);
    let header_size = 128;
    let mut ptr = header_size;
    str += "==== ICC Profiles defined data ====\n";
    let tags = read_u32_be(&icc_profile.data,ptr);
    ptr +=  4;
    str += &format!("Tags {}\n",tags);

    for _ in 0..tags {
        let tag_name = read_string(&icc_profile.data,ptr,4);
        ptr +=  4;
        let tag_offset = read_u32_be(&icc_profile.data,ptr) as usize;
        ptr +=  4;
        let tag_length = read_u32_be(&icc_profile.data,ptr) as usize;
        ptr +=  4;
        str +=  &format!("Tag name {} {}bytes\n",tag_name,tag_length);
        match &*tag_name {
            "A2B0" | "A2B1" | "A2B2" | "B2A0" | "B2A1" | "B2A2" => {
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length);
                str += &format!("LUT Table - data_type:{}\n",data_type);
                str += &val.as_string();
                str += "\n";
            },
            "chad" => {
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length);
                str += &format!("Conversion D65 to D50 - data_type:{}\n",data_type);
                str += &val.as_string();
                str += "\n";
            },
            "bkpt" => {
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length);
                str += &format!("Media Black Point - data_type:{}\n",data_type);
                str += &val.as_string();
                str += "\n\n";
            },

            "bXYZ" | "gXYZ" | "rXYZ" => {
                str += "rgb XYZ Tag ";
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length);
                str += &format!("data_type:{}\n",data_type);
                str += &val.as_string();
                str += "\n";
            },
            "bTRC" | "gTRC" | "rTRC"=> { // bTRC
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length);
                str += &format!("Color tone reproduction curve - data_type:{}\n",data_type);
                str += &val.as_string();
                str += "\n\n";
            },
            "desc" => {
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length);
                str += &format!("desc - data_type:{}\n",data_type);
                str += &val.as_string();
                str += "\n\n";
            },
            "cprt" => {
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length);
                str += &format!("cprt - data_type:{}\n",data_type);
                str += &val.as_string();
                str += "\n\n";
            },
            "wtpt" => {
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length);
                str += &format!("Media white point - data_type:{}\n",data_type);
                str += &val.as_string();
                str += "\n";
            },
            _ => {
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length);
                str += &format!("{} - data_type:{}\n",tag_name,data_type);
                str += &val.as_string();
                str += "\n";
            },

        }
    }
    str
}
