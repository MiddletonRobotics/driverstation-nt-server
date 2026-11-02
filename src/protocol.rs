use crate::animation::animation::Animation;

pub const BUFFER_ARRAY_SIZE: usize = 22;

pub fn encode(animation: &Animation, buffer: &mut [u8]) -> std::result::Result<usize, ()> {
    let mut i: usize = 0;

    match animation {
        Animation::Solid(solid) => {
            buffer[i] = 0;
            i += 1;

            buffer[i..i + 2].copy_from_slice(&solid.start_index.to_le_bytes());
            i += 2;

            buffer[i..i + 2].copy_from_slice(&solid.end_index.to_le_bytes());
            i += 2;

            buffer[i] = solid.color.red;
            i += 1;

            buffer[i] = solid.color.green;
            i += 1;

            buffer[i] = solid.color.blue;
            i += 1;

            buffer[i] = solid.color.white;
            i += 1;
        }

        Animation::Strobe(strobe) => {
            buffer[i] = 1;
            i += 1;

            buffer[i..i + 2].copy_from_slice(&strobe.start_index.to_le_bytes());
            i += 2;

            buffer[i..i + 2].copy_from_slice(&strobe.end_index.to_le_bytes());
            i += 2;

            buffer[i] = strobe.color.red;
            i += 1;

            buffer[i] = strobe.color.green;
            i += 1;

            buffer[i] = strobe.color.blue;
            i += 1;

            buffer[i] = strobe.color.white;
            i += 1;

            buffer[i..i + 8].copy_from_slice(&strobe.frame_rate.to_le_bytes());
            i += 8;
        }

        Animation::Larson(larson) => {
            buffer[i] = 2;
            i += 1;

            buffer[i..i + 2].copy_from_slice(&larson.start_index.to_le_bytes());
            i += 2;

            buffer[i..i + 2].copy_from_slice(&larson.end_index.to_le_bytes());
            i += 2;

            buffer[i] = larson.color.red;
            i += 1;

            buffer[i] = larson.color.green;
            i += 1;

            buffer[i] = larson.color.blue;
            i += 1;

            buffer[i] = larson.color.white;
            i += 1;

            buffer[i..i + 8].copy_from_slice(&larson.frame_rate.to_le_bytes());
            i += 8;

            buffer[i] = larson.size;
            i += 1;

            match larson.bounce_mode {
                crate::animation::larson_animation::LarsonBounceMode::FRONT => buffer[i] = 0,
                crate::animation::larson_animation::LarsonBounceMode::CENTER => buffer[i] = 1,
                crate::animation::larson_animation::LarsonBounceMode::BACK => buffer[i] = 2,
            }

            i += 1;
        }

        Animation::Rainbow(rainbow) => {
            buffer[i] = 3;
            i += 1;

            buffer[i..i + 2].copy_from_slice(&rainbow.start_index.to_le_bytes());
            i += 2;

            buffer[i..i + 2].copy_from_slice(&rainbow.end_index.to_le_bytes());
            i += 2;

            buffer[i..i + 8].copy_from_slice(&rainbow.frame_rate.to_le_bytes());
            i += 8;

            buffer[i..i + 8].copy_from_slice(&rainbow.brightness.to_le_bytes());
            i += 8;

            match rainbow.direction {
                crate::animation::rainbow_animation::AnimationDirection::FORWARD => buffer[i] = 0,
                crate::animation::rainbow_animation::AnimationDirection::BACKWARD => buffer[i] = 1,
            }

            i += 1;
        }
    }

    println!("{buffer:?}");

    Ok(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::color::RGBWColor;
    use crate::animation::solid_animation::SolidAnimation;
    use crate::animation::strobe_animation::StrobeAnimation;
    use crate::animation::larson_animation::LarsonAnimation;
    use crate::animation::larson_animation::LarsonBounceMode;
    use crate::animation::rainbow_animation::RainbowAnimation;
    use crate::animation::rainbow_animation::AnimationDirection;

    #[test]
    fn encode_solid_animation() {
        let animation = Animation::Solid(SolidAnimation::new(2, 30, RGBWColor::new(255, 198, 219, 255)));
        let mut buffer: [u8; BUFFER_ARRAY_SIZE] = [0u8; BUFFER_ARRAY_SIZE];
        let _ = encode(&animation, &mut buffer);

        assert_eq!(buffer, [0, 2, 0, 30, 0, 255, 198, 219, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]

    fn encode_strobe_animation() {
        let animation = Animation::Strobe(StrobeAnimation::new(0, 80, RGBWColor::new(18, 235, 200, 147), 120.0));
        let mut buffer: [u8; BUFFER_ARRAY_SIZE] = [0u8; BUFFER_ARRAY_SIZE];
        let _ = encode(&animation, &mut buffer);

        assert_eq!(buffer, [1, 0, 0, 80, 0, 18, 235, 200, 147, 0, 0, 0, 0, 0, 0, 94, 64, 0, 0, 0, 0, 0]);
    }

    #[test]

    fn encode_larson_animation() {
        let animation = Animation::Larson(LarsonAnimation::new(2, 103, RGBWColor::new(187, 176, 245, 198), 7, LarsonBounceMode::CENTER, 120.0));
        let mut buffer: [u8; BUFFER_ARRAY_SIZE] = [0u8; BUFFER_ARRAY_SIZE];
        let _ = encode(&animation, &mut buffer);

        assert_eq!(buffer, [2, 2, 0, 103, 0, 187, 176, 245, 198, 0, 0, 0, 0, 0, 0, 94, 64, 7, 1, 0, 0, 0]);
    }

    #[test]

    fn encode_rainbow_animation() {
        let animation = Animation::Rainbow(RainbowAnimation::new(2, 103, RGBWColor::new(187, 176, 245, 198), 0.996, AnimationDirection::FORWARD, 90.0));
        let mut buffer: [u8; BUFFER_ARRAY_SIZE] = [0u8; BUFFER_ARRAY_SIZE];
        let _ = encode(&animation, &mut buffer);

        assert_eq!(buffer, [3, 2, 0, 103, 0, 0, 0, 0, 0, 0, 128, 86, 64, 172, 28, 90, 100, 59, 223, 239, 63, 0]);
    }
}
