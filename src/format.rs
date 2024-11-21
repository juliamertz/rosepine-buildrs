use crate::palette::transform::Rgb;
use clap::ValueEnum;
use palette::{transform::Color, ColorValues};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString, VariantNames};

#[derive(
    EnumString,
    EnumIter,
    VariantNames,
    Display,
    Debug,
    ValueEnum,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    Default,
)]
#[strum(serialize_all = "snake_case")]
pub enum Format {
    #[default]
    /// #ebbcba | #ebbcbaff
    Hex,
    /// ebbcbaff
    HexNs,
    /// #ebbcba | #ffebbcba
    Ahex,
    /// ffebbcba
    AhexNs,
    /// 235, 188, 186
    Rgb,
    /// 235 188 186
    RgbNs,
    /// rgb(235, 188, 186)
    RgbFunction,
    /// 235;188;186
    RgbArray,
    /// 2, 55%, 83%
    RgbAnsi,
    /// [235, 188, 186]
    Hsl,
    /// 2 55% 83%
    HslNs,
    /// hsl(2, 55%, 83%)
    HslFunction,
    /// [2, 55%, 83%]
    HslArray,
}

impl Format {
    pub fn is_hsl(&self) -> bool {
        matches!(
            self,
            Self::Hsl | Self::HslNs | Self::HslArray | Self::HslFunction
        )
    }

    pub fn is_rgb(&self) -> bool {
        matches!(
            self,
            Self::Rgb | Self::RgbNs | Self::RgbArray | Self::RgbFunction | Self::RgbAnsi
        )
    }

    pub fn is_hex(&self) -> bool {
        matches!(self, Self::Hex | Self::HexNs | Self::Ahex | Self::AhexNs)
    }

    pub fn format_color(&self, color: Rgb, alpha: Option<impl Into<f32> + Copy>) -> String {
        let mut chunks = match self.is_hsl() {
            true => color.to_hsl().color_values(),
            false => color.color_values(),
        };

        if let Some(alpha) = alpha.map(|a| a.into() / 100.0) {
            match *self {
                Self::Ahex | Self::AhexNs => chunks.insert(0, alpha * 255.0),
                Self::Hex | Self::HexNs => chunks.push(alpha * 255.0),
                _ => chunks.push(alpha),
            }
        }

        let chunks = self.format_chunks(&chunks);
        match self {
            Self::Hex | Self::Ahex => format!("#{chunks}"),
            Self::Rgb
            | Self::Hsl
            | Self::RgbNs
            | Self::HslNs
            | Self::HexNs
            | Self::AhexNs
            | Self::RgbAnsi => chunks,
            Self::RgbArray | Self::HslArray => format!("[{chunks}]"),
            Self::RgbFunction | Self::HslFunction => {
                let fn_name = match self {
                    Self::RgbFunction => "rgb",
                    Self::HslFunction => "hsl",
                    _ => unreachable!(),
                };
                let fn_name = match alpha.is_some() {
                    true => &format!("{fn_name}a"),
                    false => fn_name,
                };
                format!("{fn_name}({chunks})")
            }
        }
    }

    fn format_chunks(&self, chunks: &[f32]) -> String {
        let chunks = chunks
            .iter()
            .enumerate()
            .map(|(i, x)| self.format_chunk(*x, i))
            .collect::<Vec<_>>();
        match self {
            Self::Hex | Self::HexNs | Self::Ahex | Self::AhexNs => chunks.join("").to_lowercase(),
            Self::Rgb
            | Self::RgbArray
            | Self::RgbFunction
            | Self::Hsl
            | Self::HslArray
            | Self::HslFunction => chunks.join(", "),
            Self::RgbNs | Self::HslNs => chunks.join(" "),
            Self::RgbAnsi => chunks.join(";"),
        }
    }

    fn format_chunk(&self, chunk: f32, i: usize) -> String {
        if self.is_hsl() && (i > 0) {
            return format!("{chunk}%");
        }

        match self {
            Self::Hex | Self::HexNs | Self::Ahex | Self::AhexNs => {
                format!("{:02X}", chunk.round() as u8)
            }
            _ => chunk.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::{Role, Variant};

    fn assert_format(format: Format, alpha: Option<f32>, val: &str) {
        let color = Role::Love.get_color(&Variant::Moon);
        assert_eq!(val, format.format_color(color, alpha));
    }

    #[test]
    fn format_rgb() {
        assert_format(Format::Rgb, None, "235, 111, 146");
        assert_format(Format::RgbFunction, None, "rgb(235, 111, 146)");
        assert_format(Format::RgbFunction, Some(80.0), "rgba(235, 111, 146, 0.8)");
        assert_format(Format::RgbArray, None, "[235, 111, 146]");
    }

    #[test]
    fn format_hsl() {
        assert_format(Format::HslFunction, None, "hsl(343, 76%, 68%)");
        assert_format(Format::HslFunction, Some(70.0), "hsla(343, 76%, 68%, 0.7%)");
    }

    #[test]
    fn format_hex() {
        assert_format(Format::Hex, None, "#eb6f92");
        assert_format(Format::Hex, Some(100.0), "#eb6f92ff");
        assert_format(Format::Ahex, None, "#eb6f92");
        assert_format(Format::Ahex, Some(100.0), "#ffeb6f92");
        assert_format(Format::HexNs, None, "eb6f92");
        assert_format(Format::HexNs, Some(100.0), "eb6f92ff");
        assert_format(Format::AhexNs, Some(100.0), "ffeb6f92");
    }
}