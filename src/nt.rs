use nt_client::data::NetworkTableData;
use nt_client::subscribe::ReceivedMessage;
use nt_client::Client;
use nt_client::NewClientOptions;
use nt_client::NTAddr;

use crate::animation::animation::Animation;
use crate::animation::larson_animation::LarsonAnimation;
use crate::animation::larson_animation::LarsonBounceMode;
use crate::animation::rainbow_animation::AnimationDirection;
use crate::animation::rainbow_animation::RainbowAnimation;
use crate::animation::solid_animation::SolidAnimation;
use crate::animation::strobe_animation::StrobeAnimation;
use crate::color::RGBWColor;

#[derive(Debug, Default)]
pub struct AnimationData {
    pub animation_type: String,
    pub start_index: u16,
    pub end_index: u16,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub white: u8,
    pub frame_rate: f64,
    pub size: u8,
    pub bounce_mode: u8,
    pub brightness: f64,
    pub direction: u8
}

impl AnimationData {
    pub fn to_animation(&self) -> Result<Animation, ()> {
        let color: RGBWColor = RGBWColor::new(self.red, self.green, self.blue, self.white);
        match self.animation_type.as_str() {
            "SolidColor" => {
                Ok(Animation::Solid(
                    SolidAnimation::new(self.start_index, self.end_index, color)
                ))
            }

            "StrobeAnimation" => {
                Ok(Animation::Strobe(
                    StrobeAnimation::new(self.start_index, self.end_index, color, self.frame_rate)
                ))
            }

            "LarsonAnimation" => {
                let bounce_mode: LarsonBounceMode = match self.bounce_mode {
                    0 => LarsonBounceMode::FRONT,
                    1 => LarsonBounceMode::CENTER,
                    2 => LarsonBounceMode::BACK,
                    _ => return Err(())
                };

                Ok(Animation::Larson(
                    LarsonAnimation::new(self.start_index, self.end_index, color, self.size, bounce_mode, self.frame_rate)
                ))
            }

            "RainbowAnimation" => {
                let direction: AnimationDirection = match self.direction {
                    0 => AnimationDirection::FORWARD,
                    1 => AnimationDirection::BACKWARD,
                    _ => return Err(())
                };

                Ok(Animation::Rainbow(
                    RainbowAnimation::new(self.start_index, self.end_index, color, self.brightness, direction, self.frame_rate)
                ))
            }

            _ => Err(())
        }
    }
}

pub async fn run() {
    let options = NewClientOptions {
        addr: NTAddr::Local,
        name: "driver-station-led".to_string(),
        ..Default::default()
    };

    let client = Client::new(options);
    println!("Connected to NetworkTables...");

    client.connect_setup(setup).await.unwrap();
}

fn setup(client: &Client) {
    let animation_type = client.topic("/Leds/Animation/Type");
    let start_index = client.topic("/Leds/Animation/StartIndex");
    let end_index = client.topic("Leds/Animation/EndIndex");

    let red = client.topic("Leds/Animation/Red");
    let green = client.topic("Leds/Animation/Green");
    let blue = client.topic("Leds/Animation/Blue");
    let white = client.topic("Leds/Anmation/White");

    let frame_rate = client.topic("Leds/Animation/FrameRate");
    let size = client.topic("Leds/Animation/Size");
    let bounce_mode = client.topic("Leds/Animation/BounceMode");

    let brightness = client.topic("Leds/Animation/Brightness");
    let direction = client.topic("Leds/Animation/Direction");

    tokio::spawn(async move {
        let mut animation_type_subscriber = animation_type.subscribe(Default::default()).await.unwrap();
        let mut start_index_subscriber = start_index.subscribe(Default::default()).await.unwrap();
        let mut end_index_subscriber = end_index.subscribe(Default::default()).await.unwrap();
        let mut red_subscriber = red.subscribe(Default::default()).await.unwrap();
        let mut green_subscriber = green.subscribe(Default::default()).await.unwrap();
        let mut blue_subscriber = blue.subscribe(Default::default()).await.unwrap();
        let mut white_subscriber = white.subscribe(Default::default()).await.unwrap();
        let mut frame_rate_subscriber = frame_rate.subscribe(Default::default()).await.unwrap();
        let mut size_subscriber = size.subscribe(Default::default()).await.unwrap();
        let mut bounce_mode_subscriber = bounce_mode.subscribe(Default::default()).await.unwrap();
        let mut brightness_subscriber = brightness.subscribe(Default::default()).await.unwrap();
        let mut direction_subscriber = direction.subscribe(Default::default()).await.unwrap();

        let mut data = AnimationData::default();

        loop {
            tokio::select! {
                message = animation_type_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = String::from_value(value) {
                            data.animation_type = value;    
                        }
                    }
                },

                message = start_index_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u16::from_value(value) {
                            data.start_index = value;
                        }
                    }
                },

                message = end_index_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u16::from_value(value) {
                            data.end_index = value;
                        }
                    }
                },

                message = red_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.red = value;
                        }
                    }
                },

                message = green_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.green = value;
                        }
                    }
                },

                message = blue_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.blue = value;
                        }
                    }
                },

                message = white_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.white = value;
                        }
                    }
                },

                message = frame_rate_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = f64::from_value(value) {
                            data.frame_rate = value;
                        }
                    }
                },

                message = size_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.size = value;
                        }
                    }
                },

                message = bounce_mode_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.bounce_mode = value;
                        }
                    }
                },

                message = brightness_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = f64::from_value(value) {
                            data.brightness = value;
                        }
                    }
                },

                message = direction_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.direction = value;
                        }
                    }
                }
            }
        }
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_solid_animation_data() {
        let data: AnimationData = AnimationData {
            animation_type: "SolidColor".to_string(),
            start_index: 0,
            end_index: 30,
            red: 255,
            green: 0,
            blue: 0,
            white: 0,
            frame_rate: 0.0,
            size: 0,
            bounce_mode: 0,
            brightness: 0.0,
            direction: 0
        };

        let animation: Animation = data.to_animation().unwrap();
        assert_eq!(animation, Animation::Solid(SolidAnimation::new(0, 30, RGBWColor::new(255, 0, 0, 0))));
    }

    #[test]
    fn convert_strobe_animation_data() {
        let data: AnimationData = AnimationData {
            animation_type: "StrobeAnimation".to_string(),
            start_index: 10,
            end_index: 70,
            red: 0,
            green: 255,
            blue: 0,
            white: 0,
            frame_rate: 30.0,
            size: 0,
            bounce_mode: 0,
            brightness: 0.0,
            direction: 0
        };

        let animation: Animation = data.to_animation().unwrap();
        assert_eq!(animation, Animation::Strobe(StrobeAnimation::new(10, 70, RGBWColor::new(0, 255, 0, 0), 30.0)));
    }

    #[test]
    fn convert_larson_animation_data() {
        let data: AnimationData = AnimationData {
            animation_type: "LarsonAnimation".to_string(),
            start_index: 20, 
            end_index: 80,
            red: 0,
            green: 0,
            blue: 255,
            white: 0,
            frame_rate: 35.0,
            size: 7,
            bounce_mode: 1,
            brightness: 0.0,
            direction: 0
        };

        let animation: Animation = data.to_animation().unwrap();
        assert_eq!(animation, Animation::Larson(LarsonAnimation::new(20, 80, RGBWColor::new(0, 0, 255, 0), 7, LarsonBounceMode::CENTER, 35.0)));
    }

    #[test]
    fn convert_rainbow_animation_data() {
        let data: AnimationData = AnimationData {
            animation_type: "RainbowAnimation".to_string(),
            start_index: 30,
            end_index: 50,
            red: 0,
            green: 0,
            blue: 0,
            white: 255,
            frame_rate: 20.0,
            size: 0,
            bounce_mode: 0,
            brightness: 1.0,
            direction: 1
        };

        let animation: Animation = data.to_animation().unwrap();
        assert_eq!(animation, Animation::Rainbow(RainbowAnimation::new(30, 50, RGBWColor::new(0, 0, 0, 255), 1.0, AnimationDirection::BACKWARD, 20.0)));
    }

    #[test]
    fn reject_unknown_animation() {
        let data: AnimationData = AnimationData {
            animation_type: "TwinkleAnimation".to_string(),
            start_index: 10, 
            end_index: 90,
            red: 245,
            green: 192,
            blue: 255,
            white: 9,
            frame_rate: 35.0,
            size: 7,
            bounce_mode: 1,
            brightness: 0.0,
            direction: 0
        };

        assert!(data.to_animation().is_err());
    }
}
