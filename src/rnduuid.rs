use rand::Rng;

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

pub fn rnd_uuid() -> String {
    let mut rnd_uuid = u8_hex_rnd(); 
    for _i in 0..15 {
        rnd_uuid.push_str(&u8_hex_rnd()[..])
    }
    rnd_uuid
}