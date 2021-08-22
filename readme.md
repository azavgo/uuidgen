## Library to generate UUID-like objects

### Features: 
1. True random generated UUID-like string using an http call to https://qrng.anu.edu.au;
1. If the https://qrng.anu.edu.au site is not available, random UUID-like string is generated using **rand** crate (https://crates.io/crates/rand); 
1. Generated UUID-like string is formatted to a low case string without dashes, *e.g.* c69436a1975fe2ebb0a6b09163031397;
1. UUID QR code svg file is generated using **qrcodegen** crate (https://crates.io/crates/qrcodegen) and **svg_to_string** function from https://github.com/nayuki/QR-Code-generator/blob/master/rust/examples/qrcodegen-demo.rs

*Note*: This library does not comply with the RFC4122 version 4 (random based) UUIDs

### How to use this library: 
1. Add to Cargo.toml: 
```
    [dependencies]
    uuidgen = {git = "https://github.com/azavgo/uuidgen"}
```
2. Generate a true random UUID-like string:  
```
    use uuidgen::UUID;

    let uuid = UUID::new().uuid();
    println!("Generated UUID: {}", &uuid);  
```
3. Generate QR code from the UUID-like string and write it as a "uuid".svg file: 
```
    use uuidgen::UUID;

    UUID::new().to_svg().unwrap(); 
``` 