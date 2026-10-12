use crate::color::RGBWColor;

#[derive(Debug, PartialEq)]
pub enum AnimationDirection {
    FORWARD,
    BACKWARD
}

#[derive(Debug, PartialEq)]
pub struct RainbowAnimation {
    pub start_index: u16,
    pub end_index: u16,
    pub color: RGBWColor,
    pub brightness: f64,
    pub direction: AnimationDirection,
    pub frame_rate: f64
}

impl RainbowAnimation {
    pub fn new(start_index: u16, end_index: u16, color: RGBWColor, brightness: f64, direction: AnimationDirection, frame_rate: f64) -> RainbowAnimation {
        self::RainbowAnimation {
            start_index,
            end_index,
            color,
            brightness,
            direction,
            frame_rate
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantize_animation_rainbow() {
        let animation: RainbowAnimation = RainbowAnimation::new(0, 89, RGBWColor::new(255, 10, 135, 200), 1.0, AnimationDirection::BACKWARD, 60.0);

        assert_eq!(animation.start_index, 0);
        assert_eq!(animation.end_index, 89);
        assert_eq!(animation.color, RGBWColor::new(255, 10, 135, 200));
        assert_eq!(animation.brightness, 1.0);
        assert_eq!(animation.direction, AnimationDirection::BACKWARD);
        assert_eq!(animation.frame_rate, 60.0);
    }
}