# azl-distribute-image
Distribute a gallery image to all Azure Local clusters registered in a subscription. A Storage container is dynamically selected on each extended location.

![alt text](files/img.png)
```bash
#!/bin/bash

# BUILD DISPATCHER
cargo fmt
cargo build --release

# GENERATE SAS TOKEN THAT IS VALID FOR 2 HOURS
TOKEN_EXP=$(date -u -d '+2 hours' +"%Y-%m-%dT%H:%M:%SZ")
SAS_TOKEN=$(az storage container generate-sas --account-name azlimgdistribute --name images --permissions r --expiry $TOKEN_EXP --https-only --output tsv)
clear

echo ""
echo "Staring Image Distribution"
echo "--------------------------"

# GENERATE TOKEN FOR GALLERY ACCESS
export TOKEN=$(az account get-access-token --query "accessToken" --output tsv)
# GENERATE PARAMS
export SUBSCRIPTION='d38b5566-1cb7-411a-95ac-e94507237470'
export LOCATION='westeurope'
export OS_TYPE='Linux'
export API_VERSION='2024-01-01'

# Define an array
image_list=("rhel9-basic-v1" "rhel9-postgres-preinst-v1")
# Loop through the array
for image in "${image_list[@]}"; do
  export IMAGE_NAME=$image
  export IMAGE_PATH=$(echo https://azlimgdistribute.blob.core.windows.net/images/$IMAGE_NAME.vhdx?$SAS_TOKEN)

./target/release/azl-distribute-image --token $TOKEN \
    --image-path $IMAGE_PATH \
    --image-name $IMAGE_NAME \
    --location $LOCATION \
    --subscription $SUBSCRIPTION \
    --os-type $OS_TYPE \
    --api-version $API_VERSION \
    #--overwrite
done

```