#!/bin/bash

BASE="02"
COPIES=("03" "04" "05" "06" "07" "08" "09" "10" "11" "12" "13" "14" "15" "16" "17" "18" "19" "20" "21" "22" "23" "24" "25")

for i in "${COPIES[@]}"
do
    cp -n "src/day$BASE.rs" "src/day$i.rs"
done