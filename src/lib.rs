pub use crate::iccprofile::*;
pub mod utils;
pub mod iccprofile;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
