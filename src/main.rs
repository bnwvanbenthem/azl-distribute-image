use config::Config;
use reqwest::Client;
use std::error::Error;

mod config;
mod gallery;
mod storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::build();

    // Create a reqwest client
    let client = Client::new();

    // list storage containers
    let containers = storage::list_storage_containers(client.clone(), &config).await?;
    let storage_locations = storage::get_unique_storage_locations(containers).await?;

    for container in storage_locations.values() {
        println!("{}", container);
    }

    // list images and check if the image exists
    let images = gallery::list_images(client.clone(), &config).await?;
    // check existing images
    let image_exists = images.iter().any(|image| *image == config.image_name);

    // upload images if it doesn`t exist in the gallery
    // or the overwrite parameter is given
    if !image_exists || config.overwrite == true {
        let operation = gallery::upload_image(client.clone(), &config).await?;

        // Access a specific header value by name
        if let Some(header_value) = operation.headers().get("azure-asyncoperation") {
            // Optionally, you can convert it to a string
            if let Ok(header_str) = header_value.to_str() {
                println!("{}", header_str);
            }
        } else {
            eprintln!("Header 'azure-asyncoperation' not found.");
        }
    } else {
        println!("Gallery Image exists, no overwrite requested");
    }

    Ok(())
}
