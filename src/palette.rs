use crate::{
    color_lightness_offset::ColorLightnessOffset,
    greyscale_lightness_offset::GreyscaleLightnessOffset, palette_definition::PaletteDefinition,
    palette_foundation::PaletteFoundation, palette_oklch::PaletteOklch,
};

pub struct Palette {
    pub definition: PaletteDefinition,
}

impl Palette {
    pub fn foundation(&self) -> &'static PaletteFoundation {
        self.definition.foundation
    }

    pub fn accent(&self) -> PaletteOklch {
        self.definition.accent.clone()
    }

    pub fn foreground(&self) -> PaletteOklch {
        (self.definition.foreground)(
            self.foundation()
                .create_foreground_from_color(self.accent().adjust_hue_degrees(15.).color),
        )
    }

    pub fn bright_foreground(&self) -> PaletteOklch {
        self.foreground()
            .with_lightness((self.foundation().base_foreground_lightness + 0.09).min(0.99))
            .with_chroma(self.foundation().foreground_chroma)
    }

    pub fn keywords(&self) -> PaletteOklch {
        self.accent()
    }

    pub fn variables(&self) -> PaletteOklch {
        self.foreground()
    }

    pub fn functions(&self) -> PaletteOklch {
        (self.definition.functions)(self.foreground().with_lightness(1.0).with_chroma(0.05))
    }

    pub fn operators(&self) -> PaletteOklch {
        self.accent().adjust_lightness(1).adjust_chroma(-2)
    }

    pub fn types(&self) -> PaletteOklch {
        (self.definition.types)(
            self.accent().adjust_hue_degrees(180.).with_lightness(0.76).adjust_chroma(-3),
        )
    }

    pub fn interfaces(&self) -> PaletteOklch {
        self.types().adjust_lightness(-1)
    }

    pub fn constants(&self) -> PaletteOklch {
        (self.definition.constants)(self.blue().adjust_lightness(1))
    }

    pub fn enum_members(&self) -> PaletteOklch {
        self.constants()
    }

    pub fn properties(&self) -> PaletteOklch {
        self.foreground()
    }

    pub fn namespaces(&self) -> PaletteOklch {
        self.foreground().adjust_lightness(-1)
    }

    pub fn this(&self) -> PaletteOklch {
        self.numbers()
    }

    pub fn strings(&self) -> PaletteOklch {
        (self.definition.strings)(self.blue().adjust_lightness(6).adjust_chroma(-6))
    }

    pub fn numbers(&self) -> PaletteOklch {
        (self.definition.numbers)(self.maroon().adjust_lightness(5).adjust_chroma(-4))
    }

    pub fn macros(&self) -> PaletteOklch {
        self.foreground().adjust_lightness(-1)
    }

    pub fn comments(&self) -> PaletteOklch {
        (self.definition.comments)(self.green().adjust_lightness(-2))
    }

    pub fn greyscale(&self, lightness_offset: impl Into<GreyscaleLightnessOffset>) -> PaletteOklch {
        self.foundation().greyscale(lightness_offset)
    }
}

pub const TERMINAL_ANSI_LIGHTNESS: ColorLightnessOffset = ColorLightnessOffset(-1);
pub const TERMINAL_ANSI_BRIGHT_LIGHTNESS: ColorLightnessOffset = ColorLightnessOffset(1);
pub const DIFF_FG_LIGHTNESS: ColorLightnessOffset = ColorLightnessOffset(0);
pub const DIFF_BG_LIGHTNESS: ColorLightnessOffset = ColorLightnessOffset(-2);
pub const GUTTER_LIGHTNESS: ColorLightnessOffset = ColorLightnessOffset(-2);
pub const OVERVIEW_RULER_LIGHTNESS: ColorLightnessOffset = ColorLightnessOffset(-2);
pub const GIT_DECORATION_LIGHTNESS: ColorLightnessOffset = ColorLightnessOffset(-2);
pub const MINIMAP_LIGHTNESS: ColorLightnessOffset = ColorLightnessOffset(0);
pub const STATUS_BAR_LIGHTNESS: ColorLightnessOffset = ColorLightnessOffset(-1);
pub const ERROR_LENS_BACKGROUND_LIGHTNESS: ColorLightnessOffset = ColorLightnessOffset(-2);
pub const ERROR_LENS_FOREGROUND_LIGHTNESS: ColorLightnessOffset = ColorLightnessOffset(1);
