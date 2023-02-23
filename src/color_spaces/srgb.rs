use super::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Srgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<U8Srgb> for Srgb {
    fn from(c: U8Srgb) -> Self {
        let scale = |n: u8| n as f32 / 255.0;
        Srgb { r: scale(c.r), g: scale(c.g), b: scale(c.b) }
    }
}

impl From<Srgb> for U8Srgb {
    fn from(c: Srgb) -> Self {
        let scale = |n: f32| (n * 255.0).round() as u8;
        U8Srgb { r: scale(c.r), g: scale(c.g), b: scale(c.b) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sould_convert_from_srgb_to_u8srgb() {
        assert_eq!(U8Srgb::from(Srgb { r: 1., g: 0., b: 0. }), U8Srgb { r: 255, g: 0, b: 0 });
        assert_eq!(U8Srgb::from(Srgb { r: 0., g: 0.5, b: 0. }), U8Srgb { r: 0, g: 128, b: 0 });
        assert_eq!(U8Srgb::from(Srgb { r: 0., g: 0., b: 0.25 }), U8Srgb { r: 0, g: 0, b: 64 });
    }

    #[test]
    fn it_sould_convert_form_u8srgb_to_srgb() {
        assert_eq!(Srgb::from(U8Srgb { r: 255, g: 0, b: 0 }), Srgb { r: 1., g: 0., b: 0. });
        assert_eq!(Srgb::from(U8Srgb { r: 0, g: 128, b: 0 }), Srgb { r: 0., g: 0.5019608, b: 0. });
        assert_eq!(Srgb::from(U8Srgb { r: 0, g: 0, b: 64 }), Srgb { r: 0., g: 0., b: 0.2509804 });
    }
}
