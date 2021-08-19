use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TrueRndUUID {
    r#type: String,
    length: u8,
    size: u8,
    data: Vec<String>,
}

impl TrueRndUUID {
    fn new() -> Result<Self, Error> {
        let true_rnd_uuid = reqwest::blocking::get(
            "https://qrng.anu.edu.au/API/jsonI.php?length=16&type=hex16&size=1",
        )?
        .json::<Self>()?;

        Ok(true_rnd_uuid)
    }

    fn data(&self) -> &Vec<String> {
        &self.data
    }

    pub fn uuid() -> Result<String, Error> {
        let true_rnd_uuid = Self::new()?;
        let uuid = [
            &true_rnd_uuid.data()[0][..],
            &true_rnd_uuid.data()[1][..],
            &true_rnd_uuid.data()[2][..],
            &true_rnd_uuid.data()[3][..],
            &true_rnd_uuid.data()[4][..],
            &true_rnd_uuid.data()[5][..],
            &true_rnd_uuid.data()[6][..],
            &true_rnd_uuid.data()[7][..],
            &true_rnd_uuid.data()[8][..],
            &true_rnd_uuid.data()[9][..],
            &true_rnd_uuid.data()[10][..],
            &true_rnd_uuid.data()[11][..],
            &true_rnd_uuid.data()[12][..],
            &true_rnd_uuid.data()[13][..],
            &true_rnd_uuid.data()[14][..],
            &true_rnd_uuid.data()[15][..],
        ]
        .join(""); 

        Ok(uuid)
    }
}