mod color_chroma;
mod color_chroma_offset;
mod color_lightness;
mod color_lightness_offset;
mod color_spaces;
mod colors;
mod create_palettes;
mod greyscale_lightness_offset;
mod imp;
mod palette;
mod palette_definition;
mod palette_foundation;
mod palette_oklch;
mod test_utilities;

use create_palettes::create_palettes;
use mottle::dsl::ThemeBuilder;
use palette::Palette;
use palette_foundation::PaletteFoundation;

pub const COLOR_EPSILON: f32 = 0.001;
pub const CHROMA_MAX: f32 = 0.5;

fn main() -> anyhow::Result<()> {
    let palettes = create_palettes();

    let reference_palette = &palettes[4];
    print_base_colors(reference_palette.definition.foundation);
    print_reference_palette(reference_palette);
    print_themes_json(&palettes);

    for palette in palettes {
        gen_and_save_theme(palette)?;
    }

    Ok(())
}

fn gen_and_save_theme(palette: Palette) -> anyhow::Result<()> {
    let mut theme_builder = ThemeBuilder::default();
    let name = palette.definition.name.clone();

    imp::add_rules(&mut theme_builder, palette);

    let theme = theme_builder.build(name);
    mottle::save_theme(&theme)?;

    Ok(())
}

fn print_themes_json(palettes: &Vec<Palette>) {
    println!();
    println!("THEMES JSON:");
    println!(r#"    "themes": ["#);
    for palette in palettes {
        let theme_name = &palette.definition.name;
        println!(
            r#"      {{
        "label": "{theme_name}",
        "uiTheme": "vs-dark",
        "path": "./themes/{theme_name}-color-theme.json"
      }},"#
        );
    }
    println!(r#"    ]"#);
}

fn print_reference_palette(reference_palette: &Palette) {
    println!();
    println!("REFERENCE COLORS:");
    println!("name:\t\t{:?}", reference_palette.definition.name);
    println!("functions:\t{:?}", reference_palette.functions().color);
    println!("operators:\t{:?}", reference_palette.operators().color);
}

fn print_base_colors(foundation: &PaletteFoundation) {
    println!();
    println!("BASE COLORS:");
    println!("violet:\t\t{:?}", foundation.violet().color);
    println!("maroon\t\t{:?}", foundation.maroon().color);
    println!("red:\t\t{:?}", foundation.red().color);
    println!("orange:\t\t{:?}", foundation.orange().color);
    println!("yellow:\t\t{:?}", foundation.yellow().color);
    println!("green:\t\t{:?}", foundation.green().color);
    println!("teal:\t\t{:?}", foundation.teal().color);
    println!("cyan:\t\t{:?}", foundation.cyan().color);
    println!("turquoise:\t{:?}", foundation.turquoise().color);
    println!("blue:\t\t{:?}", foundation.blue().color);
    println!("purple:\t\t{:?}", foundation.purple().color);
}
