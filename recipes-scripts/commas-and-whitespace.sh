#! /bin/bash

./transform-recipes.sh \
  -n commas-and-whitespace \
  -f 'select(.ingredients[].ingredientsList[].name
    | test("(,| )+$")) | .' \
  -q '.ingredients[].ingredientsList[].name
    |= sub("(?<ingredient>.*?)[, ]+$"; "\(.ingredient)")' \
  -o '{ id, ingredients }' \
  -d false
