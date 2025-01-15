cargo fmt
cargo build --release

export IMAGE_NAME='dummy-image-bvb'
export IMAGE_PATH='https://azlimgdistribute.blob.core.windows.net/images/dummy-image-bvb.vhd'
export SUBSCRIPTION='d38b5566-1cb7-411a-95ac-e94507237470'
export LOCATION='westeurope'
export OS_TYPE='Linux'
export API_VERSION='2024-01-01'

export TOKEN=$(az account get-access-token --query "accessToken" --output tsv)

./target/release/azl-distribute-image --token $TOKEN \
    --image-path $IMAGE_PATH \
    --image-name $IMAGE_NAME \
    --location $LOCATION \
    --subscription $SUBSCRIPTION \
    --os-type $OS_TYPE \
    --api-version $API_VERSION \
    #--overwrite
