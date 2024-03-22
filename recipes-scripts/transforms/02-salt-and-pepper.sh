#! /bin/bash

./transform-recipes.sh \
  -n salt-and-pepper \
  -f 'select(
      .ingredients[].ingredientsList[]
      | (.name | test("^salt"; "i"))
        and (.suffix != null)
        and (.suffix | test("^and")))' \
  -q '.ingredients[].ingredientsList[]
    |= if (
      (.name | test("^salt"; "i"))
      and (.suffix != null)
      and (.suffix | test("^and"))
    ) then
      .name |= "Salt and (black) pepper"
      | del(.suffix)
    end'
