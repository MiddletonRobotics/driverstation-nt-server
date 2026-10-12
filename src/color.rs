#[derive(Debug, PartialEq)]
pub struct RGBWColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub white: u8,
}

impl RGBWColor {
    pub fn new(red: u8, green: u8, blue: u8, white: u8) -> Self {
        Self {
            red,
            green,
            blue,
            white,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantize_rgbw_color() {
        let color = RGBWColor::new(255, 100, 25, 200);

        assert_eq!(color.red, 255);
        assert_eq!(color.green, 100);
        assert_eq!(color.blue, 25);
        assert_eq!(color.white, 200);
    }
}