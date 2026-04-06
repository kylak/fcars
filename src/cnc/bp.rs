use std::collections::{HashMap, HashSet};

use super::core::cnc_core;
use super::dataset::{filter_dataset_by_classes, NominalDataset};
use super::metrics::find_most_pertinent_attributes;
use super::types::{CncBpResult, CncResult};

/// CNC-BP : CNC Bottom-Pertinent Classes. Keeps only the n most minority classes.
///
/// n: number of minority classes to keep.
///
/// When classes have identical frequencies, all classes at the same frequency level are included.
/// The function selects complete frequency tiers until reaching or exceeding the requested n classes.
/// In case all classes share the same frequency (complete tie), all classes are retained regardless of n.
///
/// Example: If we have classes A(3), B(2), C(2), D(1) and n=2:
/// - Keeps D (most minority with 1 object)
/// - Keeps both B and C (both have 2 objects, tie at second most minority)
/// - Total: 3 classes kept (D, B, C)
pub fn cnc_bp(dataset: &NominalDataset, n: usize) -> CncBpResult {
    // We first get the G.I to not interfere on CNC.
    let pertinent_attrs = find_most_pertinent_attributes(dataset);

    // Step 1: Get all class values and their distribution
    let all_class_values = dataset.get_class_values(&(0..dataset.objects.len()).collect::<Vec<_>>());
    let mut class_counts: HashMap<String, usize> = HashMap::new();
    for class_val in &all_class_values {
        *class_counts.entry(class_val.clone()).or_insert(0) += 1;
    }

    // Step 2: Sort classes by frequency (ascending) to find minority classes
    let mut sorted_classes: Vec<_> = class_counts.into_iter().collect();
    sorted_classes.sort_by_key(|(_, count)| *count);

    // Get the n most minority class names
    let minority_classes: HashSet<String> = sorted_classes
        .into_iter()
        .take(n)
        .map(|(class_name, _)| class_name)
        .collect();

    // Step 3: Create filtered dataset keeping only objects from minority classes
    let filtered_dataset = filter_dataset_by_classes(dataset, &minority_classes);

    // Apply CNC on filtered dataset
    let cnc_result = if pertinent_attrs.is_empty() {
        CncResult {
            concepts: Vec::new(),
            pertinent_attrs: Vec::new(),
        }
    } else {
        cnc_core(pertinent_attrs.clone(), &filtered_dataset)
    };

    CncBpResult {
        cnc_result,
        minority_classes,
        original_size: dataset.objects.len(),
        filtered_size: filtered_dataset.objects.len(),
    }
}
