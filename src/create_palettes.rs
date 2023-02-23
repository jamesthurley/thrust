use crate::{
    palette::Palette, palette_definition::PaletteDefinition, palette_foundation::PaletteFoundation,
};

const WASH: PaletteFoundation = PaletteFoundation {
    name: "Wash",
    base_foreground_lightness: 0.93,
    foreground_chroma: 0.01,
    base_greyscale_lightness: 0.34,
    greyscale_lightness_scale_multiplier: 1.5,
    color_lightness_scale_multiplier: 1.15,
    color_chroma_scale_multiplier: 1.0,
};

const DARK: PaletteFoundation = PaletteFoundation {
    name: "Dark",
    base_foreground_lightness: 0.92,
    foreground_chroma: 0.005,
    base_greyscale_lightness: 0.19,
    greyscale_lightness_scale_multiplier: 0.9,
    color_lightness_scale_multiplier: 1.05,
    color_chroma_scale_multiplier: 1.0,
};

const STEALTH: PaletteFoundation = PaletteFoundation {
    name: "Stealth",
    base_foreground_lightness: 0.92,
    foreground_chroma: 0.005,
    base_greyscale_lightness: 0.22,
    greyscale_lightness_scale_multiplier: 0.9,
    color_lightness_scale_multiplier: 1.05,
    color_chroma_scale_multiplier: 1.0,
};

pub fn create_palettes() -> Vec<Palette> {
    let mut palette_definitions = vec![];
    for foundation in [&WASH, &DARK, &STEALTH] {
        let foundation_name = foundation.name;

        palette_definitions.push(
            PaletteDefinition::builder()
                .name(format!("Thrust Orange {foundation_name}").to_string())
                .foundation(foundation)
                .accent(foundation.orange())
                .build(),
        );

        palette_definitions.push(
            PaletteDefinition::builder()
                .name(format!("Thrust Red {foundation_name}").to_string())
                .foundation(foundation)
                .accent(foundation.red().adjust_chroma(6))
                .numbers(Box::new(|_| foundation.yellow().adjust_chroma(-6)))
                .types(Box::new(|c| c.adjust_chroma(-2)))
                .build(),
        );

        palette_definitions.push(
            PaletteDefinition::builder()
                .name(format!("Thrust Green {foundation_name}").to_string())
                .foundation(foundation)
                .accent(foundation.green())
                // .accent(foundation.hex(0x00FF41).adjust_chroma(-3).adjust_lightness(-2))
                .comments(Box::new(|_| {
                    foundation.turquoise().adjust_lightness(2).adjust_chroma(-3)
                }))
                .types(Box::new(|c| c.adjust_hue_degrees(60.)))
                .build(),
        );

        palette_definitions.push(
            PaletteDefinition::builder()
                .name(format!("Thrust Yellow {foundation_name}").to_string())
                .foundation(foundation)
                .accent(foundation.yellow().adjust_chroma(-3))
                .functions(Box::new(|c| c.adjust_hue_degrees(-30.)))
                .types(Box::new(|c| c.adjust_chroma(-2)))
                .build(),
        );
    }

    let palettes: Vec<Palette> =
        palette_definitions.into_iter().map(|definition| Palette { definition }).collect();

    palettes
}
