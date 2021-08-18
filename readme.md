## Library to generate UUID-like objects

### Features: 
1. True random generated 32-bit number using an http call to https://qrng.anu.edu.au
1. Random generated 32-bit number using **random** crate at https://crates.io/crates/random  
1. Format is a low case without dashes, *e.g.* c69436a1975fe2ebb0a6b09163031397
1. The first hex number is always > 0
1. Allows to be generated as a string
1. Allows to be generated as a QR code using **qrcodegen** crate at https://crates.io/crates/qrcodegen 
1. Allows to be generated as a QR code svg file using **svg_to_string** function at  https://github.com/nayuki/QR-Code-generator/blob/master/rust/examples/qrcodegen-demo.rs

