use std::collections::{HashMap, HashSet};

/// CNC (Classifier Nominal Concept)
/// A classifier that uses Formal Concept Analysis to extract concepts from nominal (multi-valued) data.
/// The algorithm finds the most pertinent attribute and computes its closure (concept).

/// Nominal dataset structure for CNC
#[derive(Debug, Clone)]
pub struct NominalDataset {
    pub objects: Vec<String>,
    pub attributes: Vec<String>,
    pub class_attribute: String,
    pub data: Vec<HashMap<String, String>>, // Each object's attribute values
}

impl std::fmt::Display for NominalDataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Print header - include all attributes with class clearly separated
        write!(f, "{:>10}", "")?;
        for attr in &self.attributes {
            if attr != &self.class_attribute {
                write!(f, "{:>15}", attr)?;
            }
        }
        // Add visual separator and class column together
        write!(f, "       │  {:>8}", self.class_attribute)?;
        writeln!(f)?;

        // Print separator
        write!(f, "{:>10}", "")?;
        for attr in &self.attributes {
            if attr != &self.class_attribute {
                write!(f, "{:>15}", "")?;
            }
        }
        // Add visual separator in the separator line
        write!(f, "       │  {:>8}", "")?;
        writeln!(f)?;

        // Print each row
        for (i, obj) in self.objects.iter().enumerate() {
            write!(f, "{:>10}", obj)?;
            // Print descriptive attributes (exclude class)
            for attr in &self.attributes {
                if attr != &self.class_attribute {
                    if let Some(val) = self.data[i].get(attr) {
                        write!(f, "{:>15}", val)?;
                    } else {
                        write!(f, "{:>15}", "?")?;
                    }
                }
            }
            // Add visual separator and class value together
            write!(f, "       │  ")?;
            if let Some(class_val) = self.data[i].get(&self.class_attribute) {
                write!(f, "{:>8}", class_val)?;
            } else {
                write!(f, "{:>8}", "?")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl NominalDataset {
    /// Create a new nominal dataset
    pub fn new(
        objects: Vec<String>,
        attributes: Vec<String>,
        class_attribute: String,
        data: Vec<HashMap<String, String>>,
    ) -> Self {
        Self {
            objects,
            attributes,
            class_attribute,
            data,
        }
    }

    /// Get all unique values for an attribute
    pub fn get_attribute_values(&self, attr_name: &str) -> Vec<String> {
        let mut values = HashSet::new();
        for obj_data in &self.data {
            if let Some(val) = obj_data.get(attr_name) {
                values.insert(val.clone());
            }
        }
        let mut result: Vec<String> = values.into_iter().collect();
        result.sort();
        result
    }

    /// Group objects by attribute value
    pub fn group_by_attribute_value(&self, attr_name: &str) -> HashMap<String, Vec<usize>> {
        let mut groups = HashMap::new();
        for (obj_idx, obj_data) in self.data.iter().enumerate() {
            if let Some(val) = obj_data.get(attr_name) {
                groups.entry(val.clone()).or_insert_with(Vec::new).push(obj_idx);
            }
        }
        groups
    }

    /// Get class values for a group of objects
    pub fn get_class_values(&self, object_indices: &[usize]) -> Vec<String> {
        object_indices
            .iter()
            .filter_map(|&obj_idx| self.data[obj_idx].get(&self.class_attribute).cloned())
            .collect()
    }

    /// Get the majority class from a class distribution
    /// Returns (majority_class, count, percentage)
    pub fn get_majority_class(class_values: &[String]) -> Option<(String, usize, f64)> {
        if class_values.is_empty() {
            return None;
        }

        let mut class_counts = HashMap::new();

        // Count occurrences of each class
        for class_val in class_values {
            *class_counts.entry(class_val.clone()).or_insert(0) += 1;
        }

        // Find the class with maximum count
        let (majority_class, count) = class_counts.into_iter().max_by_key(|(_, count)| *count)?;

        let percentage = (count as f64 / class_values.len() as f64) * 100.0;

        Some((majority_class, count, percentage))
    }

    /// Display summary statistics about the dataset
    pub fn display_summary(&self) {
        println!("Context:\n{}", &self);

        // Count descriptive attributes (excluding class)
        let desc_attrs: Vec<_> = self
            .attributes
            .iter()
            .filter(|attr| attr != &&self.class_attribute)
            .collect();

        println!("Dataset Summary:");
        println!("- Objects: {}", self.objects.len());
        println!("- Descriptive attributes: {}", desc_attrs.len());
        println!("- Class attribute: {}", self.class_attribute);

        for attr in &desc_attrs {
            let values = self.get_attribute_values(attr);
            println!("- Attribute '{}': {} unique values", attr, values.len());
        }

        // Show class distribution
        let class_values = self.get_class_values(&(0..self.objects.len()).collect::<Vec<_>>());
        let mut class_counts = HashMap::new();
        for class_val in class_values {
            *class_counts.entry(class_val).or_insert(0) += 1;
        }

        println!("- Class distribution:");
        for (class_val, count) in class_counts {
            println!(
                "  {}: {} ({:.1}%)",
                class_val,
                count,
                (count as f64 / self.objects.len() as f64) * 100.0
            );
        }
    }
}

pub(crate) fn filter_dataset_by_classes(
    dataset: &NominalDataset,
    classes: &HashSet<String>,
) -> NominalDataset {
    let filtered_objects: Vec<usize> = dataset
        .data
        .iter()
        .enumerate()
        .filter(|(_, obj_data)| {
            if let Some(class_val) = obj_data.get(&dataset.class_attribute) {
                classes.contains(class_val)
            } else {
                false // Exclude objects with missing class
            }
        })
        .map(|(idx, _)| idx)
        .collect();

    let filtered_objects_names: Vec<String> = filtered_objects
        .iter()
        .map(|&obj_idx| dataset.objects[obj_idx].clone())
        .collect();

    let filtered_data: Vec<HashMap<String, String>> = filtered_objects
        .iter()
        .map(|&obj_idx| dataset.data[obj_idx].clone())
        .collect();

    NominalDataset {
        objects: filtered_objects_names,
        attributes: dataset.attributes.clone(),
        class_attribute: dataset.class_attribute.clone(),
        data: filtered_data,
    }
}
