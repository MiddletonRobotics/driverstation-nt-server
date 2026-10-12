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

#[tokio::main]
async fn main() {
    nt::run().await;
}