#!/bin/bash

cat data/day16.txt |
sed 's/Valve //;s/has.* rate=//;s/;.*valves*//' |
tr -d ',' |
awk '
  BEGIN {
    printf "digraph G {\n"
  }
  {
    printf "%s [label=\"%s - %s\"];\n",$1,$1,$2
    for (i=3; i<=NF; i++) {
      printf "%s -> %s;\n",$1,$i
    }
  }
  END {
    printf "}\n"
  }' |
dot -Tsvg > day16.svg
