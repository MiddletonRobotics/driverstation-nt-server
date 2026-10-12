use crate::animation::{
    solid_animation::SolidAnimation,
    strobe_animation::StrobeAnimation,
    larson_animation::LarsonAnimation,
    rainbow_animation::RainbowAnimation,
};

#[derive(Debug, PartialEq)]
pub enum Animation {
    Solid(SolidAnimation),
    Strobe(StrobeAnimation),
    Larson(LarsonAnimation),
    Rainbow(RainbowAnimation),
}
