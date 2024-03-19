#! /bin/bash

INTEGRATION_ENDPOINT="http://localhost:9084"

DRY_RUN=true

while getopts d:q:n:f:o: flag
do
    case "${flag}" in
        d) DRY_RUN=${OPTARG};;
        q) JQ_QUERY=${OPTARG};;
        n) MIGRATION_NAME=${OPTARG};;
        f) JQ_FILTER=${OPTARG};;
        o) JQ_OUTPUT=${OPTARG};;
        *) echo "Unrecognised argument $OPTARG"
    esac
done

printf "Dry run: $DRY_RUN\n\n"

OUT_DIR="./out/$MIGRATION_NAME"
ORIGINAL_DIR="$OUT_DIR/original"
FORMATTED_DIR="$OUT_DIR/original_formatted"
MODIFIED_DIR="$OUT_DIR/modified"

rm -rf "$MIGRATION_NAME"
mkdir -p "$ORIGINAL_DIR"
mkdir -p "$FORMATTED_DIR"
mkdir -p "$MODIFIED_DIR"

printf "Filtering recipes with expression '$JQ_FILTER'\n\n"

jq -c "$JQ_FILTER" recipes.ndjson \
    | uniq \
    > "$OUT_DIR/selected-recipes.json"

split -l 1 "./$OUT_DIR/selected-recipes.json" "$ORIGINAL_DIR/recipe_"

printf "Found $(wc -l < "$OUT_DIR"/selected-recipes.json) recipes - transforming them with expression '$JQ_QUERY' \n\n"

for ORIGINAL_FILE in "$ORIGINAL_DIR"/*
do
    COMPOSER_ID=$(jq -r .composerId "$ORIGINAL_FILE")
    FORMATTED_FILE="$FORMATTED_DIR/$COMPOSER_ID.json"
    jq . "$ORIGINAL_FILE" > "$FORMATTED_FILE"

    OUTFILE=$MODIFIED_DIR/$COMPOSER_ID.json

    jq "$JQ_QUERY" "$FORMATTED_FILE" > "$OUTFILE"
done

DIFF_FILE_PATH="./$OUT_DIR/diff.txt"
printf "Writing diff to $DIFF_FILE_PATH\n\n"

for FILE in "$FORMATTED_DIR"/*
do
    diff "$FILE" "$MODIFIED_DIR/$(basename "$FILE")" -U0 >> "$DIFF_FILE_PATH"
done

if [ "$DRY_RUN" = true ]
then
  echo "Dry run - exiting."
  exit 0
fi

printf "Not a dry run - updating recipes at ${INTEGRATION_ENDPOINT}\n\n"

for FILE in "$MODIFIED_DIR"/*
do
    jq -c "$JQ_OUTPUT" "$FILE" | \
      while read -r LINE; do \
        curl \
          --location "$INTEGRATION_ENDPOINT/recipes/import/update-recipe-element/$(basename $FILE .json)" \
          --header 'Content-Type: application/json' \
          --data "$LINE" \
          ;
      done
done

printf "\n\nDone."
