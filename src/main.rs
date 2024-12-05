use clap::{Arg, Command};
use reqwest::Client;
use serde::Serialize;
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
        .get_matches();

    // Extract values from arguments using `get_one`
    let token = matches.get_one::<String>("token").unwrap();
    let image_path = matches.get_one::<String>("image-path").unwrap();
    let container_id = matches.get_one::<String>("container-id").unwrap();
    let location = matches.get_one::<String>("location").unwrap();
    let extended_location_name = matches.get_one::<String>("extended-location-name").unwrap();
    let subscription = matches.get_one::<String>("subscription").unwrap();
    let resource_group = matches.get_one::<String>("resource-group").unwrap();
    let image_name = matches.get_one::<String>("image-name").unwrap();
    let os_type = matches.get_one::<String>("os-type").unwrap();

    // Create the request body
    let request_body = ImageUploadRequest {
        location: location.clone(),
        extended_location: ExtendedLocation {
            r#type: "CustomLocation".to_string(),
            name: extended_location_name.clone(),
        },
        properties: Properties {
            image_path: image_path.clone(),
            container_id: container_id.clone(),
            os_type: os_type.clone(),
        },
    };

    // Build the URL for the Azure REST API endpoint
    let url = format!(
        "https://management.azure.com/subscriptions/{}/resourceGroups/{}/providers/Microsoft.AzureStackHCI/galleryImages/{}?api-version=2024-01-01",
        subscription, 
        resource_group, 
        image_name 
    );

    // Create a reqwest client
    let client = Client::new();

    // Send the PUT request with the Authorization token
    let response = client
        .put(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&request_body)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        println!("Image upload successful.");
    } else {
        let error_text = response.text().await?;
        eprintln!("Failed to upload image: {}", error_text);
    }

    Ok(())
}
