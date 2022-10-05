use crate::error::{Result, ZipError};

#[non_exhaustive]
pub enum AttributeCompatibility {
    Unix
}

impl TryFrom<u16> for AttributeCompatibility {
    type Error = ZipError;

    fn try_from(value: u16) -> Result<Self> {
        match value {
            3 => Ok(AttributeCompatibility::Unix),
            _ => Err(ZipError::UnsupportedAttributeCompatibility(value))
        }
    }
}

impl From<&AttributeCompatibility> for u16 {
    fn from(compatibility: &AttributeCompatibility) -> Self {
        match compatibility {
            AttributeCompatibility::Unix => 3
        }
    }
}

impl From<AttributeCompatibility> for u16 {
    fn from(compatibility: AttributeCompatibility) -> Self {
        (&compatibility).into()
    }
}