use crate::color::RGBWColor;

#[derive(Debug, PartialEq)]
pub struct SolidAnimation {
    pub start_index: u16,
    pub end_index: u16,
    pub color: RGBWColor
}

impl SolidAnimation {
    pub fn new(start_index: u16, end_index: u16, color: RGBWColor) -> SolidAnimation {
        self::SolidAnimation {
            start_index,
            end_index,
            color
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantize_animation_solid_color() {
        let animation: SolidAnimation = SolidAnimation::new(0, 89, RGBWColor::new(255, 10, 135, 100));

        assert_eq!(animation.start_index, 0);
        assert_eq!(animation.end_index, 89);
        assert_eq!(animation.color, RGBWColor::new(255, 10, 135, 100));
    }
}