use crate::color::RGBWColor;

#[derive(Debug, PartialEq)]
pub struct FadeAnimation {
    pub start_index: u16,
    pub end_index: u16,
    pub color: RGBWColor,
    pub frame_rate: f64
}

impl FadeAnimation {
    pub fn new(start_index: u16, end_index: u16, color: RGBWColor, frame_rate: f64) -> FadeAnimation {
        Self {
            start_index,
            end_index,
            color,
            frame_rate
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    pub fn instantize_animation_fade() {
        let animation: FadeAnimation = FadeAnimation::new(0, 31, RGBWColor::new(152, 203, 76, 0), 17.9);

        assert_eq!(animation.start_index, 0);
        assert_eq!(animation.end_index, 31);
        assert_eq!(animation.color, RGBWColor::new(152, 203, 76, 0));
        assert_eq!(animation.frame_rate, 17.9);
    } 
}
