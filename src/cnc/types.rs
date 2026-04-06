use std::collections::{HashMap, HashSet};

pub type CncConcept = (String, String, Vec<usize>, HashMap<String, String>);

/// Result structure for CNC containing both the concepts and debug information
#[derive(Debug)]
pub struct CncResult {
    pub concepts: Vec<CncConcept>,
    pub pertinent_attrs: Vec<String>,
}

/// Result structure for CNC-BP containing both the concepts and debug information
/// Uses CncResult to avoid duplication and maintain consistency
#[derive(Debug)]
pub struct CncBpResult {
    pub cnc_result: CncResult,
    pub minority_classes: HashSet<String>,
    pub original_size: usize,
    pub filtered_size: usize,
}
