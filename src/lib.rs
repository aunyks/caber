use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CaberError {
    #[error("invalid export mode provided. Expected default, object, or none")]
    InvalidExportMode,
    #[error(
        "unrecognized output language requested. Expected JavaScript / ECMAScript or TypeScript"
    )]
    UnrecognizedLanguage,
}

#[cfg_attr(feature = "cli", derive(clap::Parser))]
#[derive(Debug, Clone, Copy)]
pub enum ExportMode {
    Default,
    Object,
    None,
}

impl Default for ExportMode {
    fn default() -> Self {
        Self::Default
    }
}

// #[cfg(feature = "cli")]
impl FromStr for ExportMode {
    type Err = CaberError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(Self::Default),
            "object" => Ok(Self::Object),
            "none" => Ok(Self::None),
            _ => Err(Self::Err::InvalidExportMode),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct CaberConfig {
    pub export_mode: ExportMode,
}

#[derive(Default, Clone, Copy)]
pub struct Binary<'a> {
    bytes: &'a [u8],
    config: CaberConfig,
}

impl<'a> Binary<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            config: CaberConfig::default(),
        }
    }

    pub fn with_config(mut self, config: CaberConfig) -> Self {
        self.config = config;
        self
    }

    pub fn to_javascript(&self) -> String {
        self.to_ecmascript()
    }

    pub fn to_typescript(&self) -> String {
        self.to_ecmascript()
    }

    pub fn to_ecmascript(&self) -> String {
        let mut array_innards = String::new();
        for (byte_index, byte) in self.bytes.iter().enumerate() {
            let mut element = format!("{}", byte.to_string());
            if byte_index != self.bytes.len() - 1 {
                element.push_str(", ")
            }
            array_innards.push_str(&element);
        }
        self.export_text(array_innards)
    }

    fn export_text(&self, array_innards: String) -> String {
        match self.config.export_mode {
            ExportMode::Default => format!(
                "const binary = new Uint8Array([{}]);\r\nexport default binary;",
                array_innards
            ),
            ExportMode::Object => format!(
                "const binary = new Uint8Array([{}]);\r\nexport {{ binary }};",
                array_innards
            ),
            ExportMode::None => format!("const binary = new Uint8Array([{}]);", array_innards),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_ecmascript() {
        let bin = Binary::new(&[0, 1, 2, 3]);
        assert_eq!(
            bin.to_ecmascript(),
            String::from("const binary = new Uint8Array([0, 1, 2, 3]);\r\nexport default binary;")
        )
    }
}
