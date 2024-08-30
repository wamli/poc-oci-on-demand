#!/bin/bash

# This script is supposed to 
# (1) tag a buildah image,
# (2) push it to a local registry,
# (3) save the image as a file to disk

EXEC_PATH=`dirname "$0"`
EXEC_PATH=`( cd "$EXEC_PATH" && pwd )`
echo -e "\tThis script executes from $EXEC_PATH"

if [ "$#" -lt 2 ]; then
    echo -e "Error: No argument provided. Please provide" 
    echo -e "\t(1) the name of the image to tag"
    echo -e "\t(2) the new tag"
    exit 1
fi

IMAGE_TO_TAG="$1"
NEW_TAG="$2"

echo -e "\ttagging .."
buildah tag localhost/$IMAGE_TO_TAG localhost:5000/$NEW_TAG

echo -e "\npushing .."
buildah push --tls-verify=false localhost:5000/$NEW_TAG

echo -e "\nverifying .."
curl -X GET http://localhost:5000/v2/_catalog | jq

echo -e "\npull from localhost"
docker pull localhost:5000/$NEW_TAG

echo -e "\nsaving to disk"
docker save -o $IMAGE_TO_TAG.tar localhost:5000/$NEW_TAG