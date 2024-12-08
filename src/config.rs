use clap::{Arg, Command};

#[derive(Debug)]
pub struct Config {
    pub token: String,
    pub image_path: String,
    pub container_id: String,
    pub location: String,
    pub extended_location_name: String,
    pub subscription: String,
    pub resource_group: String,
    pub image_name: String,
    pub os_type: String,
    pub overwrite: bool,
}

impl Config {
    pub fn build() -> Self {
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

        return config;
    }
}
