#!/bin/bash

# This script is supposed to create an OCI container based on buildah.
# It expects one argument which shall be a file to be put into the container.

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