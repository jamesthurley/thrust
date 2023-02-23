#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
// 0 - 255 range.
pub struct U8Srgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<(u8, u8, u8)> for U8Srgb {
    fn from(value: (u8, u8, u8)) -> Self {
        U8Srgb { r: value.0, g: value.1, b: value.2 }
    }
}

impl From<U8Srgb> for (u8, u8, u8) {
    fn from(value: U8Srgb) -> Self {
        (value.r, value.g, value.b)
    }
}

impl From<u32> for U8Srgb {
    fn from(hex: u32) -> Self {
        let r = ((hex >> 16) & 0x000000FF) as u8;
        let g = ((hex >> 8) & 0x000000FF) as u8;
        let b = (hex & 0x000000FF) as u8;
        Self { r, g, b }
    }
}

impl From<U8Srgb> for u32 {
    fn from(value: U8Srgb) -> Self {
        (value.r as u32) << 16 | (value.g as u32) << 8 | value.b as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_convert_hex_to_u8srgb_and_back() {
        let input: u32 = 0x881177;
        let u8srgb = U8Srgb::from(input);
        let output = u32::from(u8srgb);

        assert_eq!(u8srgb, U8Srgb { r: 136, g: 17, b: 119 });
        assert_eq!(input, output, "input={input:#x}, output={output:#x}");
    }

    #[test]
    fn it_should_convert_tuple_to_u8srgb_and_back() {
        let input = (0x88, 0x11, 0x77);
        let u8srgb = U8Srgb::from(input);
        let output: (u8, u8, u8) = u8srgb.into();

        assert_eq!(u8srgb, U8Srgb { r: 136, g: 17, b: 119 });
        assert_eq!(input, output, "input={input:?}, output={output:?}");
    }
}
