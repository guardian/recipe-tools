#! /bin/bash

./transform-recipes.sh \
  -n composer-id \
  -f '.' \
  -q '.' \
  -o '{ id, composerId }' \
  -d false
