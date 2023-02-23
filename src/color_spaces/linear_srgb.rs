use super::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct LinearSrgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<Srgb> for LinearSrgb {
    fn from(c: Srgb) -> Self {
        let linearize = |x: f32| {
            if x >= 0.04045 {
                ((x + 0.055) / 1.055).powf(2.4)
            } else {
                x / 12.92
            }
        };

        LinearSrgb { r: linearize(c.r), g: linearize(c.g), b: linearize(c.b) }
    }
}

impl From<LinearSrgb> for Srgb {
    fn from(c: LinearSrgb) -> Self {
        let nonlinearize = |x: f32| {
            if x >= 0.0031308 {
                x.powf(1.0 / 2.4) * 1.055 - 0.055
            } else {
                x * 12.92
            }
        };

        Srgb { r: nonlinearize(c.r), g: nonlinearize(c.g), b: nonlinearize(c.b) }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::COLOR_EPSILON;

    use super::*;

    #[test]
    fn it_should_convert_from_srgb_to_linear_srgb_and_back() {
        let input = Srgb { r: 1.0, g: 0.5, b: 0.01 };
        let linear_srgb = LinearSrgb::from(input);
        let output = Srgb::from(linear_srgb);

        assert_approx_eq!(f32, input.r, output.r, epsilon = COLOR_EPSILON);
        assert_approx_eq!(f32, input.g, output.g, epsilon = COLOR_EPSILON);
        assert_approx_eq!(f32, input.b, output.b, epsilon = COLOR_EPSILON);
    }
}
