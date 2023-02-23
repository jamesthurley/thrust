use crate::{
    color_chroma::ColorChroma,
    color_chroma_offset::ColorChromaOffset,
    color_lightness::ColorLightness,
    color_lightness_offset::ColorLightnessOffset,
    color_spaces::{Oklch, U8Srgb},
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct PaletteOklch {
    pub color: Oklch,
    pub lightness_scale_multiplier: f32,
    pub chroma_scale_multiplier: f32,
}

impl From<PaletteOklch> for mottle::dsl::Color {
    fn from(value: PaletteOklch) -> Self {
        let u8srgb = U8Srgb::from(value.color);
        mottle::dsl::Color::from(((u8srgb.r, u8srgb.g, u8srgb.b), 0xFF))
    }
}

impl From<PaletteOklch> for (u8, u8, u8) {
    fn from(value: PaletteOklch) -> Self {
        value.color.into()
    }
}

impl PaletteOklch {
    pub fn new(
        color: Oklch,
        lightness_scale_multiplier: f32,
        chroma_scale_multiplier: f32,
    ) -> Self {
        Self { color, lightness_scale_multiplier, chroma_scale_multiplier }
    }

    fn get_new_lightness(&self, lightness_offset: impl Into<ColorLightnessOffset>) -> f32 {
        let lightness_offset = lightness_offset.into().0;
        let result =
            self.color.l * (1. + (0.1 * self.lightness_scale_multiplier)).powi(lightness_offset);
        result.clamp(0.0, 1.0)
    }

    fn get_new_chroma(&self, chroma_offset: impl Into<ColorChromaOffset>) -> f32 {
        let chroma_offset = chroma_offset.into().0;
        let result = self.color.c * (1. + (0.1 * self.chroma_scale_multiplier)).powi(chroma_offset);
        result.clamp(0.0, crate::CHROMA_MAX)
    }

    #[must_use]
    pub fn adjust_lightness(&self, lightness_offset: impl Into<ColorLightnessOffset>) -> Self {
        let mut result = self.clone();
        result.color.l = self.get_new_lightness(lightness_offset);
        result
    }

    #[must_use]
    pub fn with_lightness(&self, lightness: impl Into<ColorLightness>) -> Self {
        let mut result = self.clone();
        result.color.l = lightness.into().0;
        result
    }

    #[must_use]
    pub fn adjust_chroma(&self, chroma_offset: impl Into<ColorChromaOffset>) -> Self {
        let mut result = self.clone();
        result.color.c = self.get_new_chroma(chroma_offset);
        result
    }

    #[must_use]
    pub fn with_chroma(&self, chroma: impl Into<ColorChroma>) -> Self {
        let mut result = self.clone();
        result.color.c = chroma.into().0;
        result
    }

    #[must_use]
    pub fn adjust_hue_degrees(&self, hue_offset: f32) -> Self {
        let mut result = self.clone();
        let new_hue = result.color.h_degrees() + hue_offset;
        result.color.set_h_degrees(new_hue);
        result
    }

    #[must_use]
    pub fn with_hue_degrees(&self, hue_degrees: f32) -> Self {
        let mut result = self.clone();
        result.color.set_h_degrees(hue_degrees);
        result
    }

    #[must_use]
    pub fn with_alpha(&self, alpha: u8) -> mottle::dsl::Color {
        let u8srgb = U8Srgb::from(self.color);
        mottle::dsl::Color::from(((u8srgb.r, u8srgb.g, u8srgb.b), alpha))
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::test_utilities::assert_panic;

    use super::*;

    #[test]
    fn adjust_lightness() {
        let color = Oklch::from_radians(0.5, 0.25, 1.0);
        let p = PaletteOklch::new(color, 1.0, 1.0);

        let p_lighter_1 = p.adjust_lightness(1);
        assert!(p.color.l < p_lighter_1.color.l);

        let p_lighter_2 = p.adjust_lightness(2);
        assert!(p_lighter_1.color.l < p_lighter_2.color.l);

        let p_darker_1 = p.adjust_lightness(-1);
        assert!(p.color.l > p_darker_1.color.l);

        let p_darker_2 = p.adjust_lightness(-2);
        assert!(p_darker_1.color.l > p_darker_2.color.l);
    }

    #[test]
    fn adjust_lightness_with_multiplier() {
        let color = Oklch::from_radians(0.5, 0.25, 1.0);
        let p1 = PaletteOklch::new(color, 1.0, 1.0);
        let p2 = PaletteOklch::new(color, 2.0, 1.0);

        let p_lighter_1 = p1.adjust_lightness(1);
        let p_lighter_2 = p2.adjust_lightness(1);
        assert!(p_lighter_1.color.l < p_lighter_2.color.l);

        let p_darker_1 = p1.adjust_lightness(-1);
        let p_darker_2 = p2.adjust_lightness(-1);
        assert!(p_darker_1.color.l > p_darker_2.color.l);
    }

    #[test]
    fn adjust_lightness_should_clamp() {
        let p_light = PaletteOklch::new(Oklch::from_radians(0.99, 0.25, 1.0), 1.0, 1.0);
        assert_eq!(p_light.adjust_lightness(10).color.l, 1.0);

        let p_dark = PaletteOklch::new(Oklch::from_radians(0.0, 0.25, 1.0), 1.0, 1.0);
        assert_eq!(p_dark.adjust_lightness(-10).color.l, 0.0);
    }

    #[test]
    fn with_lightness() {
        let color = Oklch::from_radians(0.5, 0.25, 1.0);
        let p = PaletteOklch::new(color, 1.0, 1.0);

        assert_eq!(p.with_lightness(0.9).color.l, 0.9);
        assert_eq!(p.with_lightness(1.0).color.l, 1.0);
        assert_eq!(p.with_lightness(0.0).color.l, 0.0);

        assert_panic!(p.with_lightness(1.1));
        assert_panic!(p.with_lightness(-0.1));
    }

    #[test]
    fn adjust_chroma() {
        let color = Oklch::from_radians(0.5, 0.25, 1.0);
        let p = PaletteOklch::new(color, 1.0, 1.0);

        let p_saturated_1 = p.adjust_chroma(1);
        assert!(p.color.c < p_saturated_1.color.c);

        let p_saturated_2 = p.adjust_chroma(2);
        assert!(p_saturated_1.color.c < p_saturated_2.color.c);

        let p_desaturated_1 = p.adjust_chroma(-1);
        assert!(p.color.c > p_desaturated_1.color.c);

        let p_desaturated_2 = p.adjust_chroma(-2);
        assert!(p_desaturated_1.color.c > p_desaturated_2.color.c);
    }

    #[test]
    fn adjust_chroma_with_multiplier() {
        let color = Oklch::from_radians(0.5, 0.25, 1.0);
        let p1 = PaletteOklch::new(color, 1.0, 1.0);
        let p2 = PaletteOklch::new(color, 1.0, 2.0);

        let p_saturated_1 = p1.adjust_chroma(1);
        let p_saturated_2 = p2.adjust_chroma(1);
        assert!(p_saturated_1.color.c < p_saturated_2.color.c);

        let p_desaturated_1 = p1.adjust_chroma(-1);
        let p_desaturated_2 = p2.adjust_chroma(-1);
        assert!(p_desaturated_1.color.c > p_desaturated_2.color.c);
    }

    #[test]
    fn adjust_chroma_should_clamp() {
        let p_light = PaletteOklch::new(
            Oklch::from_radians(0.25, crate::CHROMA_MAX - crate::COLOR_EPSILON, 1.0),
            1.0,
            1.0,
        );
        assert_eq!(p_light.adjust_chroma(10).color.c, crate::CHROMA_MAX);

        let p_dark = PaletteOklch::new(Oklch::from_radians(0.5, 0.0, 1.0), 1.0, 1.0);
        assert_eq!(p_dark.adjust_chroma(-10).color.c, 0.0);
    }

    #[test]
    fn with_chroma() {
        let color = Oklch::from_radians(0.5, 0.25, 1.0);
        let p = PaletteOklch::new(color, 1.0, 1.0);

        assert_eq!(p.with_chroma(0.4).color.c, 0.4);
        assert_eq!(p.with_chroma(crate::CHROMA_MAX).color.c, crate::CHROMA_MAX);
        assert_eq!(p.with_chroma(0.0).color.c, 0.0);

        assert_panic!(p.with_chroma(crate::CHROMA_MAX + 0.1));
        assert_panic!(p.with_chroma(-0.1));
    }

    #[test]
    fn adjust_hue_degrees() {
        let color = Oklch::from_radians(0.5, 0.25, 1.0);
        let p = PaletteOklch::new(color, 1.0, 1.0);

        assert_approx_eq!(
            f32,
            p.color.h_degrees() + 1.0,
            p.adjust_hue_degrees(1.0).color.h_degrees()
        );

        assert_approx_eq!(
            f32,
            p.color.h_degrees() - 1.0,
            p.adjust_hue_degrees(-1.0).color.h_degrees()
        );

        assert_approx_eq!(
            f32,
            p.color.h_degrees() + 1000.0,
            p.adjust_hue_degrees(1000.0).color.h_degrees()
        );

        assert_approx_eq!(
            f32,
            p.color.h_degrees() - 1000.0,
            p.adjust_hue_degrees(-1000.0).color.h_degrees()
        );
    }

    #[test]
    fn with_hue_degrees() {
        let color = Oklch::from_radians(0.5, 0.25, 1.0);
        let p = PaletteOklch::new(color, 1.0, 1.0);

        assert_approx_eq!(f32, 254.6, p.with_hue_degrees(254.6).color.h_degrees());

        assert_approx_eq!(f32, -1254.6, p.with_hue_degrees(-1254.6).color.h_degrees());
    }
}
