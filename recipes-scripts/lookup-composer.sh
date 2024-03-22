#!/bin/bash

for x in `cat checksum.lst`; do
    COMPOSER_ID=$(curl https://recipes.guardianapis.com/content/$x 2>/dev/null| jq -r .composerId)
    echo "https://composer.gutools.co.uk/content/$COMPOSER_ID"
done