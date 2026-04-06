mod bp;
mod core;
mod dataset;
mod display;
mod metrics;
pub mod rules;
mod types;

pub use bp::cnc_bp;
pub use core::{cnc, compute_nominal_closure};
pub use dataset::NominalDataset;
pub use display::{display_cnc_chosen_attribute, display_cnc_results};
pub use types::{CncBpResult, CncConcept, CncResult};

#[cfg(test)]
mod tests;
