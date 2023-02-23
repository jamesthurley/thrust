pub struct ColorChroma(pub f32);

impl From<f32> for ColorChroma {
    fn from(chroma: f32) -> Self {
        assert!((0.0..=crate::CHROMA_MAX).contains(&chroma));
        Self(chroma)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utilities::assert_panic;

    use super::*;

    #[test]
    fn it_should_allow_offsets_within_range() {
        assert_panic!(ColorChroma::from(-0.001));
        assert_eq!(ColorChroma::from(0.).0, 0.);
        assert_eq!(ColorChroma::from(0.5).0, 0.5);
        assert_eq!(ColorChroma::from(crate::CHROMA_MAX).0, crate::CHROMA_MAX);
        assert_panic!(ColorChroma::from(crate::CHROMA_MAX + crate::COLOR_EPSILON));
    }
}
