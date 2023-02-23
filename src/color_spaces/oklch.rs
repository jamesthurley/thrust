use super::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Oklch {
    pub l: f32,
    pub c: f32,
    h: f32,
}

impl From<U8Srgb> for Oklch {
    fn from(u8srgb: U8Srgb) -> Self {
        let srgb = Srgb::from(u8srgb);
        let linear_srgb = LinearSrgb::from(srgb);
        let oklab = Oklab::from(linear_srgb);
        Oklch::from(oklab)
    }
}

impl From<Oklch> for U8Srgb {
    fn from(oklch: Oklch) -> Self {
        let oklab = Oklab::from(oklch);
        let linear_srgb = LinearSrgb::from(oklab);
        let srgb = Srgb::from(linear_srgb);
        U8Srgb::from(srgb)
    }
}

impl From<Oklab> for Oklch {
    fn from(c: Oklab) -> Self {
        Oklch { l: c.l, c: c.a.hypot(c.b), h: c.b.atan2(c.a) }
    }
}

impl From<Oklch> for Oklab {
    fn from(c: Oklch) -> Self {
        Oklab { l: c.l, a: c.c * c.h_radians().cos(), b: c.c * c.h_radians().sin() }
    }
}

impl From<(u8, u8, u8)> for Oklch {
    fn from(value: (u8, u8, u8)) -> Self {
        Oklch::from(U8Srgb::from(value))
    }
}

impl From<Oklch> for (u8, u8, u8) {
    fn from(value: Oklch) -> Self {
        U8Srgb::from(value).into()
    }
}

impl From<u32> for Oklch {
    fn from(hex: u32) -> Self {
        U8Srgb::from(hex).into()
    }
}

impl From<Oklch> for u32 {
    fn from(value: Oklch) -> Self {
        U8Srgb::from(value).into()
    }
}

impl Oklch {
    pub fn from_degrees(l: f32, c: f32, h: f32) -> Self {
        Self { l, c, h: h.to_radians() }
    }

    pub fn from_radians(l: f32, c: f32, h: f32) -> Self {
        Self { l, c, h }
    }

    pub fn h_radians(self) -> f32 {
        self.h
    }

    pub fn h_degrees(self) -> f32 {
        self.h.to_degrees()
    }

    pub fn set_h_radians(&mut self, h: f32) {
        self.h = h;
    }

    pub fn set_h_degrees(&mut self, h: f32) {
        self.h = h.to_radians();
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::COLOR_EPSILON;

    use super::*;

    #[test]
    fn it_should_convert_from_oklch_and_back() {
        let input = Oklab { l: 0.940, a: 0.102, b: 0.246 };
        let oklch = Oklch::from(input);
        let output = Oklab::from(oklch);

        assert_approx_eq!(f32, input.l, output.l, epsilon = COLOR_EPSILON);
        assert_approx_eq!(f32, input.a, output.a, epsilon = COLOR_EPSILON);
        assert_approx_eq!(f32, input.b, output.b, epsilon = COLOR_EPSILON);

        assert_approx_eq!(f32, oklch.l, 0.940, epsilon = COLOR_EPSILON);
        assert_approx_eq!(f32, oklch.c, 0.267, epsilon = COLOR_EPSILON);
        assert_approx_eq!(f32, oklch.h, 1.178, epsilon = COLOR_EPSILON);
    }

    #[test]
    fn it_should_convert_from_hex_and_back() {
        let input = 0x267ada;
        let oklch = Oklch::from(input);
        let output: u32 = oklch.into();

        assert_eq!(input, output);

        assert_approx_eq!(f32, oklch.l, 0.581, epsilon = COLOR_EPSILON);
        assert_approx_eq!(f32, oklch.c, 0.167, epsilon = COLOR_EPSILON);
        assert_approx_eq!(f32, oklch.h, -1.834, epsilon = COLOR_EPSILON);
    }

    #[test]
    fn it_should_convert_from_tuple_and_back() {
        let input = (0x26, 0x7a, 0xda);
        let oklch = Oklch::from(input);
        let output: (u8, u8, u8) = oklch.into();

        assert_eq!(input, output);

        assert_approx_eq!(f32, oklch.l, 0.581, epsilon = COLOR_EPSILON);
        assert_approx_eq!(f32, oklch.c, 0.167, epsilon = COLOR_EPSILON);
        assert_approx_eq!(f32, oklch.h, -1.834, epsilon = COLOR_EPSILON);
    }

    #[test]
    fn it_should_update_hue() {
        let mut oklch = Oklch { l: 0.1, c: 0.2, h: 0.3 };

        oklch.set_h_degrees(93.);

        assert_approx_eq!(f32, oklch.h, 1.623, epsilon = COLOR_EPSILON);

        oklch.set_h_radians(-0.123);

        assert_approx_eq!(f32, oklch.h, -0.123, epsilon = COLOR_EPSILON);
    }
}
