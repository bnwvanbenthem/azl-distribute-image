
export CUSTOM_LOACTION='hci-659-mix-y01'

export IMAGE_NAME='dummy-image-bvb'
export IMAGE_PATH='C:\\bvb\\dummy.vhd'
export SUBSCRIPTION='d38b5566-1cb7-411a-95ac-e94507237470'
export RESOURCE_GROUP='rg-azhci-infraape-01'
export STORAGE_CONTAINER='Volume1'
export LOCATION='westeurope'
export OS_TYPE='Linux'

export TOKEN=$(az account get-access-token --query "accessToken" --output tsv)

cargo run -- --token $TOKEN \
    --image-path $IMAGE_PATH \
    --image-name $IMAGE_NAME \
    --container-id "/subscriptions/$SUBSCRIPTION/resourceGroups/$RESOURCE_GROUP/providers/Microsoft.AzureStackHCI/storageContainers/$IMAGE_NAME" \
    --location $LOCATION \
    --extended-location-name "/subscriptions/$SUBSCRIPTION/resourceGroups/$RESOURCE_GROUP/providers/Microsoft.ExtendedLocation/customLocations/$CUSTOM_LOACTION" \
    --subscription $SUBSCRIPTION \
    --resource-group $RESOURCE_GROUP \
    --os-type $OS_TYPE
