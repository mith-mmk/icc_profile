pub use crate::iccprofile::*;
pub mod utils;
pub mod iccprofile;

use std::env;

pub fn main() -> std::io::Result<()> {
    let mut is_fast = true;
    for argument in env::args() {
        if is_fast {
            is_fast = false;
            continue
        }
        println!("{}",argument);
        let icc_profile = crate::utils::load(argument)?;
        let decoded = DecodedICCProfile::new(&icc_profile.data);
        println!("{:?}",decoded);
    }
    Ok(())
}