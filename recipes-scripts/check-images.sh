#! /bin/bash

while getopts f: flag
do
    case "${flag}" in
        f) FILE=${OPTARG};;
        h) printf "
Ping each image for the input (an ndjson file containing a single recipe per line), giving a status code.

Usage: ./check-images.sh -f FILE
"
        exit 0;;
        *) echo "Unrecognised argument $OPTARG"
    esac
done

echo "Reading $FILE."

function check_image() {
  curl -s -o /dev/null -w "%{http_code}\n" "$1"
}

IMAGE_VARS=$(jq -r -c "{\"featuredImage\": .featuredImage.url, \"previewImage\": .previewImage.url, \"id\": .id}" "$FILE")

while read -r IMAGE_VAR; do
  FEATURED_IMAGE=$(jq -r .featuredImage <<< "$IMAGE_VAR")
  PREVIEW_IMAGE=$(jq -r .previewImage <<< "$IMAGE_VAR")
  ID=$(jq -r .id <<< "$IMAGE_VAR")

  echo "Recipe $ID"
  echo "Featured image $FEATURED_IMAGE: $(check_image "$FEATURED_IMAGE")"
  echo "Preview image $PREVIEW_IMAGE: $(check_image "$PREVIEW_IMAGE")"
done <<< "$IMAGE_VARS"

