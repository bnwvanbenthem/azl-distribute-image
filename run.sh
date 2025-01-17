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

# CONSTRUCT IMAGE URL
#export IMAGE_NAME='rhel9-basic-v1'
export IMAGE_NAME='rhel9-postgres-preinst-v1'
export IMAGE_PATH=$(echo https://azlimgdistribute.blob.core.windows.net/images/$IMAGE_NAME.vhdx?$SAS_TOKEN)

# GENERATE TOKEN FOR GALLERY ACCESS
export TOKEN=$(az account get-access-token --query "accessToken" --output tsv)

export SUBSCRIPTION='d38b5566-1cb7-411a-95ac-e94507237470'
export LOCATION='westeurope'
export OS_TYPE='Linux'
export API_VERSION='2024-01-01'


./target/release/azl-distribute-image --token $TOKEN \
    --image-path $IMAGE_PATH \
    --image-name $IMAGE_NAME \
    --location $LOCATION \
    --subscription $SUBSCRIPTION \
    --os-type $OS_TYPE \
    --api-version $API_VERSION \
    #--overwrite
