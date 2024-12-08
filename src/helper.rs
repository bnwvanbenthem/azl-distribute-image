use std::error::Error;

pub fn extract_resource_group(resource_id: &str) -> Option<String> {
    // Split the resource ID into parts by '/'
    let parts: Vec<&str> = resource_id.split('/').collect();

    // Find the index of "resourcegroups" in the parts
    if let Some(index) = parts
        .iter()
        .position(|&part| part.eq_ignore_ascii_case("resourcegroups"))
    {
        // The resource group name follows "resourcegroups"
        if index + 1 < parts.len() {
            return Some(parts[index + 1].to_string());
        }
    }
    // Return None if no resource group was found
    None
}

pub fn extract_cluster_name(resource_id: &str) -> Option<String> {
    // Split the resource ID into parts by '/'
    let parts: Vec<&str> = resource_id.split('/').collect();

    // Find the index of "customlocations" in the parts
    if let Some(index) = parts
        .iter()
        .position(|&part| part.eq_ignore_ascii_case("customlocations"))
    {
        // The cluster name follows "customlocations"
        if index + 1 < parts.len() {
            return Some(parts[index + 1].to_string());
        }
    }
    // Return None if no cluster name was found
    None
}

pub fn value_exists(value: &String, list_of_values: &Vec<String>) -> Result<bool, Box<dyn Error>>{
    let mut value_exists = false;

    for item in list_of_values {
        if *value == *item {
            value_exists = true;
        }
    }

    Ok(value_exists)
}