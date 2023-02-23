use crate::{
    color_spaces::Oklch, greyscale_lightness_offset::GreyscaleLightnessOffset,
    palette_oklch::PaletteOklch,
};

pub struct PaletteFoundation {
    pub name: &'static str,
    pub base_foreground_lightness: f32,
    pub foreground_chroma: f32,
    pub base_greyscale_lightness: f32,
    pub greyscale_lightness_scale_multiplier: f32,
    pub color_lightness_scale_multiplier: f32,
    pub color_chroma_scale_multiplier: f32,
}

impl PaletteFoundation {
    pub fn create_palette_color(&self, color: Oklch) -> PaletteOklch {
        PaletteOklch::new(
            color,
            self.color_lightness_scale_multiplier,
            self.color_chroma_scale_multiplier,
        )
    }

    pub fn create_foreground_from_color(&self, color: Oklch) -> PaletteOklch {
        self.create_palette_color(Oklch::from_radians(
            self.base_foreground_lightness,
            self.foreground_chroma,
            color.h_radians(),
        ))
    }

    #[allow(dead_code)]
    pub fn hex(&self, value: u32) -> PaletteOklch {
        self.create_palette_color(Oklch::from(value))
    }

    pub fn greyscale(&self, lightness_offset: impl Into<GreyscaleLightnessOffset>) -> PaletteOklch {
        let color = Oklch::from_degrees(self.greyscale_lightness(lightness_offset), 0.0, 0.0);
        self.create_palette_color(color)
    }

    fn greyscale_lightness(&self, lightness_offset: impl Into<GreyscaleLightnessOffset>) -> f32 {
        let lightness = lightness_offset.into();

        match lightness.0 {
            -3 => self.base_greyscale_lightness - 0.2 * self.greyscale_lightness_scale_multiplier,
            -2 => self.base_greyscale_lightness - 0.07 * self.greyscale_lightness_scale_multiplier,
            -1 => self.base_greyscale_lightness - 0.025 * self.greyscale_lightness_scale_multiplier,
            0 => self.base_greyscale_lightness,
            1 => self.base_greyscale_lightness + 0.03 * self.greyscale_lightness_scale_multiplier,
            2 => self.base_greyscale_lightness + 0.06 * self.greyscale_lightness_scale_multiplier,
            3 => self.base_greyscale_lightness + 0.13 * self.greyscale_lightness_scale_multiplier,
            4 => self.base_greyscale_lightness + 0.18 * self.greyscale_lightness_scale_multiplier,
            5 => self.base_greyscale_lightness + 0.33 * self.greyscale_lightness_scale_multiplier,
            _ => unreachable!(),
        }
    }
}
