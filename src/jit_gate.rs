use std::env;

/// Compile-time availability: true only if built for x86_64 with `--features pro_jit`.
#[inline]
pub fn jit_available() -> bool {
    cfg!(all(target_arch = "x86_64", feature = "pro_jit"))
}

/// Simple, local-only license validation to gate runtime enablement.
/// Accepts:
/// - "EROCK-PRO-DEV" / "EROCK-PRO-TEST" (for local/dev)
/// - Any key starting with "EROCK-PRO-" and 12–64 chars of [A-Z0-9-]
pub fn license_valid() -> bool {
    match env::var("EROCK_LICENSE_KEY") {
        Ok(k) => is_valid_key(&k),
        Err(_) => false,
    }
}

fn is_valid_key(k: &str) -> bool {
    if k == "EROCK-PRO-DEV" || k == "EROCK-PRO-TEST" {
        return true;
    }
    const PREFIX: &str = "EROCK-PRO-";
    if !k.starts_with(PREFIX) {
        return false;
    }
    let body = &k[PREFIX.len()..];
    let len = body.len();
    (12..=64).contains(&len)
        && body
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-')
}

/// Runtime enablement requires:
/// - compiled availability (x86_64 + feature=pro_jit)
/// - not explicitly disabled via EROCK_DISABLE_JIT=1
/// - a valid license key via EROCK_LICENSE_KEY
pub fn jit_enabled() -> bool {
    if !jit_available() {
        return false;
    }
    if env::var("EROCK_DISABLE_JIT")
        .map(|v| v == "1")
        .unwrap_or(false)
    {
        return false;
    }
    license_valid()
}

/// Human-readable reason for current state.
pub fn jit_reason() -> String {
    if !jit_available() {
        return "not-compiled".into();
    }
    if env::var("EROCK_DISABLE_JIT")
        .map(|v| v == "1")
        .unwrap_or(false)
    {
        return "disabled-by-env".into();
    }
    if !license_valid() {
        return "no-or-invalid-license".into();
    }
    "enabled".into()
}

/// Convenience for wiring into /health
pub fn health_fields() -> (bool, bool, String) {
    (jit_available(), jit_enabled(), jit_reason())
}
