use std::collections::HashMap;

use super::dataset::NominalDataset;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AttributeInformationGain {
    pub(crate) attribute: String,
    pub(crate) gain: f64,
}

/// Calculate entropy for nominal values
fn calculate_entropy(values: &[String]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    let total = values.len() as f64;
    let mut value_counts = HashMap::new();

    // Count occurrences of each value
    for value in values {
        *value_counts.entry(value.clone()).or_insert(0) += 1;
    }

    // Calculate entropy: -Σ p_i * log2(p_i)
    value_counts
        .values()
        .map(|&count| {
            let probability = count as f64 / total;
            -probability * probability.log2()
        })
        .sum()
}

/// Calculate information gain for a nominal attribute
fn information_gain(dataset: &NominalDataset, attr_name: &str) -> f64 {
    let total_objects = dataset.objects.len() as f64;
    if total_objects == 0.0 {
        return 0.0;
    }

    // Get all class values for total entropy calculation
    let all_class_values = dataset.get_class_values(&(0..dataset.objects.len()).collect::<Vec<_>>());
    let total_entropy = calculate_entropy(&all_class_values);

    // Group by attribute value and calculate weighted entropy
    let groups = dataset.group_by_attribute_value(attr_name);
    let mut weighted_entropy = 0.0;

    for (_, object_indices) in &groups {
        let group_size = object_indices.len() as f64;
        let weight = group_size / total_objects;

        // Get class values for this group
        let group_class_values = dataset.get_class_values(object_indices);
        let group_entropy = calculate_entropy(&group_class_values);
        weighted_entropy += weight * group_entropy;
    }

    total_entropy - weighted_entropy
}

pub(crate) fn attribute_information_gains(
    dataset: &NominalDataset,
) -> Vec<AttributeInformationGain> {
    let mut gains = Vec::new();

    for attr_name in &dataset.attributes {
        if attr_name == &dataset.class_attribute {
            continue;
        }

        gains.push(AttributeInformationGain {
            attribute: attr_name.clone(),
            gain: information_gain(dataset, attr_name),
        });
    }

    gains.sort_by(|left, right| {
        right
            .gain
            .total_cmp(&left.gain)
            .then_with(|| left.attribute.cmp(&right.attribute))
    });

    gains
}

/// Find the most pertinent attribute using information gain
/// Returns all attributes with maximum gain (handles ties)
pub(crate) fn find_most_pertinent_attributes(dataset: &NominalDataset) -> Vec<String> {
    let attr_gains = attribute_information_gains(dataset);

    // Find maximum gain
    let max_gain = attr_gains
        .iter()
        .map(|gain| gain.gain)
        .fold(f64::MIN, f64::max);

    // Return all attributes with maximum gain
    attr_gains
        .into_iter()
        .filter(|gain| gain.gain == max_gain)
        .map(|gain| gain.attribute)
        .collect()
}

/// Find the most frequent values for an attribute
/// Returns all values with maximum frequency (handles ties)
pub(crate) fn find_most_frequent_values(dataset: &NominalDataset, attr_name: &str) -> Vec<String> {
    let groups = dataset.group_by_attribute_value(attr_name);

    if groups.is_empty() {
        return Vec::new();
    }

    // Find maximum frequency
    let max_freq = groups
        .values()
        .map(|indices| indices.len())
        .max()
        .unwrap_or(0);

    // Return all values with maximum frequency
    groups
        .into_iter()
        .filter(|(_, indices)| indices.len() == max_freq)
        .map(|(value, _)| value)
        .collect()
}
