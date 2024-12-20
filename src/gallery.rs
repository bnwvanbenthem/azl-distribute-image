use crate::helper;
use crate::storage::StorageLocation;
use crate::Config;

use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize)]
struct ImageUploadRequest {
    location: String,
    #[serde(rename = "extendedLocation")]
    extended_location: ExtendedLocation,
    properties: Properties,
}

#[derive(Debug, Serialize)]
struct Properties {
    #[serde(rename = "imagePath")]
    image_path: String,
    #[serde(rename = "containerId")]
    container_id: String,
    #[serde(rename = "osType")]
    os_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtendedLocation {
    pub r#type: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GalleryImage {
    pub name: String,
    #[serde(rename = "extendedLocation")]
    pub extended_location: ExtendedLocation,
}

#[derive(Debug, Deserialize)]
struct GalleryResponse {
    value: Vec<GalleryImage>, // The "value" array contains the gallery images
}

pub async fn list_images(
    client: Client,
    config: &Config,
) -> Result<Vec<GalleryImage>, Box<dyn Error>> {
    // Build the URL for the Azure REST API endpoint
    let url = format!(
    "https://management.azure.com/subscriptions/{}/providers/Microsoft.AzureStackHCI/galleryImages?api-version={}",
    config.subscription,
    config.api_version,
    );

    let mut images: Vec<GalleryImage> = Vec::new();

    // Send the PUT request with the Authorization token
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .send()
        .await?;

    // Ensure the request was successful
    if response.status().is_success() {
        // Deserialize the response body into the GalleryResponse struct
        let body: GalleryResponse = response.json().await?;

        // Print the names of all gallery images
        for image in body.value {
            images.push(image);
        }
    } else {
        // If the request failed, return the status and error
        let error_text = response.text().await?;
        return Err(format!("Request failed {}", error_text).into());
    }

    Ok(images)
}

pub async fn upload_image(
    client: Client,
    config: &Config,
    storage_location: &StorageLocation,
) -> Result<Response, Box<dyn Error>> {
    // Create the request body
    let request_body = ImageUploadRequest {
        location: config.location.clone(),
        extended_location: ExtendedLocation {
            r#type: "CustomLocation".to_string(),
            name: storage_location.cluster.clone(),
        },
        properties: Properties {
            image_path: config.image_path.clone(),
            container_id: storage_location.container_id.clone(),
            os_type: config.os_type.clone(),
        },
    };

    let resource_group =
        helper::extract_resource_group(&storage_location.cluster).unwrap_or_default();
    let cluster_name = helper::extract_cluster_name(&storage_location.cluster).unwrap_or_default();

    let image_name = format!("{}", config.image_name);
    // Build the URL for the Azure REST API endpoint
    let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AzureStackHCI/galleryImages/{}?api-version={}",
        config.subscription,
        resource_group,
        image_name,
        config.api_version,
        );

    // Send the PUT request with the Authorization token
    let response = client
        .put(&url)
        .header("Authorization", format!("Bearer {}", config.token))
        .json(&request_body)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!(
            "Request for Image upload is successful on cluster \"{}\".",
            cluster_name
        );
    } else {
        let error_text = response.text().await?;
        return Err(format!(
            "Failed to upload image on cluster \"{}\" - \"{}\"",
            cluster_name, error_text
        )
        .into());
    }

    Ok(response)
}
