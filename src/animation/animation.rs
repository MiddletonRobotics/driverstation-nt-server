use crate::animation::{
    solid_animation::SolidAnimation,
    strobe_animation::StrobeAnimation,
    larson_animation::LarsonAnimation,
    fade_animation::FadeAnimation,
    rainbow_animation::RainbowAnimation,
    fire_animation::FireAnimation
};

#[derive(Debug, PartialEq)]
pub enum Animation {
    Solid(SolidAnimation),
    Strobe(StrobeAnimation),
    Larson(LarsonAnimation),
    Fade(FadeAnimation),
    Rainbow(RainbowAnimation),
    Fire(FireAnimation)
}
