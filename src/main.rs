mod color;
mod protocol;
mod serial;
mod nt;
mod animation {
    pub mod animation;
    pub mod solid_animation;
    pub mod strobe_animation;
    pub mod larson_animation;
    pub mod rainbow_animation;
}

use crate::color::RGBWColor;
use crate::animation::animation::Animation;
use crate::animation::solid_animation::SolidAnimation;
use crate::animation::strobe_animation::StrobeAnimation;
use crate::animation::larson_animation::LarsonAnimation;
use crate::animation::rainbow_animation::RainbowAnimation;

use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let mut arduino = serial::Arduino::new("/dev/ttyACM0", 115_200)
        .expect("Failed to open Arduino");

    println!("Sucessfully connected to Arduino");
    thread::sleep(Duration::from_secs(2));

    arduino.send(&[0xAA]).expect("Failed to send buffer to Arduino");

    match arduino.read_bytes() {
        Ok(0x55) => {
            println!("Communication between laptop and Arduino successful");
        }

        Ok(value) => {
            println!("Unexpected response from Arduino");
        }

        Err(error) => {
            println!("Did not get a response from the Arduino");
        }
    }

    nt::run().await;
}
