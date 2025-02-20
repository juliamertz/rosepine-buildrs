use clap::ValueEnum;
use palette::Color;
use serde::Serialize;
use strum_macros::{Display, EnumIter};

#[derive(EnumIter, Display, Debug, ValueEnum, Clone, Copy, PartialEq, Serialize, Default)]
#[strum(serialize_all = "snake_case")]
pub enum Format {
    #[default]
    /// #ebbcba | #ebbcbaff
    Hex,
    /// ebbcba | ebbcbaff
    HexNs,
    /// #ebbcba | #ffebbcba
    Ahex,
    /// ebbcba | ffebbcba
    AhexNs,
    /// 235, 188, 186
    Rgb,
    /// 235 188 186
    RgbNs,
    /// rgb(235, 188, 186)
    RgbFunction,
    /// [235, 188, 186]
    RgbArray,
    /// 235;188;186
    RgbAnsi,
    /// 2, 55%, 83%
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

    pub fn is_hex(&self) -> bool {
        matches!(self, Self::Hex | Self::HexNs | Self::Ahex | Self::AhexNs)
    }

    pub fn format_color(&self, color: Color, alpha: Option<impl Into<f32> + Copy>) -> String {
        let mut chunks = match self.is_hsl() {
            true => vec![color.hsl.h as f32, color.hsl.s as f32, color.hsl.l as f32],
            false => vec![color.rgb.r as f32, color.rgb.g as f32, color.rgb.b as f32],
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
            Self::RgbArray | Self::HslArray => format!("[{chunks}]"),
            Self::RgbFunction => format!("rgb({chunks})",),
            Self::HslFunction => format!("hsl({chunks})"),
            _ => chunks,
        }
    }

    /// Formats and joins all color components
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

    /// Formats a single color component
    fn format_chunk(&self, chunk: f32, i: usize) -> String {
        if self.is_hsl() && (i > 0 && i < 3) {
            format!("{chunk}%")
        } else if self.is_hex() {
            format!("{:02X}", chunk.round() as u8)
        } else {
            chunk.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::{Role, Variant};

    fn assert_format(format: Format, alpha: Option<f32>, val: &str) {
        let color = Role::Love.get_color(&Variant::Moon);
        assert_eq!(val, format.format_color(color, alpha));
    }

    #[test]
    fn format_rgb() {
        assert_format(Format::Rgb, None, "235, 111, 146");
        assert_format(Format::RgbFunction, None, "rgb(235, 111, 146)");
        assert_format(Format::RgbFunction, Some(80.0), "rgb(235, 111, 146, 0.8)");
        assert_format(Format::RgbArray, None, "[235, 111, 146]");
    }

    #[test]
    fn format_hsl() {
        assert_format(Format::HslFunction, None, "hsl(343, 76%, 68%)");
        assert_format(Format::HslFunction, Some(70.0), "hsl(343, 76%, 68%, 0.7)");
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
