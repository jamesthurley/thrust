use typed_builder::TypedBuilder;

use crate::{palette_foundation::PaletteFoundation, palette_oklch::PaletteOklch};

type ColorModifier = Box<dyn Fn(PaletteOklch) -> PaletteOklch>;

pub fn default() -> ColorModifier {
    Box::new(|c| c)
}

#[derive(TypedBuilder)]
pub struct PaletteDefinition {
    pub name: String,
    pub foundation: &'static PaletteFoundation,
    pub accent: PaletteOklch,

    #[builder(default=default())]
    pub foreground: ColorModifier,
    #[builder(default=default())]
    pub strings: ColorModifier,
    #[builder(default=default())]
    pub numbers: ColorModifier,
    #[builder(default=default())]
    pub functions: ColorModifier,
    #[builder(default=default())]
    pub comments: ColorModifier,
    #[builder(default=default())]
    pub types: ColorModifier,
    #[builder(default=default())]
    pub constants: ColorModifier,
}
