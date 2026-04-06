use std::collections::HashMap;

use super::dataset::NominalDataset;
use super::metrics::{find_most_frequent_values, find_most_pertinent_attributes};
use super::types::CncResult;

/// Compute closure for nominal data (group objects by attribute value and find common attributes)
pub fn compute_nominal_closure(
    dataset: &NominalDataset,
    attr_name: &str,
    attr_value: &str,
) -> (Vec<usize>, HashMap<String, String>) {
    // Step 1: Find all objects with this attribute value (extent)
    let extent: Vec<usize> = dataset
        .data
        .iter()
        .enumerate()
        .filter(|(_, obj_data)| obj_data.get(attr_name) == Some(&attr_value.to_string()))
        .map(|(idx, _)| idx)
        .collect();

    // Step 2: Find common attributes for these objects (intent)
    if extent.is_empty() {
        return (extent, HashMap::new());
    }

    let mut intent = HashMap::new();
    let first_obj = &dataset.data[extent[0]];

    // Check which attributes have the same value across all objects in extent
    // Only include descriptive attributes (exclude class) in the intent
    for attr in &dataset.attributes {
        if attr == &dataset.class_attribute {
            continue; // Skip class attribute - it's not part of the formal context
        }

        let first_value = first_obj.get(attr);
        if first_value.is_none() {
            continue;
        }

        let all_same = extent
            .iter()
            .all(|&obj_idx| dataset.data[obj_idx].get(attr) == first_value);

        if all_same {
            intent.insert(attr.clone(), first_value.unwrap().clone());
        }
    }

    (extent, intent)
}

/// CNC algorithm.
/// Returns all concepts when there are ties in pertinence or frequency
pub fn cnc(dataset: &NominalDataset) -> CncResult {
    // Step 1: Find all most pertinent attributes (handle ties)
    let pertinent_attrs = find_most_pertinent_attributes(dataset);

    if pertinent_attrs.is_empty() {
        return CncResult {
            concepts: Vec::new(),
            pertinent_attrs: Vec::new(),
        };
    }

    // Steps 2 and 3 there.
    cnc_core(pertinent_attrs, dataset)
}

pub(crate) fn cnc_core(pertinent_attrs: Vec<String>, dataset: &NominalDataset) -> CncResult {
    let mut results = Vec::new();

    // Step 2 of the CNC algorithm: For each pertinent attribute, find all most frequent values (handle ties)
    for pertinent_attr in &pertinent_attrs {
        let most_frequent_values = find_most_frequent_values(dataset, pertinent_attr);

        // Step 3: Compute closure for each attribute-value pair
        for value in &most_frequent_values {
            let (extent, intent) = compute_nominal_closure(dataset, pertinent_attr, value);
            results.push((pertinent_attr.clone(), value.clone(), extent, intent));
        }
    }

    CncResult {
        concepts: results,
        pertinent_attrs,
    }
}
