pub mod spec;
pub mod drift;
pub mod error;

pub use spec::FunPaySpec;
pub use drift::{DriftReport, check_drift, validate_selector, check_selector_on_html, validate_all_selectors, parse_entities_from_html};
pub use error::SpecError;
