mod svg;
use svg::*;  
use std::fs::write;
//use std::io::Error; 
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use qrcodegen::DataTooLong;


fn main() -> Result<(), DataTooLong> {
    let qr = QrCode::encode_text("Hello, world!",
    QrCodeEcc::Medium)?;
    let svg = to_svg_string(&qr, 4);
    write("qrcode.svg",&svg).unwrap(); 
Ok(())

}

