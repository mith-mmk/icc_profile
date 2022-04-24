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