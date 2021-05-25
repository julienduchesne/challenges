#!/usr/bin/env bash

# Title: Day 2: Inventory Management System

set -euo pipefail

. ./utils.sh

input=${1-$(cat -)}
input="$(trim "$input" | tr " " "\n")"

readarray -t ids <<< "$input"

declare -A seen

two_count=0
three_count=0
start_id=1
p2=""
for id in "${ids[@]}"; do
    seen=()

    # Count chars
    add_two=0
    add_three=0
    while read -r -n 1 char; do
        if [ -z "$char" ]; then continue; fi
        seen[$char]="$((${seen[$char]:-0}+1))"
    done <<< "$id"

    # Check if any char is there exactly two or three times
    for char in "${!seen[@]}"; do
        if [ "${seen[$char]}" -eq 2 ]; then
            add_two=1
        fi
        if [ "${seen[$char]}" -eq 3 ]; then
            add_three=1
        fi
    done
    if [ "$add_two" -eq 1 ]; then two_count=$((two_count+1)); fi
    if [ "$add_three" -eq 1 ]; then three_count=$((three_count+1)); fi

    # Find if any other ID has only one diff with this ID. For part 2
    for i in $(seq $start_id $((${#ids[@]}-1))); do
        other="${ids[$i]}"
        j=0
        diffs=0
        while read -r -n 1 char; do
            if [ "${other:$j:1}" != "$char" ]; then
                diffs=$((diffs+1))
            fi
            if [ "$diffs" -ge 2 ]; then break; fi
            j=$((j+1))
        done <<< "$id"

        if [ "$diffs" -eq 1 ]; then
            for k in $(seq 0 ${#id}); do
                if [ "${id:$k:1}" == "${other:$k:1}" ]; then
                    p2="${p2}${id:$k:1}"
                fi
            done
            break
        fi
    done
    start_id=$((start_id+1))
done

p1=$((two_count*three_count))
echo -e "Part 1: $p1\nPart 2: $p2"