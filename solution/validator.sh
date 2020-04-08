#!/bin/bash

for ((i = 1; i <= 3; i++)); do
  cargo -q run < input${i}.txt > output${i}-actual.txt

  echo "Expected output:"
  cat output${i}-expected.txt

  echo ""

  echo "Actual output:"
  cat output${i}-actual.txt

  echo ""

  diff output${i}-actual.txt output${i}-expected.txt
  if [[ $? -gt 0 ]]; then
    exit 1
  fi

  rm -rf output${i}-actual.txt
  if [[ $? -gt 0 ]]; then
    exit 1
  fi
done

exit 0
