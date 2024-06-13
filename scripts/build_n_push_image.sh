#!/bin/bash

# This script is supposed to create an OCI container based on buildah.
# It expects one argument which shall be a file to be put into the container.

EXEC_PATH=`dirname "$0"`
EXEC_PATH=`( cd "$EXEC_PATH" && pwd )`
echo -e "\tThis script executes from $EXEC_PATH"

if [ "$#" -lt 1 ]; then
    echo "Error: No argument provided. Please provide a file path"
    exit 1
fi


FILE_FOR_CONTAINER="$1"

if [ -f "$FILE_FOR_CONTAINER" ]; then
    echo "The provided file path is valid: $FILE_FOR_CONTAINER"
else
    echo "Error: The provided file path is not valid or the file does not exist: $FILE_FOR_CONTAINER"
    exit 1
fi

echo -e "\nCreating new image with file $FILE_FOR_CONTAINER .."

### does not work from script, something about processes ..?

# newcontainer=$(buildah from scratch)

# echo " .. created"

# export newcontainer
# buildah unshare
# scratchmnt=$(buildah mount $newcontainer)

# echo " .. available"

# echo -e "\t$scratchmnt"
# echo
# echo "copying file into container .."
# buildah copy $newcontainer $FILE_FOR_CONTAINER /
# buildah commit $newcontainer newimage

# echo "configuring new container .."
# buildah config --created-by "wamli" $newcontainer
# buildah config --author "christoph.brewing@wamli.dev" --label name=mlimage01 $newcontainer

# buildah inspect $newcontainer

# echo -e "\nvoilà, your new container:"

# buildah unmount $newcontainer
# buildah commit $newcontainer wamli-ml-01
# buildah images

echo "pushing to local registry .."

echo -e "\ttagging .."
buildah tag localhost/wamli-ml-01:latest localhost:5000/wamli-ml-01:latest

echo -e "\npushing .."
buildah push --tls-verify=false localhost:5000/wamli-ml-01:latest

echo -e "\nverifying .."
curl -X GET http://localhost:5000/v2/_catalog