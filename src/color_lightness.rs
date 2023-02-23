pub struct ColorLightness(pub f32);

impl From<f32> for ColorLightness {
    fn from(lightness: f32) -> Self {
        assert!((0.0..=1.0).contains(&lightness));
        Self(lightness)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities::assert_panic;

    use super::*;

    #[test]
    fn it_should_allow_offsets_within_range() {
        assert_panic!(ColorLightness::from(-0.001));
        assert_eq!(ColorLightness::from(0.).0, 0.);
        assert_eq!(ColorLightness::from(0.5).0, 0.5);
        assert_eq!(ColorLightness::from(1.).0, 1.);
        assert_panic!(ColorLightness::from(1.001));
    }
}
