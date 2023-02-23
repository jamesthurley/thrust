use super::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Oklab {
    pub l: f32,
    pub a: f32,
    pub b: f32,
}

impl From<LinearSrgb> for Oklab {
    #![allow(clippy::excessive_precision)]

    fn from(c: LinearSrgb) -> Self {
        let l = 0.4122214708 * c.r + 0.5363325363 * c.g + 0.0514459929 * c.b;
        let m = 0.2119034982 * c.r + 0.6806995451 * c.g + 0.1073969566 * c.b;
        let s = 0.0883024619 * c.r + 0.2817188376 * c.g + 0.6299787005 * c.b;

        let l_ = l.cbrt();
        let m_ = m.cbrt();
        let s_ = s.cbrt();

        Oklab {
            l: 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
            a: 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
            b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
        }
    }
}

impl From<Oklab> for LinearSrgb {
    #![allow(clippy::excessive_precision)]

    fn from(c: Oklab) -> Self {
        let l_ = c.l + 0.3963377774 * c.a + 0.2158037573 * c.b;
        let m_ = c.l - 0.1055613458 * c.a - 0.0638541728 * c.b;
        let s_ = c.l - 0.0894841775 * c.a - 1.2914855480 * c.b;

        let l = l_ * l_ * l_;
        let m = m_ * m_ * m_;
        let s = s_ * s_ * s_;

        LinearSrgb {
            r: 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            g: -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            b: -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::assert_approx_eq;

    use crate::COLOR_EPSILON;

    use super::*;

    #[test]
    fn it_should_convert_from_linear_srgb_and_back() {
        let input = LinearSrgb { r: 1.0, g: 0.5, b: 0.01 };
        let oklab = Oklab::from(input);
        let output = LinearSrgb::from(oklab);

        assert_approx_eq!(f32, input.r, output.r, epsilon = COLOR_EPSILON);
        assert_approx_eq!(f32, input.g, output.g, epsilon = COLOR_EPSILON);
        assert_approx_eq!(f32, input.b, output.b, epsilon = COLOR_EPSILON);
    }
}
