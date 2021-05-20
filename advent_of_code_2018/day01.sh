#!/usr/bin/env bash

# Title: Day 1: Chronal Calibration

. ./utils.sh

input=${1-$(cat -)}
input="$(trim "$input")"

declare -A seen
seen[0]=1

total=0
p1=-1
p2=-1
while [ $p2 -eq -1 ]; do
    for num in $(echo "$input" | tr "," "\n"); do
        num=$((0 + $num)) # Parse as num
        total=$(($total + $num))

        if [ $p2 -eq -1 ]; then
            if [ ${seen[$total]+_} ]; then p2=$total; else seen[$total]=1; fi
        fi
        
    done
    if [ $p1 -eq -1 ]; then
        p1=$total
    fi
done

echo "Part 1: $p1\nPart 2: $p2"