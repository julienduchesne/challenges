#!/usr/bin/env bash

# Title: Day 3: No Matter How You Slice It

set -euo pipefail

source ./lib/string.sh

input=${1-$(cat -)}
input="$(trim "$input")"

declare -A matrix

num_rows=0
num_columns=0
p1="0"
p2=""

readarray -t lines <<< "$input"

for n in 0 1; do

    for line in "${lines[@]}"; do
        IFS='@' read -ra parts <<< "$line"
        id=$(trim "${parts[0]}")
        rest=$(trim "${parts[1]}")

        IFS=':' read -ra parts <<< "$rest"
        coords=$(trim "${parts[0]}")
        size=$(trim "${parts[1]}")

        IFS=',' read -ra parts <<< "$coords"
        x=${parts[0]}
        y=${parts[1]}
        
        IFS='x' read -ra parts <<< "$size"
        width=${parts[0]}
        length=${parts[1]}

        i="$x"
        j="$y"
        end_x="$((x+width))"
        end_y="$((y+length))"
        num_rows="$((end_x>num_rows ? end_x : num_rows))"
        num_columns="$((end_y>num_columns ? end_y : num_columns))"
        has_overlap="0"
        while [ "$i" -ne "$end_x" ]; do
            while [ "$j" -ne "$end_y" ]; do
                if [ "$n" -eq 0 ]; then
                    new_val="$(("${matrix[$i,$j]:-0}"+1))"
                    if [ "$new_val" -eq 2 ]; then
                        p1="$(("$p1"+1))"
                    fi
                    matrix[$i,$j]="$new_val"
                else
                    if [ "${matrix[$i,$j]}" -ne 1 ]; then
                        has_overlap="1"
                    fi
                fi
                j="$((j+1))"
            done
            j="$y"
            i="$((i+1))"
        done

        if [ "$n" -eq 1 ] && [ "$has_overlap" -eq 0 ]; then
            p2="$id"
        fi
    done
done

echo -e "Part 1: $p1\nPart 2: $p2"