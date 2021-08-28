use std::fs::write;
use std::io::Error;
use std::u32;

use qrcodegen::{QrCode, QrCodeEcc};
use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize)]
struct ANUqrng {
    //r#type: String,
    //length: u8,
    //size: u8,
    data: Vec<String>,
}

impl ANUqrng {
    fn new() -> Result<Self, reqwest::Error> {
        let anu_qrng = reqwest::blocking::get(
            "https://qrng.anu.edu.au/API/jsonI.php?length=16&type=hex16&size=1",
        )?
        .json::<Self>()?;

        Ok(anu_qrng)
    }

    fn data(&self) -> &Vec<String> {
        &self.data
    }

    fn anu_qrng_uuid() -> Result<String, reqwest::Error> {
        let anu_qrnd = Self::new()?;
        let uuid = [
            &anu_qrnd.data()[0][..],
            &anu_qrnd.data()[1][..],
            &anu_qrnd.data()[2][..],
            &anu_qrnd.data()[3][..],
            &anu_qrnd.data()[4][..],
            &anu_qrnd.data()[5][..],
            to_four(&anu_qrnd.data()[6][..]).as_str(),
            &anu_qrnd.data()[7][..],
            to_two(&anu_qrnd.data()[8][..]).as_str(),
            &anu_qrnd.data()[9][..],
            &anu_qrnd.data()[10][..],
            &anu_qrnd.data()[11][..],
            &anu_qrnd.data()[12][..],
            &anu_qrnd.data()[13][..],
            &anu_qrnd.data()[14][..],
            &anu_qrnd.data()[15][..],
        ]
        .join("");

        Ok(uuid)
    }
}

pub struct UUID {
    uuid: String,
}

impl UUID {
    pub fn new() -> Self {
        let anu_qrng = ANUqrng::anu_qrng_uuid();
        let uuid = match anu_qrng {
            Ok(uuid_qrng) => uuid_qrng,
            _ => rnd_uuid(),
        };

        Self { uuid: uuid }
    }

    pub fn uuid(self) -> String {
        self.uuid
    }

    pub fn to_svg(self) -> Result<(), Error> {
        let uuid = self.uuid();
        match QrCode::encode_text(&uuid[..], QrCodeEcc::Medium) {
            Ok(uuid_qrcode) => {
                let uuid_qrcode_svg = to_svg_string(&uuid_qrcode, 4);
                write(format!("{}.svg", &uuid), &uuid_qrcode_svg)?;
            }
            Err(e) => eprintln!("{}", e.to_string()),
        };
        Ok(())
    }
}

//returns an unbiased random hex integer over the u8 range 0..=255 as a double character String
fn u8_hex_rnd() -> String {
    let mut rng = rand::thread_rng();

    //unbiased random integer over the u8 range 0..=255
    let u: u8 = rng.gen();
    let mut h = format!("{:x}", &u);

    //add 0 to numbers generated within the range 0..=15
    if u < 16 {
        h.push('0');
        h = h.chars().rev().collect::<String>();
    }
    h
}

fn rnd_uuid() -> String {
    let uuid_vec = vec![
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        to_four(u8_hex_rnd().as_str()),
        u8_hex_rnd(),
        to_two(u8_hex_rnd().as_str()),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
        u8_hex_rnd(),
    ];
    uuid_vec.join("")
}

fn to_four(s: &str) -> String {
    let n: u32 = u32::from_str_radix(s, 16).unwrap();
    format!("{:x}", 64 + n - ((n >> 4) << 4))
}

fn to_two(s: &str) -> String {
    let n: u32 = u32::from_str_radix(s, 16).unwrap();
    format!("{:x}", 128 + n - ((n >> 6) << 6))
}

// Returns a string of SVG code for an image depicting
// the given QR Code, with the given number of border modules.
// The string always uses Unix newlines (\n), regardless of the platform.
// From: https://github.com/nayuki/QR-Code-generator/blob/master/rust/examples/qrcodegen-demo.rs

fn to_svg_string(qr: &QrCode, border: i32) -> String {
    assert!(border >= 0, "Border must be non-negative");
    let mut result = String::new();
    result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
    let dimension = qr
        .size()
        .checked_add(border.checked_mul(2).unwrap())
        .unwrap();
    result += &format!(
		"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension);
    result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
    result += "\t<path d=\"";
    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    result += " ";
                }
                result += &format!("M{},{}h1v1h-1z", x + border, y + border);
            }
        }
    }
    result += "\" fill=\"#000000\"/>\n";
    result += "</svg>\n";
    result
}

#[cfg(tests)]
mod tests; 

mod tests {
    use super::*;  
    #[test]
    fn test_to_four() {
        assert_eq!("4f".to_string(), to_four("af")); 
    }

    #[test]
    fn test_to_two() {
        assert_eq!("bf".to_string(), to_two("ff")); 
    }

}
