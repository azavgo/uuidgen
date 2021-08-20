## Library to generate UUID-like objects

### Features: 
1. True random generated 32-bit number using an http call to https://qrng.anu.edu.au
1. Random generated 32-bit number using **rand** crate at https://crates.io/crates/rand 
1. Format is a low case without dashes, *e.g.* c69436a1975fe2ebb0a6b09163031397
1. Allows to be generated as a string
1. Allows to be generated as a QR code using **qrcodegen** crate at https://crates.io/crates/qrcodegen 
1. Allows to be generated as a QR code svg file using **svg_to_string** function at  https://github.com/nayuki/QR-Code-generator/blob/master/rust/examples/qrcodegen-demo.rs

### How to use this library: 
1. Generate a true random UUID like object as a string using ANU quantum random numbers service: 

    use truernduuid::TrueRndUUID;

    let uuid = TrueRndUUID::uuid();

1. Generate a random UUID like object as a string without accessing the Internet: 

    use rnduuid::*;

    let uuid = rnd_uuid(); 

1. Generate QR code from the UUID string and write it as a uuid_qrcode.svg file: 

    use qrcodegen::{QrCode, QrCodeEcc};
    use svg::*; 

    let uuid_qrcode = QrCode::encode_text(UUID, QrCodeEcc::Medium); 
    let uuid_svg = to_svg_string(&uuid_qrcode, 4);
    write("uuid_qrcode.svg",&uuid_svg).unwrap();
