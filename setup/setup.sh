#!/bin/bash
set -euo pipefail

# YEAR=$(date +%Y)
YEAR=2019
DAY=$(TZ=America/New_York date +%-d)
SESSION=$(cat session)

mkdir ../$YEAR/$DAY
cp solution.rs ../$YEAR/$DAY/solution.rs
cd ../$YEAR/$DAY
echo curl -H "\"Cookie: session=$SESSION\"" https://adventofcode.com/$YEAR/day/$DAY/input
curl -H "Cookie: session=$SESSION" https://adventofcode.com/$YEAR/day/$DAY/input > input
