use super::dataset::NominalDataset;
use super::types::{CncConcept, CncResult};

pub fn display_cnc_chosen_attribute(dataset: &NominalDataset, results: &CncResult) {
    println!("Most pertinent attribute(s): {:?}", results.pertinent_attrs);

    for pertinent_attr in &results.pertinent_attrs {
        let most_frequent_values = dataset.get_attribute_values(pertinent_attr);
        println!(
            "  Most frequent value(s) for '{}': {:?}",
            pertinent_attr, most_frequent_values
        );
    }
}

/// Display CNC results in a standardized format
pub fn display_cnc_results(dataset: &NominalDataset, results: &[CncConcept]) {
    if results.is_empty() {
        println!("No concepts found");
        return;
    }

    // There is a theroem ("connexion de Galois") saying that A''' = A'.
    println!("\nCNC Results ({} concept(s) found):", results.len());

    for (i, (pertinent_attr, attr_value, extent, intent)) in results.iter().enumerate() {
        println!("\nConcept {}:", i + 1);
        println!(
            "  Pertinent attribute: '{}' with value '{}'",
            pertinent_attr, attr_value
        );

        // Show extent (objects)
        let extent_objects: Vec<String> = extent
            .iter()
            .map(|&obj_idx| dataset.objects[obj_idx].clone())
            .collect();
        println!("  Extent of the pertinent attribute(s): {:?}", extent_objects);
        println!(
            "  Extent size: {}/{} objects ({:.1}%)",
            extent.len(),
            dataset.objects.len(),
            (extent.len() as f64 / dataset.objects.len() as f64) * 100.0
        );

        // Show intent (common attributes)
        println!("  Intent (common attributes) of the found extent : {:?}", intent);
        let desc_attrs_count = dataset
            .attributes
            .iter()
            .filter(|a| *a != &dataset.class_attribute)
            .count();
        println!(
            "  Intent size: {}/{} attributes ({:.1}%)",
            intent.len(),
            desc_attrs_count,
            (intent.len() as f64 / desc_attrs_count as f64) * 100.0
        );

        // Show class distribution in extent
        let class_values = dataset.get_class_values(extent);
        println!("  Class distribution in extent: {:?}", class_values);

        // Show majority class
        if let Some((majority_class, count, percentage)) =
            NominalDataset::get_majority_class(&class_values)
        {
            println!(
                "  Majority class: '{}' ({}/{}, {:.1}%)",
                majority_class,
                count,
                extent.len(),
                percentage
            );
        }
    }
}
