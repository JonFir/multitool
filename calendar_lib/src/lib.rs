//! Calendar domain library.
//!
//! This crate contains calendar-related domain models and integrations.

/// Returns the crate name. Useful for smoke tests while functionality is added.
pub fn name() -> &'static str {
    "calendar_lib"
}

#[cfg(test)]
mod tests {
    use super::name;

    #[test]
    fn returns_crate_name() {
        assert_eq!(name(), "calendar_lib");
    }
}
