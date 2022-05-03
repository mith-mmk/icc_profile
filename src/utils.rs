use crate::iccprofile::ICCNumber;
use crate::Data;
use crate::DecodedICCProfile;
use bin_rs::io::*;
use crate::ICCProfile;
use std::fs::File;
use std::io::Result;
use std::io::{Error,ErrorKind};
use std::io::prelude::*;

pub(crate) fn bound_check(buf:&[u8],ptr:usize,size:usize) -> Result<()> {
    if buf.len() < ptr + size {
        let string = format!("Outbound check error len {} but index {}",buf.len(),ptr+size);
        return Err(Error::new(ErrorKind::Other,string))
    }

    Ok(())
}

pub fn dump(filename:String,buf:&mut [u8]) -> Result<()>  {
    let mut file = File::create(filename)?;
    file.write_all(buf)?;
    file.flush()?;
    Ok(())
}

pub fn load(filename:String) -> Result<ICCProfile> {
    let mut file = File::open(filename)?;
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    ICCProfile::new(&buf)
}

pub fn icc_profile_header_print(header: &ICCProfile) -> String {
    let mut str = "=========== ICC Profile ===========\n".to_string();
    str += &format!("cmmid {}\n",read_ascii_string(&header.cmmid.to_be_bytes(),0,4));
    str += &format!("version {:08x}\n",&header.version);
    str += &format!("Device Class {}\n",read_ascii_string(&header.device_class.to_be_bytes(),0,4));
    str += &format!("Color Space {}\n",read_ascii_string(&header.color_space.to_be_bytes(),0,4));
    str += &format!("PCS {}\n",read_ascii_string(&header.pcs.to_be_bytes(),0,4));
    str += &format!("DATE {}\n",header.create_date);
    str += &format!("It MUST be 'ascp' {}\n",read_ascii_string(&header.magicnumber_ascp.to_be_bytes(),0,4));
    str += &format!("Platform {}\n",read_ascii_string(&header.platform.to_be_bytes(),0,4));
    str += &format!("flags {}\n",&header.flags);
    str += &format!("manufacturer {}\n",read_ascii_string(&header.manufacturer.to_be_bytes(),0,4));
    str += &format!("Model {:04x}\n",&header.model);
    str += &format!("Attributes {:>064b}\n",&header.attributes);
    str += &format!("Illiuminate X:{} Y:{} Z:{}\n",&header.illuminate.x.as_f64(),&header.illuminate.y.as_f64(),&header.illuminate.z.as_f64());
    str += &format!("Creator {}\n",read_ascii_string(&header.creator.to_be_bytes(),0,4));
    str += &format!("Profile ID (MD5 {:016x})\n",&header.profile_id);
    str += &format!("Data length {}bytes\n",&header.length);
    str
}


pub fn icc_profile_print(icc_profile :&ICCProfile,verbose:usize) -> Result<String> {
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
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length,icc_profile.version)?;
                str += &format!("LUT Table - data_type:{}\n",data_type);
                str += &val.as_string(verbose);
                str += "\n";
            },
            "chad" => {
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length,icc_profile.version)?;
                str += &format!("Conversion D65 to D50 - data_type:{}\n",data_type);
                str += &val.as_string(verbose);
                str += "\n";
            },
            "bkpt" => {
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length,icc_profile.version)?;
                str += &format!("Media Black Point - data_type:{}\n",data_type);
                str += &val.as_string(verbose);
                str += "\n\n";
            },

            "bXYZ" | "gXYZ" | "rXYZ" => {
                str += "rgb XYZ Tag ";
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length,icc_profile.version)?;
                str += &format!("data_type:{}\n",data_type);
                str += &val.as_string(verbose);
                str += "\n";
            },
            "bTRC" | "gTRC" | "rTRC"=> { // bTRC
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length,icc_profile.version)?;
                str += &format!("Color tone reproduction curve - data_type:{}\n",data_type);
                str += &val.as_string(verbose);
                str += "\n\n";
            },
            "desc" => {
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length,icc_profile.version)?;
                str += &format!("desc - data_type:{}\n",data_type);
                str += &val.as_string(verbose);
                str += "\n\n";
            },
            "cprt" => {
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length,icc_profile.version)?;
                str += &format!("cprt - data_type:{}\n",data_type);
                str += &val.as_string(verbose);
                str += "\n\n";
            },
            "wtpt" => {
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length,icc_profile.version)?;
                str += &format!("Media white point - data_type:{}\n",data_type);
                str += &val.as_string(verbose);
                str += "\n";
            },
            _ => {
                let ptr = tag_offset;
                let (data_type,val) = Data::parse(&icc_profile.data[ptr..],tag_length,icc_profile.version)?;
                str += &format!("{} - data_type:{}\n",tag_name,data_type);
                str += &val.as_string(verbose);
                str += "\n";
            },

        }
    }
    Ok(str)
}

/// decoded_print to String ICC Profile data
/// - verbose > 0  very very long information

pub fn decoded_print(header :&DecodedICCProfile,verbose:usize) -> Result<String> {
    let mut str = "=========== ICC Profile ===========\n".to_string();
    str += &format!("cmmid {}\n",read_ascii_string(&header.cmmid.to_be_bytes(),0,4));
    str += &format!("version {}.{:02x}\n",&header.version>>24 & 0xff,&header.version>>16 & 0xff);
    str += &format!("Device Class {}\n",read_ascii_string(&header.device_class.to_be_bytes(),0,4));
    str += &format!("Color Space {}\n",read_ascii_string(&header.color_space.to_be_bytes(),0,4));
    str += &format!("PCS {}\n",read_ascii_string(&header.pcs.to_be_bytes(),0,4));
    str += &format!("DATE {}\n",header.create_date);
    str += &format!("It MUST be 'ascp' {}\n",read_ascii_string(&header.magicnumber_ascp.to_be_bytes(),0,4));
    str += &format!("Platform {}\n",read_ascii_string(&header.platform.to_be_bytes(),0,4));
    str += &format!("flags {}\n",&header.flags);
    str += &format!("manufacturer {}\n",read_ascii_string(&header.manufacturer.to_be_bytes(),0,4));
    str += &format!("Model {:04x}\n",&header.model);
    str += &format!("Attributes {:>064b}\n",&header.attributes);

    let x = header.illuminate.x.as_f64();
    let y = header.illuminate.y.as_f64();
    let z = header.illuminate.z.as_f64();
    str += &format!("Illiuminate X:{} Y:{} Z:{}\n",x,y,z);
    str += &format!("Creator {}\n",read_ascii_string(&header.creator.to_be_bytes(),0,4));
    str += &format!("Profile ID (MD5 {:016x})\n",&header.profile_id);
    str += &format!("Data length {}bytes\n",&header.length);
    str += "\n==== ICC Profiles defined data ====\n";
    for (tag_name,val) in &header.tags {
        let tag_name = tag_name.to_string();
        str +=  &format!("Tag name {}\n",tag_name);
        match &*tag_name {
            "A2B0" | "A2B1" | "A2B2" | "B2A0" | "B2A1" | "B2A2" => {
                str += "LUT Table\n";
            },
            "chad" => {
                str += "Conversion D65 to D50\n";
            },
            "bkpt" => {
                str += "Media Black Point\n";
            },

            "bXYZ" | "gXYZ" | "rXYZ" => {
                str += "rgb XYZ Tag ";
            },
            "bTRC" | "gTRC" | "rTRC"=> { // bTRC
                str += "Color tone reproduction curve\n";
            },
            "desc" => {
                str += "description: ";
            },
            "cprt" => {
                str += "copyright: ";
            },
            "wtpt" => {
                str += "Media white point\n"
            },
            _ => {
            },

        }
        str += &val.as_string(verbose);
        str += "\n";
    }
    Ok(str)
}

