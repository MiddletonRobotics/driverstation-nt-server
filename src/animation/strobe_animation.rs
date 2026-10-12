use crate::color::RGBWColor;

#[derive(Debug, PartialEq)]
pub struct StrobeAnimation {
    pub start_index: u16,
    pub end_index: u16,
    pub color: RGBWColor,
    pub frame_rate: f64
}

impl StrobeAnimation {
    pub fn new(start_index: u16, end_index: u16, color: RGBWColor, frame_rate: f64) -> StrobeAnimation {
        self::StrobeAnimation {
            start_index,
            end_index,
            color,
            frame_rate
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantize_animation_strobe() {
        let animation: StrobeAnimation = StrobeAnimation::new(0, 89, RGBWColor::new(255, 10, 135, 5), 60.0);

        assert_eq!(animation.start_index, 0);
        assert_eq!(animation.end_index, 89);
        assert_eq!(animation.color, RGBWColor::new(255, 10, 135, 5));
        assert_eq!(animation.frame_rate, 60.0);
    }
}