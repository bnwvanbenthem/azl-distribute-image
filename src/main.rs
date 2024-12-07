use clap::{Arg, Command};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct ImageUploadRequest {
    location: String,
    #[serde(rename = "extendedLocation")]
    extended_location: ExtendedLocation,
    properties: Properties,
}

#[derive(Serialize)]
struct ExtendedLocation {
    r#type: String,
    name: String,
}

#[derive(Serialize)]
struct Properties {
    #[serde(rename = "imagePath")]
    image_path: String,
    #[serde(rename = "containerId")]
    container_id: String,
    #[serde(rename = "osType")]
    os_type: String,
}

#[derive(Debug, Serialize)]
struct AsyncOperation {
    url: String,
}

#[derive(Debug, Deserialize)]
struct GalleryImage {
    name: String, // Only capture the "name" field
}

#[derive(Debug, Deserialize)]
struct GalleryResponse {
    value: Vec<GalleryImage>, // The "value" array contains the gallery images
}

#[derive(Debug)]
struct Config {
    token: String,
    image_path: String,
    container_id: String,
    location: String,
    extended_location_name: String,
    subscription: String,
    resource_group: String,
    image_name: String,
    os_type: String,
    overwrite: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments using `clap`
    let matches = Command::new("Azure Image Uploader")
        .about("Uploads an image to Azure Stack HCI gallery.")
        .arg(
            Arg::new("token")
                .long("token")
                .required(true)
                .help("OAuth2 Bearer token for Azure API authorization"),
        )
        .arg(
            Arg::new("image-path")
                .long("image-path")
                .required(true)
                .help("The path to the image file to upload"),
        )
        .arg(
            Arg::new("container-id")
                .long("container-id")
                .required(true)
                .help("The container ID for the storage container"),
        )
        .arg(
            Arg::new("location")
                .long("location")
                .required(true)
                .help("Azure region location for the gallery image"),
        )
        .arg(
            Arg::new("extended-location-name")
                .long("extended-location-name")
                .required(true)
                .help("Name of the extended location"),
        )
        .arg(
            Arg::new("subscription")
                .long("subscription")
                .required(true)
                .help("Name of the subscription"),
        )
        .arg(
            Arg::new("resource-group")
                .long("resource-group")
                .required(true)
                .help("Name of the resource_group"),
        )
        .arg(
            Arg::new("image-name")
                .long("image-name")
                .required(true)
                .help("Name of the image"),
        )
        .arg(
            Arg::new("os-type")
                .long("os-type")
                .required(true)
                .help("Linux or Windows"),
        )
        .arg(
            Arg::new("overwrite")
                .long("overwrite")
                .help("Overwrite existing gallery images")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let config = Config {
        token: matches.get_one::<String>("token").unwrap().to_string(),
        image_path: matches.get_one::<String>("image-path").unwrap().to_string(),
        container_id: matches
            .get_one::<String>("container-id")
            .unwrap()
            .to_string(),
        location: matches.get_one::<String>("location").unwrap().to_string(),
        extended_location_name: matches
            .get_one::<String>("extended-location-name")
            .unwrap()
            .to_string(),
        subscription: matches
            .get_one::<String>("subscription")
            .unwrap()
            .to_string(),
        resource_group: matches
            .get_one::<String>("resource-group")
            .unwrap()
            .to_string(),
        image_name: matches.get_one::<String>("image-name").unwrap().to_string(),
        os_type: matches.get_one::<String>("os-type").unwrap().to_string(),
        overwrite: *matches.get_one::<bool>("overwrite").unwrap(),
    };

    // Create a reqwest client
    let client = Client::new();

    // list images and check if the image exists
    let images = list_images(client.clone(), &config).await?;
    let image_exists = images.iter().any(|image| *image == config.image_name);

    // upload images if it doesn`t exist in the gallery
    // or the overwrite parameter is given
    if !image_exists || config.overwrite == true {
        upload_image(client.clone(), &config).await?;
    } else {
        println!("Gallery Image exists, no overwrite requested");
    }

    Ok(())
}

async fn list_images(client: Client, config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    // check existing images
    let url = format!(
    "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AzureStackHCI/galleryImages?api-version=2024-01-01",
    config.subscription,
    config.resource_group,
);

    let mut images: Vec<String> = Vec::new();

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
            images.push(image.name);
        }
    } else {
        // If the request failed, return the status and error
        let error_text = response.text().await?;
        return Err(format!("Request failed {}", error_text).into());
    }

    Ok(images)
}

async fn upload_image(client: Client, config: &Config) -> Result<(), Box<dyn Error>> {
    // Create the request body
    let request_body = ImageUploadRequest {
        location: config.location.clone(),
        extended_location: ExtendedLocation {
            r#type: "CustomLocation".to_string(),
            name: config.extended_location_name.clone(),
        },
        properties: Properties {
            image_path: config.image_path.clone(),
            container_id: config.container_id.clone(),
            os_type: config.os_type.clone(),
        },
    };

    // Build the URL for the Azure REST API endpoint
    let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AzureStackHCI/galleryImages/{}?api-version=2024-01-01",
        config.subscription,
        config.resource_group,
        config.image_name
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
        println!("Request for Image upload is successful.");
    } else {
        let error_text = response.text().await?;
        return Err(format!("Failed to upload image: {}", error_text).into());
    }

    Ok(())
}
