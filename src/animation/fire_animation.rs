#[derive(Debug, PartialEq)]
pub enum AnimationDirection {
    FORWARD,
    BACKWARD
}

#[derive(Debug, PartialEq)]
pub struct FireAnimation {
    pub start_index: u16,
    pub end_index: u16,
    pub brightness: f64,
    pub cooling: f64,
    pub sparkling: f64,
    pub direction: AnimationDirection,
    pub frame_rate: f64
}

impl FireAnimation {
    pub fn new(start_index: u16, end_index: u16, brightness: f64, cooling: f64, sparkling: f64, direction: AnimationDirection, frame_rate: f64) -> FireAnimation {
        self::FireAnimation {
            start_index,
            end_index,
            brightness,
            cooling,
            sparkling,
            direction,
            frame_rate
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn instantize_animation_fire() {
        let animation: FireAnimation = FireAnimation::new(0, 10, 0.76, 0.33, 0.12, AnimationDirection::FORWARD, 82.9);

        assert_eq!(animation.start_index, 0);
        assert_eq!(animation.end_index, 10);
        assert_eq!(animation.brightness, 0.76);
        assert_eq!(animation.cooling, 0.33);
        assert_eq!(animation.sparkling, 0.12);
        assert_eq!(animation.direction, AnimationDirection::FORWARD);
        assert_eq!(animation.frame_rate, 82.9);
    } 
}
