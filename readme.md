## Library to generate UUID-like objects

### Features: 
1. True random generated UUID-like string using an http call to https://qrng.anu.edu.au
1. If the https://qrng.anu.edu.au site is not available, random UUID-like string is generated using **rand** crate (https://crates.io/crates/rand) 
1. Generated UUID-like string is formatted to a low case string without dashes, *e.g.* c69436a1975fe2ebb0a6b09163031397
1. Allows to be generated as a QR code using **qrcodegen** crate at https://crates.io/crates/qrcodegen 
1. Allows to be generated as a QR code (https://crates.io/crates/qrcodegen) svg file using **svg_to_string** function from https://github.com/nayuki/QR-Code-generator/blob/master/rust/examples/qrcodegen-demo.rs

### How to use this library: 
1. Generate a true random UUID-like string:  
```
    use truernduuid::UUID;

    let uuid_gen = UUID::new();
    let uuid = uuid_gen.uuid();
    println!("Generated UUID: {}", &uuid);  
```
1. Generate QR code from the UUID-like string and write it as a "uuid".svg file: 
```
    use truernduuid::UUID;

    let uuid_gen = UUID::new();
    uuid_gen.to_svg().unwrap(); 
``` 