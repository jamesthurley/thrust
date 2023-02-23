pub struct ColorChromaOffset(pub i32);

impl From<i32> for ColorChromaOffset {
    fn from(chroma: i32) -> Self {
        assert!((-10..=10).contains(&chroma));
        Self(chroma)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities::assert_panic;

    use super::*;

    #[test]
    fn it_should_allow_offsets_within_range() {
        assert_panic!(ColorChromaOffset::from(11));
        assert_eq!(ColorChromaOffset::from(10).0, 10);
        assert_eq!(ColorChromaOffset::from(0).0, 0);
        assert_eq!(ColorChromaOffset::from(-10).0, -10);
        assert_panic!(ColorChromaOffset::from(-11));
    }
}
