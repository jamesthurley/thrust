pub struct GreyscaleLightnessOffset(pub i32);

impl From<i32> for GreyscaleLightnessOffset {
    fn from(lightness: i32) -> Self {
        assert!((-3..=5).contains(&lightness));
        Self(lightness)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities::assert_panic;

    use super::*;

    #[test]
    fn it_should_allow_offsets_within_range() {
        assert_panic!(GreyscaleLightnessOffset::from(6));
        assert_eq!(GreyscaleLightnessOffset::from(5).0, 5);
        assert_eq!(GreyscaleLightnessOffset::from(0).0, 0);
        assert_eq!(GreyscaleLightnessOffset::from(-3).0, -3);
        assert_panic!(GreyscaleLightnessOffset::from(-4));
    }
}
