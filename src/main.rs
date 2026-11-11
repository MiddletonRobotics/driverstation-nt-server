mod color;
mod protocol;
mod serial;
mod nt;
mod animation {
    pub mod animation;
    pub mod solid_animation;
    pub mod strobe_animation;
    pub mod larson_animation;
    pub mod fade_animation;
    pub mod rainbow_animation;
    pub mod fire_animation;
}

use std::thread;
use std::time::Duration;

use tokio::time::{interval};
use tokio::sync::watch;

#[tokio::main]
async fn main() {
    let mut arduino = serial::Arduino::new("/dev/ttyACM0", 115_200).expect("Failed to open Arduino");
    println!("Sucessfully connected to Arduino");
    thread::sleep(Duration::from_secs(2));

    arduino.send(&[0xAA]).expect("Failed to send buffer to Arduino");

    match arduino.read_bytes() {
        Ok(0x55) => {
            println!("Communication between laptop and Arduino successful");
        }

        Ok(value) => {
            println!("Unexpected response from Arduino: {}", value);
        }

        Err(error) => {
            println!("Did not get a response from the Arduino. Ended with error: {}", error);
        }
    }

    let (sender, receiver) = watch::channel(nt::AnimationData::default());
    tokio::spawn(nt::run(sender));

    let mut update_timer = interval(Duration::from_millis(100));
    let mut last_sent: Vec<u8> = Vec::new();

    loop {
        update_timer.tick().await;

        let data = receiver.borrow().clone();
        let animation = match data.to_animation() {
            Ok(animation) => animation,
            Err(_) => continue
        };

        let mut buffer = [0u8; protocol::BUFFER_ARRAY_SIZE];
        let size = match protocol::encode(&animation, &mut buffer) {
            Ok(size) => size,
            Err(_) => continue
        };

        let packet = &buffer[..size];
        if packet == last_sent.as_slice() {
            continue;
        }

        match arduino.send_with_ack(packet) {
            Ok(true) => {
                println!("Send new animation to the Arduino");

                last_sent.clear();
                last_sent.extend_from_slice(packet);
            }

            Ok(false) => {
                println!("Arduino rejected the sent animation packet");
            }

            Err(error) => {
                println!("Failed to communicate with the Arduino because of this error: {}", error);
            }
        }
    }
}
