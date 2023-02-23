pub struct ColorLightnessOffset(pub i32);

impl From<i32> for ColorLightnessOffset {
    fn from(lightness: i32) -> Self {
        assert!((-10..=10).contains(&lightness));
        Self(lightness)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities::assert_panic;

    use super::*;

    #[test]
    fn it_should_allow_offsets_within_range() {
        assert_panic!(ColorLightnessOffset::from(11));
        assert_eq!(ColorLightnessOffset::from(10).0, 10);
        assert_eq!(ColorLightnessOffset::from(0).0, 0);
        assert_eq!(ColorLightnessOffset::from(-10).0, -10);
        assert_panic!(ColorLightnessOffset::from(-11));
    }
}
