use nt_client::data::NetworkTableData;
use nt_client::subscribe::ReceivedMessage;
use nt_client::Client;
use nt_client::NewClientOptions;
use nt_client::NTAddr;

use crate::animation::animation::Animation;
use crate::animation::larson_animation::LarsonAnimation;
use crate::animation::larson_animation::LarsonBounceMode;
use crate::animation::rainbow_animation::{AnimationDirection as RainbowAnimationDirection};
use crate::animation::fire_animation::{AnimationDirection as FireAnimationDirection};
use crate::animation::rainbow_animation::RainbowAnimation;
use crate::animation::solid_animation::SolidAnimation;
use crate::animation::strobe_animation::StrobeAnimation;
use crate::animation::fade_animation::FadeAnimation;
use crate::animation::fire_animation::FireAnimation;
use crate::color::RGBWColor;

use tokio::sync::watch;
use tokio::time::{sleep, Duration};

#[derive(Debug, Default, Clone)]
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
    pub cooling: f64,
    pub sparkling: f64,
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

            "FadeAnimation" => {
                Ok(Animation::Fade(
                    FadeAnimation::new(self.start_index, self.end_index, color, self.frame_rate)
                ))
            }

            "RainbowAnimation" => {
                let direction: RainbowAnimationDirection = match self.direction {
                    0 => RainbowAnimationDirection::FORWARD,
                    1 => RainbowAnimationDirection::BACKWARD,
                    _ => return Err(())
                };

                Ok(Animation::Rainbow(
                    RainbowAnimation::new(self.start_index, self.end_index, color, self.brightness, direction, self.frame_rate)
                ))
            }

            "FireAnimation" => {
                let direction: FireAnimationDirection = match self.direction {
                    0 => FireAnimationDirection::FORWARD,
                    1 => FireAnimationDirection::BACKWARD,
                    _ => return Err(())
                };

                Ok(Animation::Fire(
                    FireAnimation::new(self.start_index, self.end_index, self.brightness, self.cooling, self.sparkling, direction, self.frame_rate)
                ))
            }

            _ => Err(())
        }
    }
}

pub async fn run(sender: watch::Sender<AnimationData>) {
    loop {
        let options = NewClientOptions {
            addr: NTAddr::Local,
            name: "driverstation-nt-server".to_string(),
            ..Default::default()
        };

        let client = Client::new(options);
        println!("Connecting to NetworkTable server...");

        let sender = sender.clone();
        let result = client.connect_setup(move |client| {
            println!("Connected and listening to NetworkTable server");
            setup(client, sender.clone());
        }).await;

        match result {
            Ok(_) => {
                println!("Disconnected from NetworkTable server");
            }

            Err(error) => {
                eprintln!("Disconnected from NetworkTable server: {}", error);
            }
        }   

        sleep(Duration::from_secs(1)).await;
    }
}

fn setup(client: &Client, sender: watch::Sender<AnimationData>) {
    let animation_type = client.topic("/AdvantageKit/RealOutputs/Leds/Animation/Type");
    let start_index = client.topic("/AdvantageKit/RealOutputs/Leds/Animation/StartIndex");
    let end_index = client.topic("/AdvantageKit/RealOutputs/Leds/Animation/EndIndex");

    let red = client.topic("/AdvantageKit/RealOutputs/Leds/Animation/Red");
    let green = client.topic("/AdvantageKit/RealOutputs/Leds/Animation/Green");
    let blue = client.topic("/AdvantageKit/RealOutputs/Leds/Animation/Blue");
    let white = client.topic("/AdvantageKit/RealOutputs/Leds/Anmation/White");

    let frame_rate = client.topic("/AdvantageKit/RealOutputs/Leds/Animation/FrameRate");
    let size = client.topic("/AdvantageKit/RealOutputs/Leds/Animation/Size");
    let bounce_mode = client.topic("/AdvantageKit/RealOutputs/Leds/Animation/BounceMode");

    let brightness = client.topic("/AdvantageKit/RealOutputs/Leds/Animation/Brightness");
    let direction = client.topic("/AdvantageKit/RealOutputs/Leds/Animation/Direction");

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
                            sender.send_replace(data.clone());

                            println!("Received new information from animaiton_type subscriber");
                        }
                    }
                },

                message = start_index_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u16::from_value(value) {
                            data.start_index = value;
                            sender.send_replace(data.clone());

                            println!("Received new information from start_index subscriber");
                        }
                    }
                },

                message = end_index_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u16::from_value(value) {
                            data.end_index = value;
                            sender.send_replace(data.clone());

                            println!("Received new information from end_index subscriber");
                        }
                    }
                },

                message = red_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.red = value;
                            sender.send_replace(data.clone());

                            println!("Received new information from red RGBW color subscriber");
                        }
                    }
                },

                message = green_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.green = value;
                            sender.send_replace(data.clone());

                            println!("Received new information from green RGBW color subscriber");
                        }
                    }
                },

                message = blue_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.blue = value;
                            sender.send_replace(data.clone());

                            println!("Received new information from blue RGBW color subscriber");
                        }
                    }
                },

                message = white_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.white = value;
                            sender.send_replace(data.clone());

                            println!("Received new information from white RGBW color subscriber");
                        }
                    }
                },

                message = frame_rate_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = f64::from_value(value) {
                            data.frame_rate = value;
                            sender.send_replace(data.clone());

                            println!("Received new information from frame rate subscriber");
                        }
                    }
                },

                message = size_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.size = value;
                            sender.send_replace(data.clone());

                            println!("Received new information from size subscriber");
                        }
                    }
                },

                message = bounce_mode_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.bounce_mode = value;
                            sender.send_replace(data.clone());

                            println!("Received new information from bounce mode subscriber");
                        }
                    }
                },

                message = brightness_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = f64::from_value(value) {
                            data.brightness = value;
                            sender.send_replace(data.clone());

                            println!("Received new information from brightness subscriber");
                        }
                    }
                },

                message = direction_subscriber.recv() => {
                    if let Ok(ReceivedMessage::Updated((_topic, value))) = message {
                        if let Some(value) = u8::from_value(value) {
                            data.direction = value;
                            sender.send_replace(data.clone());

                            println!("Recieved new information from direction subscriber")
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
            cooling: 0.0,
            sparkling: 0.0,
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
            cooling: 0.0,
            sparkling: 0.0,
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
            cooling: 0.0,
            sparkling: 0.0,
            direction: 0
        };

        let animation: Animation = data.to_animation().unwrap();
        assert_eq!(animation, Animation::Larson(LarsonAnimation::new(20, 80, RGBWColor::new(0, 0, 255, 0), 7, LarsonBounceMode::CENTER, 35.0)));
    }

    #[test]
    fn convert_fade_animation_data() {
        let data: AnimationData = AnimationData {
            animation_type: "FadeAnimation".to_string(),
            start_index: 20,
            end_index: 25,
            red: 204,
            green: 109,
            blue: 212,
            white: 72,
            frame_rate: 19.3,
            size: 0,
            bounce_mode: 0,
            brightness: 0.0,
            cooling: 0.0,
            sparkling: 0.0,
            direction: 0
        };

        let animation: Animation = data.to_animation().unwrap();
        assert_eq!(animation, Animation::Fade(FadeAnimation::new(20, 25, RGBWColor::new(204, 109, 212, 72), 19.3)));
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
            cooling: 0.0,
            sparkling: 0.0,
            direction: 1
        };

        let animation: Animation = data.to_animation().unwrap();
        assert_eq!(animation, Animation::Rainbow(RainbowAnimation::new(30, 50, RGBWColor::new(0, 0, 0, 255), 1.0, RainbowAnimationDirection::BACKWARD, 20.0)));
    }

    #[test]
    fn convert_fire_animation_data() {
        let data: AnimationData = AnimationData {
            animation_type: "FireAnimation".to_string(),
            start_index: 0,
            end_index: 119,
            red: 119,
            green: 20,
            blue: 255,
            white: 255,
            frame_rate: 10.6,
            size: 0,
            bounce_mode: 0,
            brightness: 0.68,
            cooling: 0.18,
            sparkling: 0.01,
            direction: 1
        };

        let animation: Animation = data.to_animation().unwrap();
        assert_eq!(animation, Animation::Fire(FireAnimation::new(0, 119, 0.68, 0.18, 0.01, FireAnimationDirection::BACKWARD, 10.6)));
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
            cooling: 0.0,
            sparkling: 0.0,
            direction: 0
        };

        assert!(data.to_animation().is_err());
    }
}
