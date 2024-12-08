use config::Config;
use reqwest::Client;
use std::error::Error;
use storage::StorageLocation;

mod config;
mod gallery;
mod helper;
mod storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::build();

    // Create a reqwest client
    let client = Client::new();

    // list storage containers
    let containers = storage::list_storage_containers(client.clone(), &config).await?;
    let storage_locations = storage::get_unique_storage_locations(containers).await?;

    // list images and check if the image exists
    let images = gallery::list_images(client.clone(), &config).await?;

    for location in storage_locations {
        let cluster_name = helper::extract_cluster_name(&location.cluster).unwrap_or_default();
        let image_name = format!("{}--{}", cluster_name, config.image_name.clone());

        let image_exists = helper::value_exists(&image_name, &images)?;

        if image_exists == true && !config.overwrite {
            println!(
                "Image {} already exisists no overwrite instruction given",
                &image_name
            )
        } else if image_exists == true && config.overwrite {
            println!("Uploading image {}", image_name);
            start_image_upload_req(client.clone(), &config, &location).await?;
        } else {
            println!("Uploading image {}", image_name);
            start_image_upload_req(client.clone(), &config, &location).await?;
        }
    }

    Ok(())
}

async fn start_image_upload_req(
    client: Client,
    config: &Config,
    location: &StorageLocation,
) -> Result<(), Box<dyn Error>> {
    let operation = gallery::upload_image(client.clone(), &config, &location).await?;

    // Access a specific header value by name
    if let Some(header_value) = operation.headers().get("azure-asyncoperation") {
        // Optionally, you can convert it to a string
        if let Ok(header_str) = header_value.to_str() {
            let _ = header_str;
        }
    } else {
        eprintln!("Header 'azure-asyncoperation' not found.");
    }

    Ok(())
}
