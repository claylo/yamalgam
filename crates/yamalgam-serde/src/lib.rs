//! Streaming serde Deserializer for YAML.
//!
//! Consumes the yamalgam parser event stream directly -- no intermediate DOM
//! materialization. Uses erased-serde internally so the parser never
//! monomorphizes.
#![deny(unsafe_code)]

mod de;
mod error;

pub use de::Deserializer;
pub use error::Error;

/// Deserialize a single YAML document.
///
/// Errors if the input contains multiple documents. For multi-document
/// streams, use [`Deserializer::from_str`] with [`Deserializer::documents`].
///
/// # Errors
///
/// Returns [`Error`] on parse failure, type mismatch, or multiple documents.
pub fn from_str<'de, T: serde::Deserialize<'de>>(input: &'de str) -> Result<T, Error> {
    let mut de = Deserializer::from_str(input);
    let value = T::deserialize(&mut de)?;
    de.check_end()?;
    Ok(value)
}

/// Deserialize a single YAML document with explicit resource limits.
///
/// Same as [`from_str`] but applies the given [`ResourceLimits`] for
/// anchor count and alias expansion caps.
///
/// # Errors
///
/// Returns [`Error`] on parse failure, type mismatch, limit exceeded, or
/// multiple documents.
pub fn from_str_with_limits<'de, T: serde::Deserialize<'de>>(
    input: &'de str,
    limits: yamalgam_core::ResourceLimits,
) -> Result<T, Error> {
    let mut de = Deserializer::from_str_with_limits(input, limits);
    let value = T::deserialize(&mut de)?;
    de.check_end()?;
    Ok(value)
}
