#!/usr/bin/env bash

# Title: Day 4: Repose Record

set -euo pipefail

source ./lib/string.sh

input="$(printf "%s" "${1-$(cat -)}" | sort)"
input="$(trim "$input")"

declare -A minutes_by_guard
declare -A hours_slept_counter

readarray -t lines <<< "$input"

current_guard=0
start=-1

for line in "${lines[@]}"; do
    IFS=']' read -ra parts <<< "$line"
    datetime=$(trim "${parts[0]#[}")
    event=$(trim "${parts[1]}")

    IFS=' ' read -ra parts <<< "$datetime"
    time=$(trim "${parts[1]}")

    IFS=':' read -ra parts <<< "$time"
    minute=$(trim "${parts[1]#0}")

    if [[ "$event" == *"Guard #"* ]]; then
        IFS=' ' read -ra parts <<< "$event"
        current_guard="${parts[1]#"#"}"
    elif [[ "$event" == *"falls asleep"* ]]; then
        start="${minute}"
    elif [[ "$event" == *"wakes up"* ]]; then
        minutes_by_guard["$current_guard"]=$(("${minutes_by_guard["$current_guard"]:-0}"+(minute-start)))
        for i in $(seq "$start" $((minute-1))); do
            hours_slept_counter["$current_guard",$i]=$(("${hours_slept_counter["$current_guard",$i]:-0}"+1))
        done
    fi
done

max=0
guard_with_max=0
for key in "${!minutes_by_guard[@]}"; do
    if [ "$max" -lt "${minutes_by_guard["$key"]}" ]; then
        max="${minutes_by_guard["$key"]}"
        guard_with_max="$key"
    fi
done


global_max=0
guard_with_global_max=0
minute_with_global_max=0

max=0
minute_with_max=0
for key in "${!hours_slept_counter[@]}"; do
    count="${hours_slept_counter["$key"]}"
    if [ "$global_max" -lt "$count" ]; then
        global_max="$count"
        IFS=',' read -ra parts <<< "$key"
        guard_with_global_max="${parts[0]}"
        minute_with_global_max="${parts[1]}"
    fi

    if [[ "$key" == "$guard_with_max"* ]]; then
        if [ "$max" -lt "$count" ]; then
            max="$count"
            IFS=',' read -ra parts <<< "$key"
            minute_with_max="${parts[1]}"
        fi
    fi
done



p1=$((minute_with_max*guard_with_max))
p2=$((minute_with_global_max*guard_with_global_max))

echo -e "Part 1: $p1\nPart 2: $p2"