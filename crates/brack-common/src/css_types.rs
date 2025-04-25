use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AbsoluteSize {
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XxLarge,
    XxxLarge,
}

impl fmt::Display for AbsoluteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbsoluteSize::XxSmall => write!(f, "xx-small"),
            AbsoluteSize::XSmall => write!(f, "x-small"),
            AbsoluteSize::Small => write!(f, "small"),
            AbsoluteSize::Medium => write!(f, "medium"),
            AbsoluteSize::Large => write!(f, "large"),
            AbsoluteSize::XLarge => write!(f, "x-large"),
            AbsoluteSize::XxLarge => write!(f, "xx-large"),
            AbsoluteSize::XxxLarge => write!(f, "xxx-large"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AlphaValue {
    Number(Number),
    Percentage(Percentage),
}

impl fmt::Display for AlphaValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlphaValue::Number(num) => write!(f, "{}", num),
            AlphaValue::Percentage(perc) => write!(f, "{}", perc),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AnglePercentage {
    Angle(Angle),
    Percentage(Percentage),
}

impl fmt::Display for AnglePercentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnglePercentage::Angle(angle) => write!(f, "{}", angle),
            AnglePercentage::Percentage(percentage) => write!(f, "{}", percentage),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Angle {
    Deg(f32),
    Grad(f32),
    Rad(f32),
    Turn(f32),
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Angle::Deg(value) => write!(f, "{}deg", value),
            Angle::Grad(value) => write!(f, "{}grad", value),
            Angle::Rad(value) => write!(f, "{}rad", value),
            Angle::Turn(value) => write!(f, "{}turn", value),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BaselinePosition {
    Baseline,
    FirstBaseline,
    LastBaseline,
}

impl fmt::Display for BaselinePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaselinePosition::Baseline => write!(f, "baseline"),
            BaselinePosition::FirstBaseline => write!(f, "first baseline"),
            BaselinePosition::LastBaseline => write!(f, "last baseline"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BasicShape {
    todo!()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl fmt::Display for BlendMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlendMode::Normal => write!(f, "normal"),
            BlendMode::Multiply => write!(f, "multiply"),
            BlendMode::Screen => write!(f, "screen"),
            BlendMode::Overlay => write!(f, "overlay"),
            BlendMode::Darken => write!(f, "darken"),
            BlendMode::Lighten => write!(f, "lighten"),
            BlendMode::ColorDodge => write!(f, "color-dodge"),
            BlendMode::ColorBurn => write!(f, "color-burn"),
            BlendMode::HardLight => write!(f, "hard-light"),
            BlendMode::SoftLight => write!(f, "soft-light"),
            BlendMode::Difference => write!(f, "difference"),
            BlendMode::Exclusion => write!(f, "exclusion"),
            BlendMode::Hue => write!(f, "hue"),
            BlendMode::Saturation => write!(f, "saturation"),
            BlendMode::Color => write!(f, "color"),
            BlendMode::Luminosity => write!(f, "luminosity"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BoxEdge {

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CalcKeyword {
    E,
    Pi,
    Infinity,
    MinusInfinity,
    NaN,
}

impl fmt::Display for CalcKeyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcKeyword::E => write!(f, "e"),
            CalcKeyword::Pi => write!(f, "pi"),
            CalcKeyword::Infinity => write!(f, "infinity"),
            CalcKeyword::MinusInfinity => write!(f, "-infinity"),
            CalcKeyword::NaN => write!(f, "NaN"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalcSum {
    value: CalcProduct,
    expr: Vec<(CalcSumToken, CalcProduct)>,
}

impl fmt::Display for CalcSum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)?;
        for (token, product) in &self.expr {
            write!(f, "{} {}", token, product)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CalcSumToken {
    Plus,
    Minus,
}

impl fmt::Display for CalcSumToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcSumToken::Plus => write!(f, "+"),
            CalcSumToken::Minus => write!(f, "-"),
        }
    }   
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalcProduct {
    value: CalcValue,
    expr: Vec<(CalcProductToken, CalcValue)>,
}

impl fmt::Display for CalcProduct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)?;
        for (token, value) in &self.expr {
            write!(f, "{} {}", token, value)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CalcProductToken {
    Multiply,
    Divide,
}

impl fmt::Display for CalcProductToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcProductToken::Multiply => write!(f, "*"),
            CalcProductToken::Divide => write!(f, "/"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CalcValue {
    Number(Number),
    Dimension(Dimension),
    Percentage(Percentage),
    CalcKeyword(CalcKeyword),
}

impl fmt::Display for CalcValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcValue::Number(num) => write!(f, "{}", num),
            CalcValue::Dimension(dim) => write!(f, "{}", dim),
            CalcValue::Percentage(perc) => write!(f, "{}", perc),
            CalcValue::CalcKeyword(keyword) => write!(f, "{}", keyword),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ColorInterpolationMethod {
    RectangularColorSpace(RectangularColorSpace),
    PolarColorSpace((PolarColorSpace, Option<HueInterpolationMethod>)),
}

impl fmt::Display for ColorInterpolationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorInterpolationMethod::RectangularColorSpace(space) => write!(f, "in {}", space),
            ColorInterpolationMethod::PolarColorSpace((space, method)) => {
                if let Some(method) = method {
                    write!(f, "in {} {}", space, method)
                } else {
                    write!(f, "in {}", space)
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RectangularColorSpace {
    Srgb,
    SrgbLinear,
    DisplayP3,
    A98Rgb,
    ProphotoRgb,
    Rec2020,
    Lab,
    Oklab,
    Xyz,
    XyzD50,
    XyzD65,
}

impl fmt::Display for RectangularColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RectangularColorSpace::Srgb => write!(f, "sRGB"),
            RectangularColorSpace::SrgbLinear => write!(f, "sRGB-linear"),
            RectangularColorSpace::DisplayP3 => write!(f, "display-p3"),
            RectangularColorSpace::A98Rgb => write!(f, "a98-rgb"),
            RectangularColorSpace::ProphotoRgb => write!(f, "prophoto-rgb"),
            RectangularColorSpace::Rec2020 => write!(f, "rec2020"),
            RectangularColorSpace::Lab => write!(f, "lab"),
            RectangularColorSpace::Oklab => write!(f, "oklab"),
            RectangularColorSpace::Xyz => write!(f, "xyz"),
            RectangularColorSpace::XyzD50 => write!(f, "xyz-d50"),
            RectangularColorSpace::XyzD65 => write!(f, "xyz-d65"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PolarColorSpace {
    Hsl,
    Hwb,
    Lch,
    Oklch,
}

impl fmt::Display for PolarColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolarColorSpace::Hsl => write!(f, "hsl"),
            PolarColorSpace::Hwb => write!(f, "hwb"),
            PolarColorSpace::Lch => write!(f, "lch"),
            PolarColorSpace::Oklch => write!(f, "oklch"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HueInterpolationMethod {
    Shorter,
    Longer,
    Increasing,
    Decreasing,
}

impl fmt::Display for HueInterpolationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HueInterpolationMethod::Shorter => write!(f, "shorter hue"),
            HueInterpolationMethod::Longer => write!(f, "longer hue"),
            HueInterpolationMethod::Increasing => write!(f, "increasing hue"),
            HueInterpolationMethod::Decreasing => write!(f, "decreasing hue"),
        }
    }
}

pub enum Color {

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModernHslaSyntax {
    hue: Option<Hue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Hue {
    Number(Number),
    Angle(Angle),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ColorspaceParams {
    PredefinedRgb(PredefinedRgbParams),
    Xyz(XyzParams),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PredefinedRgbParams {
    Params0(PredefinedRgb),
    Params1(PredefinedRgb, PredefinedRgbParamsProperty),
    Params2(PredefinedRgb, PredefinedRgbParamsProperty, PredefinedRgbParamsProperty),
    Params3(PredefinedRgb, PredefinedRgbParamsProperty, PredefinedRgbParamsProperty, PredefinedRgbParamsProperty),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PredefinedRgbParamsProperty {
    Number(Number),
    Percentage(Percentage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum XyzParams {
    Params0(XyzSpace),
    Params1(XyzSpace, XyzParamsProperty),
    Params2(XyzSpace, XyzParamsProperty, XyzParamsProperty),
    Params3(XyzSpace, XyzParamsProperty, XyzParamsProperty, XyzParamsProperty),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum XyzParamsProperty {
    Number(Number),
    Percentage(Percentage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PredefinedRgb {
    Srgb,
    SrgbLinear,
    DisplayP3,
    A98Rgb,
    ProphotoRgb,
    Rec2020,
    Rec2100Pq,
    Rec2100Hlg,
    Rec2100Linear,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum XyzSpace {
    Xyz,
    XyzD50,
    XyzD65,
}