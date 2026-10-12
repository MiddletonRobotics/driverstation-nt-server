use crate::color::RGBWColor;

#[derive(Debug, PartialEq)]
pub enum LarsonBounceMode {
    FRONT,
    CENTER,
    BACK
}

#[derive(Debug, PartialEq)]
pub struct LarsonAnimation {
    pub start_index: u16,
    pub end_index: u16,
    pub color: RGBWColor,
    pub size: u8,
    pub bounce_mode: LarsonBounceMode,
    pub frame_rate: f64
}

impl LarsonAnimation {
    pub fn new(start_index: u16, end_index: u16, color: RGBWColor, size: u8, bounce_mode: LarsonBounceMode, frame_rate: f64) -> LarsonAnimation {
        self::LarsonAnimation {
            start_index,
            end_index,
            color,
            size,
            bounce_mode,
            frame_rate
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantize_animation_larson() {
        let animation: LarsonAnimation = LarsonAnimation::new(0, 89, RGBWColor::new(255, 10, 135, 5), 10, LarsonBounceMode::FRONT, 60.0);

        assert_eq!(animation.start_index, 0);
        assert_eq!(animation.end_index, 89);
        assert_eq!(animation.color, RGBWColor::new(255, 10, 135, 5));
        assert_eq!(animation.size, 10);
        assert_eq!(animation.bounce_mode, LarsonBounceMode::FRONT);
        assert_eq!(animation.frame_rate, 60.0)
    }
}