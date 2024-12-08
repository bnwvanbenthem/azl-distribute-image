use crate::Config;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct StorageContainers {
    pub value: Vec<StorageContainer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageContainer {
    pub id: String,
    pub name: String,
    pub r#type: String,
    pub location: String,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
    pub properties: Properties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedLocation {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "provisioningState")]
    pub provisioning_state: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLocation {
    pub cluster: String,
    pub container_id: String,
}

pub async fn list_storage_containers(
    client: Client,
    config: &Config,
) -> Result<Vec<StorageContainer>, Box<dyn Error>> {
    // Build the URL for the Azure REST API endpoint
    let url = format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.AzureStackHCI/storageContainers?api-version=2024-01-01",
            config.subscription,
        );

    let mut containers: Vec<StorageContainer> = Vec::new();

    // Send the PUT request with the Authorization token
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .send()
        .await?;

    // Ensure the request was successful
    if response.status().is_success() {
        // Deserialize the response body into the GalleryResponse struct
        let body: StorageContainers = response.json().await?;
        for container in body.value {
            containers.push(container);
        }
    } else {
        // If the request failed, return the status and error
        let error_text = response.text().await?;
        return Err(format!("Request failed {}", error_text).into());
    }

    Ok(containers)
}

pub async fn get_unique_storage_locations(
    containers: Vec<StorageContainer>,
) -> Result<Vec<StorageLocation>, Box<dyn Error>> {
    let mut unique: HashMap<String, String> = HashMap::new();

    for container in &containers {
        unique.insert(
            container.extended_location.name.clone(),
            container.id.clone(),
        );
    }

    let mut storage_locations: Vec<StorageLocation> = Vec::new();

    for (cluster, container_id) in unique {
        let storage_location = StorageLocation {
            cluster,
            container_id,
        };

        storage_locations.push(storage_location);
    }

    Ok(storage_locations)
}
