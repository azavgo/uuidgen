## Library to generate version 4 (random based) UUID strings 

### Features: 
1. True random RFC4122 version 4 UUID string is generated using the ANU QRNG service at https://qrng.anu.edu.au;
1. If the https://qrng.anu.edu.au site is not available, UUID string is generated using **rand** crate (https://crates.io/crates/rand); 
1. Generated UUID string is formatted to a low case string without dashes, *e.g.* 808fd7067c5c4cae93ab4a7a286fc271;
1. UUID QR code svg file is generated using **qrcodegen** crate (https://crates.io/crates/qrcodegen) and **svg_to_string** function from https://github.com/nayuki/QR-Code-generator/blob/master/rust/examples/qrcodegen-demo.rs


### How to use this library: 
1. Add to Cargo.toml: 
```Toml
    [dependencies]
    uuidgen = {git = "https://github.com/azavgo/uuidgen", branch = "main"}
```
2. Generate version 4 UUID string:  
```Rust
    use uuidgen::{UUID, UUIDError};

    fn main() -> Result<(), UUIDError>{
        let uuid = UUID::new()?.uuid();
        println!("Generated UUID: {}", &uuid); 
        Ok(())
    }
  
```
3. Generate QR code from the UUID string and write it as a "uuid".svg file: 
```Rust
    use uuidgen::{UUID, UUIDError};

    fn main() -> Result<(), UUIDError>{
        UUID::new()?.to_svg()?;
        Ok(())
    } 
``` 