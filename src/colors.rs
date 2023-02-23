use crate::{
    color_spaces::Oklch, palette::Palette, palette_foundation::PaletteFoundation,
    palette_oklch::PaletteOklch,
};
use paste::paste;

macro_rules! def_color_method {
    ($name:ident, hex: $hex:literal) => {
        impl PaletteFoundation {
            #[allow(dead_code)]
            pub fn $name(&self) -> PaletteOklch {
                self.create_palette_color(Oklch::from($hex))
            }

            paste! {
                #[allow(dead_code)]
                pub fn [<$name _wash>](&self) -> PaletteOklch {
                    self.create_palette_color(Oklch::from($hex)).with_lightness(0.52).with_chroma(0.14)
                }

                #[allow(dead_code)]
                pub fn [<$name _intense>](&self) -> PaletteOklch {
                    self.create_palette_color(Oklch::from($hex)).with_lightness(0.72).with_chroma(0.33)
                }
            }
        }

        impl Palette {
            #[allow(dead_code)]
            pub fn $name(&self) -> PaletteOklch {
                self.definition.foundation.$name()
            }

            paste! {
                #[allow(dead_code)]
                pub fn [<$name _wash>](&self) -> PaletteOklch {
                    self.definition.foundation.[<$name _wash>]()
                }

                #[allow(dead_code)]
                pub fn [<$name _intense>](&self) -> PaletteOklch {
                    self.definition.foundation.[<$name _intense>]()
                }
            }
        }
    };
}

// These base colors are from the 12-bit rainbow palette, except when they're not:
// https://iamkate.com/data/12-bit-rainbow/
def_color_method!(violet, hex: 0x881177);
def_color_method!(maroon, hex: 0xaa3355);
def_color_method!(red, hex: 0xcc6666);
def_color_method!(orange, hex: 0xee9944);
def_color_method!(yellow, hex: 0xeedd00);
def_color_method!(green, hex: 0x85bf86);
def_color_method!(teal, hex: 0x22ccbb);
def_color_method!(cyan, hex: 0x00bbcc);
def_color_method!(turquoise, hex: 0x0099cc);
def_color_method!(blue, hex: 0x3366bb);
def_color_method!(purple, hex: 0x663399);

// Useful links:
// https://bottosson.github.io/posts/oklab/
// https://observablehq.com/search?query=oklab
