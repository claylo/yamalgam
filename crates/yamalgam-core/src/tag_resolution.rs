//! Pluggable tag resolution for plain scalars.
//!
//! The YAML spec (Section 10) defines "schemas" that determine how untagged
//! plain scalars are implicitly typed. This module calls the operation "tag
//! resolution" to avoid confusion with schema validation (JSON Schema, etc.).
//!
//! Four built-in implementations cover the spec-defined schemas plus YAML 1.1
//! legacy behavior. Users can implement [`TagResolver`] for custom typing rules.

use crate::Value;

/// Resolves untagged plain scalars to typed [`Value`]s.
///
/// Only plain (unquoted) scalars undergo tag resolution. Quoted scalars are
/// always strings — the Composer handles that distinction before calling
/// this trait.
///
/// Named after the YAML spec's "tag resolution" operation (§10) to avoid
/// collision with schema validation (JSON Schema, etc.).
pub trait TagResolver {
    /// Resolve a plain scalar string to a typed Value.
    fn resolve_scalar(&self, value: &str) -> Value;
}

/// Tag resolution scheme selector.
///
/// Used in [`LoaderConfig`](crate::LoaderConfig) to select a built-in tag
/// resolution scheme. Implements [`TagResolver`] by dispatching to the
/// corresponding implementation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum TagResolution {
    /// YAML 1.2 Failsafe Schema — all scalars are strings.
    Failsafe,
    /// YAML 1.2 JSON Schema — strict null/bool/int/float.
    Json,
    /// YAML 1.2 Core Schema (recommended default).
    #[default]
    Yaml12,
    /// YAML 1.1 type resolution — extended booleans, legacy octal, binary.
    Yaml11,
}

/// YAML 1.2 Failsafe Schema — all plain scalars are strings.
///
/// The simplest schema: no implicit typing at all. Every plain scalar
/// becomes `Value::String`. Use this for lossless round-trip processing
/// where you don't want type coercion.
// y[impl schema.failsafe.tag-str+3]
#[derive(Debug, Clone, Copy, Default)]
pub struct FailsafeTagResolver;

impl TagResolver for FailsafeTagResolver {
    fn resolve_scalar(&self, value: &str) -> Value {
        Value::String(value.to_owned())
    }
}

// Re-export Yaml12TagResolver from tag.rs so it's accessible via this module.
pub use crate::tag::Yaml12TagResolver;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Value;

    #[test]
    fn failsafe_always_returns_string() {
        let r = FailsafeTagResolver;
        assert_eq!(r.resolve_scalar("null"), Value::String("null".into()));
        assert_eq!(r.resolve_scalar("true"), Value::String("true".into()));
        assert_eq!(r.resolve_scalar("42"), Value::String("42".into()));
        assert_eq!(r.resolve_scalar("3.14"), Value::String("3.14".into()));
        assert_eq!(r.resolve_scalar("hello"), Value::String("hello".into()));
        assert_eq!(r.resolve_scalar(""), Value::String(String::new()));
        assert_eq!(r.resolve_scalar("~"), Value::String("~".into()));
    }

    #[test]
    fn yaml12_matches_resolve_plain_scalar() {
        use crate::resolve_plain_scalar;
        let r = Yaml12TagResolver;
        let cases = [
            "null", "Null", "NULL", "~", "",
            "true", "True", "TRUE", "false", "False", "FALSE",
            "42", "-17", "+99", "0o17", "0xFF",
            "1.0", "-0.5", "1e10", ".inf", "-.inf", ".nan",
            "hello", "yes", "no", "on", "off",
        ];
        for s in cases {
            let resolver_result = r.resolve_scalar(s);
            let direct_result = resolve_plain_scalar(s);
            // NaN != NaN, so handle that case explicitly
            match (&resolver_result, &direct_result) {
                (Value::Float(a), Value::Float(b)) if a.is_nan() && b.is_nan() => {}
                _ => assert_eq!(
                    resolver_result, direct_result,
                    "Yaml12TagResolver disagrees with resolve_plain_scalar for {s:?}"
                ),
            }
        }
    }
}
