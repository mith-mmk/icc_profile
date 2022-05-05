# ICC Profile Reader
## Example
```rust
pub fn main() -> std::io::Result<()> {
    let mut is_fast = true;
    for argument in env::args() {
        if is_fast {
            is_fast = false;
            continue
        }
        println!("{}",argument);
        let icc_profile = icc_profile::utils::load(argument)?;
        let decoded = DecodedICCProfile::new(&icc_profile.data)?;
        println!("{}",decoded_print(&decoded, 0)?);
    }
    Ok(())
}
```

## Todo
- ICC Profile 4.x ,5.x tags full support.
## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
