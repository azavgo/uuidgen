mod svg;
mod truernduuid; 
mod rnduuid;

use reqwest::Error;
use truernduuid::TrueRndUUID; 
use svg::*; 
use std::{fs::write, num::ParseIntError};
use qrcodegen::{QrCode, QrCodeEcc, DataTooLong};
use rnduuid::*; 

fn main() -> Result<(), Error> {
    //let qr = QrCode::encode_text("Hello, world!",
    //QrCodeEcc::Medium)?;
    //let svg = to_svg_string(&qr, 4);
    //write("qrcode.svg",&svg).unwrap(); 

    //println!("{}", TrueRndUUID::uuid()?); 

    println!("{}", rnd_uuid());  

Ok(())
}

